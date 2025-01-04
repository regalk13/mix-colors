use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::collections::VecDeque;

#[derive(Resource)]
struct PaintCanvas {
    strokes: VecDeque<Vec<egui::Pos2>>,
    current_stroke: Vec<egui::Pos2>,
    color: egui::Color32,
    stroke_width: f32,
}

impl Default for PaintCanvas {
    fn default() -> Self {
        Self {
            strokes: VecDeque::new(),
            current_stroke: Vec::new(),
            color: egui::Color32::BLACK,
            stroke_width: 2.0,
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
    egui::SidePanel::new(egui::panel::Side::Left, "SidePanel")
    .show(contexts.ctx_mut(), |ui| {

        ui.horizontal(|ui| {
            ui.color_edit_button_srgba(&mut canvas.color);

            ui.add(
                egui::Slider::new(&mut canvas.stroke_width, 1.0..=20.0).text("Stroke width"),
            );

            if ui.button("Clear").clicked() {
                canvas.strokes.clear();
                canvas.current_stroke.clear();
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
            println!("Strokes: {:?}", canvas.strokes);
            if response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    canvas.current_stroke.push(pointer_pos);
                }
            } else if response.drag_stopped() {
                if !canvas.current_stroke.is_empty() {
                    let finished_stroke = std::mem::take(&mut canvas.current_stroke);
                    canvas.strokes.push_back(finished_stroke);
                    canvas.current_stroke.clear();
                }
            }

            for stroke in &canvas.strokes {
                if stroke.len() >= 2 {
                    painter.add(egui::Shape::line(
                        stroke.clone(),
                        egui::Stroke::new(canvas.stroke_width, canvas.color),
                    ));
                }
            }

            if canvas.current_stroke.len() >= 2 {
                painter.add(egui::Shape::line(
                    canvas.current_stroke.clone(),
                    egui::Stroke::new(canvas.stroke_width, canvas.color),
                ));
            }
        });
}
