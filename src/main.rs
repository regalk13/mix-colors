use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use mixbox;
use std::collections::VecDeque;

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
                color: egui::Color32::BLACK,
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
        ui.horizontal(|ui| {
            ui.color_edit_button_srgba(&mut canvas.current_stroke.color);
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

    egui::Window::new("Paint Canvas")
        .default_size(egui::vec2(800.0, 600.0))
        .show(contexts.ctx_mut(), |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());
            let rect = response.rect;

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


            for stroke in &canvas.strokes {
                render_stroke(&painter, stroke, &canvas.strokes);
            }

            render_stroke(&painter, &canvas.current_stroke, &canvas.strokes);
        });
}

fn render_stroke(painter: &egui::Painter, stroke: &Stroke, all_strokes: &VecDeque<Stroke>) {
    if stroke.points.len() < 2 {
        return;
    }

    for window in stroke.points.windows(2) {
        let segment = (window[0], window[1]);

        let mixed_color = mix_color_for_segment(segment, stroke.color, all_strokes);
        println!("mixed color: {:?}", mixed_color);
        painter.add(egui::Shape::line_segment(
            [segment.0, segment.1],
            egui::Stroke::new(stroke.stroke_width, mixed_color),
        ));
    }
}

fn mix_color_for_segment(
    segment: (egui::Pos2, egui::Pos2),
    base_color: egui::Color32,
    strokes: &VecDeque<Stroke>,
) -> egui::Color32 {
    let mut mixed_rgb = [
        base_color.r() as f32 / 255.0,
        base_color.g() as f32 / 255.0,
        base_color.b() as f32 / 255.0,
    ];

    for stroke in strokes {
        if stroke.points.len() < 2 {
            continue;
        }

        for other_segment in stroke.points.windows(2) {
            let other_segment = (other_segment[0], other_segment[1]);
            if segments_overlap(segment, other_segment) {
                let mixed_rgb_u8 = [
                    (mixed_rgb[0] * 255.0) as u8,
                    (mixed_rgb[1] * 255.0) as u8,
                    (mixed_rgb[2] * 255.0) as u8,
                ];

                let other_rgb = [
                    stroke.color.r(),
                    stroke.color.g(),
                    stroke.color.b(),
                ];

                let latent_base = mixbox::rgb_to_latent(&mixed_rgb_u8);
                let latent_other = mixbox::rgb_to_latent(&other_rgb);

                let mut latent_mix = [0.0; mixbox::LATENT_SIZE];
                for i in 0..mixbox::LATENT_SIZE {
                    latent_mix[i] = 0.5 * latent_base[i] + 0.5 * latent_other[i];
                }

                let mixed_rgb_u8_new = mixbox::latent_to_rgb(&latent_mix);

                mixed_rgb = [
                    mixed_rgb_u8_new[0] as f32 / 255.0,
                    mixed_rgb_u8_new[1] as f32 / 255.0,
                    mixed_rgb_u8_new[2] as f32 / 255.0,
                ];
            }
        }
    }

    egui::Color32::from_rgb(
        (mixed_rgb[0] * 255.0) as u8,
        (mixed_rgb[1] * 255.0) as u8,
        (mixed_rgb[2] * 255.0) as u8,
    )
}


fn segments_overlap(seg1: (egui::Pos2, egui::Pos2), seg2: (egui::Pos2, egui::Pos2)) -> bool {
    let distance = |a: egui::Pos2, b: egui::Pos2| ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();

    let threshold = 5.0;
    distance(seg1.0, seg2.0) < threshold
        || distance(seg1.0, seg2.1) < threshold
        || distance(seg1.1, seg2.0) < threshold
        || distance(seg1.1, seg2.1) < threshold
}
