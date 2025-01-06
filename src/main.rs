use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use mixbox;
use std::collections::VecDeque;

mod pigment;

use pigment::*;

#[derive(Resource)]
struct PaintCanvas {
    strokes: VecDeque<Stroke>,
    current_stroke: Stroke,
}

#[derive(Clone, Default)]
struct Stroke {
    points: Vec<egui::Pos2>,
    color: egui::Color32,
    stroke_width: f32,
}

impl Default for PaintCanvas {
    fn default() -> Self {
        Self {
            strokes: VecDeque::new(),
            current_stroke: Stroke {
                points: Vec::new(),
                color: BLACK.color,
                stroke_width: 10.0,
            },
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<PaintCanvas>()
        .add_systems(Update, paint_system)
        .run();
}

fn paint_system(mut contexts: EguiContexts, mut canvas: ResMut<PaintCanvas>) {
    egui::SidePanel::new(egui::panel::Side::Left, "SidePanel").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            // ui.color_edit_button_srgba(&mut canvas.current_stroke.color);

            // Aviable Pigment Colors according to mixcolor

            // Aviable Pigment Colors according to mixcolor
            let colors = [
                CADMIUM_YELLOW,
                HANSA_YELLOW,
                CADMIUM_ORANGE,
                CADMIUM_RED,
                QUINACRIDONE_MAGENTA,
                COBALT_VIOLET,
                ULTRAMARINE_BLUE,
                COBALT_BLUE,
                PHTHALO_BLUE,
                PHTHALO_GREEN,
                PERMANENT_GREEN,
                SAP_GREEN,
                BURNT_SIENNA,
                BLACK
            ];
            for paint in colors {
                ui.add(
                    egui::Label::new(egui::RichText::new(paint.name).color(egui::Color32::WHITE))
                );
                let button = ui.add(
                    egui::Button::new("")
                        .fill(paint.color)
                        .min_size(egui::Vec2::new(300.0, 20.0)),
                );
                if button.clicked() {
                    canvas.current_stroke.color = paint.color;
                }
            }

            ui.add(
                egui::Slider::new(&mut canvas.current_stroke.stroke_width, 1.0..=20.0)
                    .text("Stroke width"),
            );

            if ui.button("Clear").clicked() {
                canvas.strokes.clear();
                canvas.current_stroke.points.clear();
            }

            if ui.button("Undo").clicked() && !canvas.strokes.is_empty() {
                canvas.strokes.pop_back();
            }
        });
    });
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
        let rect = response.rect;

        // Background
        painter.rect_filled(rect, 0.0, egui::Color32::WHITE);

        if response.dragged() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                canvas.current_stroke.points.push(pointer_pos);
            }
        } else if response.drag_stopped() {
            if !canvas.current_stroke.points.is_empty() {
                let finished_stroke = std::mem::take(&mut canvas.current_stroke);
                canvas.strokes.push_back(finished_stroke);
                canvas.current_stroke.points.clear();
            }
        }

        // Render strokes
        for stroke in &canvas.strokes {
            render_stroke(&painter, stroke);
        }

        render_stroke(&painter, &canvas.current_stroke);
    });
}

fn render_stroke(painter: &egui::Painter, stroke: &Stroke) {
    if stroke.points.len() < 2 {
        return;
    }

    // Interpolate points for smoothness
    let interpolated_points = interpolate_points(&stroke.points, 2.0);

    for window in interpolated_points.windows(2) {
        let (start, end) = (window[0], window[1]);

        // Brush effect: Apply gradient and variable width
        let stroke_width = stroke.stroke_width;
        let gradient_color = stroke.color.linear_multiply(0.8); // Slightly transparent
        painter.add(egui::Shape::line_segment(
            [start, end],
            egui::Stroke::new(stroke_width, gradient_color),
        ));
    }
}

fn interpolate_points(points: &[egui::Pos2], step: f32) -> Vec<egui::Pos2> {
    let mut interpolated = vec![points[0]];
    for i in 0..points.len() - 1 {
        let start = points[i];
        let end = points[i + 1];
        let distance = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2)).sqrt();
        let steps = (distance / step).ceil() as usize;

        for t in 1..=steps {
            let factor = t as f32 / steps as f32;
            interpolated.push(egui::Pos2::lerp(&start, end, factor));
        }
    }
    interpolated
}

fn mix_color_for_segment(
    segment: (egui::Pos2, egui::Pos2),
    base_color: egui::Color32,
    strokes: &VecDeque<Stroke>,
) -> egui::Color32 {
    // Start with the base color's RGBA values normalized to [0, 1]
    let mut blended_color = [
        base_color.r() as f32 / 255.0,
        base_color.g() as f32 / 255.0,
        base_color.b() as f32 / 255.0,
        base_color.a() as f32 / 255.0,
    ];
    let mut total_opacity = blended_color[3]; // Start with the base color's opacity

    for stroke in strokes {
        if stroke.points.len() < 2 {
            continue;
        }

        for other_segment in stroke.points.windows(2) {
            let other_segment = (other_segment[0], other_segment[1]);
            if segments_overlap(segment, other_segment) {
                let other_color = stroke.color;
                let other_opacity = other_color.a() as f32 / 255.0;

                // Blend based on opacity
                for i in 0..3 {
                    blended_color[i] = (blended_color[i] * total_opacity
                        + (other_color[i] as f32 / 255.0) * other_opacity)
                        / (total_opacity + other_opacity);
                }
                total_opacity += other_opacity; // Accumulate opacity
            }
        }
    }

    // Convert back to egui::Color32
    egui::Color32::from_rgba_unmultiplied(
        (blended_color[0] * 255.0) as u8,
        (blended_color[1] * 255.0) as u8,
        (blended_color[2] * 255.0) as u8,
        (total_opacity * 255.0).min(255.0) as u8,
    )
}


fn segments_overlap(seg1: (egui::Pos2, egui::Pos2), seg2: (egui::Pos2, egui::Pos2)) -> bool {
    let distance =
        |a: egui::Pos2, b: egui::Pos2| ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();

    let threshold = 5.0;
    distance(seg1.0, seg2.0) < threshold
        || distance(seg1.0, seg2.1) < threshold
        || distance(seg1.1, seg2.0) < threshold
        || distance(seg1.1, seg2.1) < threshold
}
