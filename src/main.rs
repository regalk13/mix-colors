
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::image;

#[derive(Clone, Debug)]
enum BrushType {
    Basic,
    Textured,
    Light,
}

#[derive(Clone, Debug)]
struct BrushPoint {
    position: Point2,
    pressure: f32,
    stamp_rotation: f32,
}

#[derive(Clone, Debug)]
struct Stroke {
    points: Vec<BrushPoint>,
    color: Srgb<u8>,
    width: f32,
    brush_type: BrushType,
}

struct Model {
    strokes: Vec<Stroke>,
    current_stroke: Stroke,
    egui: Egui,
    settings: Settings,
    ui_wants_input: bool,
    texture: wgpu::Texture,
}

struct Settings {
    stroke_width: f32,
    stroke_color: Srgb<u8>,
    clear_canvas: bool,
    brush_type: BrushType,
    brush_pressure: f32,
    brush_spacing: f32,
}

pub struct PaintColor {
    pub name: &'static str,
    pub color: Srgb<u8>,
}




pub fn create_paint_colors() -> Vec<PaintColor> {
    vec![
        PaintColor {
            name: "Cadmium Yellow",
            color: srgb(254, 236, 0),
        },
        PaintColor {
            name: "Hansa Yellow",
            color: srgb(252, 211, 0),
        },
        PaintColor {
            name: "Cadmium Orange",
            color: srgb(255, 105, 0),
        },
        PaintColor {
            name: "Cadmium Red",
            color: srgb(255, 39, 2),
        },
        PaintColor {
            name: "Quinacridone Magenta",
            color: srgb(128, 2, 46),
        },
        PaintColor {
            name: "Cobalt Violet",
            color: srgb(78, 0, 66),
        },
        PaintColor {
            name: "Ultramarine Blue",
            color: srgb(25, 0, 89),
        },
        PaintColor {
            name: "Cobalt Blue",
            color: srgb(0, 33, 133),
        },
        PaintColor {
            name: "Phthalo Blue",
            color: srgb(13, 27, 68),
        },
        PaintColor {
            name: "Phthalo Green",
            color: srgb(0, 60, 50),
        },
        PaintColor {
            name: "Permanent Green",
            color: srgb(7, 109, 22),
        },
        PaintColor {
            name: "Sap Green",
            color: srgb(107, 148, 4),
        },
        PaintColor {
            name: "Burnt Sienna",
            color: srgb(123, 72, 0),
        },
        PaintColor {
            name: "Black",
            color: srgb(0, 0, 0),
        },
    ]
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let logo_path = app.assets_path().unwrap().join("images").join("RC_Brushes.png");
    let image = image::open(logo_path).unwrap();
    let texture = wgpu::Texture::from_image(&window, &image);
    // let texture_view = texture.view().build();
   
    let egui = Egui::from_window(&window);

    Model {
        strokes: Vec::new(),
        current_stroke: Stroke {
            points: Vec::new(),
            color: srgb(0, 0, 0),
            width: 5.0,
            brush_type: BrushType::Basic,
        },
        egui,
        settings: Settings {
            stroke_width: 5.0,
            stroke_color: srgb(0, 0, 0),
            clear_canvas: false,
            brush_type: BrushType::Basic,
            brush_pressure: 1.0,
            brush_spacing: 0.1,
        },
        ui_wants_input: false,
        texture
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    model.ui_wants_input = ctx.wants_pointer_input();
    egui::SidePanel::new(egui::panel::Side::Left, "SidePanel").show(&ctx, |ui| {
        ui.label("Stroke Width:");
        ui.add(egui::Slider::new(&mut settings.stroke_width, 1.0..=20.0));

        ui.label("Stroke Color:");
        let mut color = [
            settings.stroke_color.red as f32 / 255.0,
            settings.stroke_color.green as f32 / 255.0,
            settings.stroke_color.blue as f32 / 255.0,
        ];

        ui.separator();
        ui.label("Brush Settings:");

        ui.horizontal(|ui| {
            if ui.button("Basic").clicked() {
                settings.brush_type = BrushType::Basic;
            }
            if ui.button("Textured").clicked() {
                settings.brush_type = BrushType::Textured;
            }
            if ui.button("Light").clicked() {
                settings.brush_type = BrushType::Light;
            }
        });

        ui.label("Brush Pressure:");
        ui.add(egui::Slider::new(&mut settings.brush_pressure, 0.1..=1.0));
        
        ui.label("Brush Spacing:");
        ui.add(egui::Slider::new(&mut settings.brush_spacing, 0.05..=0.5));
        
        if ui.color_edit_button_rgb(&mut color).changed() {
            settings.stroke_color = srgb(
                (color[0] * 255.0) as u8,
                (color[1] * 255.0) as u8,
                (color[2] * 255.0) as u8,
            );
        }

        let colors = create_paint_colors();
        for paint in colors {
            ui.add(
                egui::Label::new(egui::RichText::new(paint.name).color(egui::Color32::WHITE))
            );
            let button = ui.add(
                egui::Button::new("")
                    .min_size(egui::Vec2::new(300.0, 20.0)),
            );
            if button.clicked() {
                settings.stroke_color = paint.color;
            }
        }

        if ui.button("Clear Canvas").clicked() {
            settings.clear_canvas = true;
        }
    });

    if settings.clear_canvas {
        model.strokes.clear();
        settings.clear_canvas = false;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {    
    let draw = app.draw();
    draw.background().color(WHITE);

    for stroke in &model.strokes {
        draw_stroke(&draw, stroke, &model.texture);
    }

    if !model.current_stroke.points.is_empty() {
        draw_stroke(&draw, &model.current_stroke, &model.texture);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}


fn draw_stroke(draw: &Draw, stroke: &Stroke, texture: &wgpu::Texture) { 
    match stroke.brush_type {
        BrushType::Basic => {
            if stroke.points.len() > 1 {
                draw.polyline()
                    .weight(stroke.width)
                    .color(stroke.color)
                    .points(stroke.points.iter().map(|p| p.position));
            }
        }
        BrushType::Textured => {
            for point in &stroke.points {
                draw.texture(texture)
                    .xy(point.position)
                    .w_h(stroke.width, stroke.width)
                    //.rgba(
                    //    stroke.color.red as f32 / 255.0,
                    //    stroke.color.green as f32 / 255.0,
                    //    stroke.color.blue as f32 / 255.0,
                    //   point.pressure,
                    //)
                    .rotate(0.0);
            }
        }
        BrushType::Light => {
            for point in &stroke.points {
                draw.ellipse()
                    .xy(point.position)
                    .w_h(stroke.width, stroke.width)
                    .color(rgba(
                        stroke.color.red as f32 / 255.0,
                        stroke.color.green as f32 / 255.0,
                        stroke.color.blue as f32 / 255.0,
                        point.pressure * 0.5, // Light brush has lower opacity
                    ));
                    // .blur(stroke.width * 0.5); // Add blur effect for light brush
            }
        }
    }
}


fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    if !model.ui_wants_input {
        model.current_stroke = Stroke {
            points: Vec::new(),
            color: model.settings.stroke_color,
            width: model.settings.stroke_width,
            brush_type: model.settings.brush_type.clone(),
        };
    }
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    if !model.ui_wants_input {
        if !model.current_stroke.points.is_empty() {
            model.strokes.push(model.current_stroke.clone());
        }
        model.current_stroke.points.clear();
    }
}


fn mouse_moved(app: &App, model: &mut Model, pos: Point2) {
    if !model.ui_wants_input && app.mouse.buttons.left().is_down() {
        let last_position = model.current_stroke.points
            .last()
            .map(|point| point.position);
            
        if let Some(last_pos) = last_position {
            let distance = pos.distance(last_pos);
            let spacing = model.settings.stroke_width * model.settings.brush_spacing;
            
            if distance >= spacing {
                let num_points = (distance / spacing).ceil() as usize;
                for i in 1..=num_points {
                    let t = i as f32 / num_points as f32;
                    let new_pos = last_pos.lerp(pos, t);
                    let rotation = random_range(0.0, TAU); // Random rotation for texture variety
                    let pressure = model.settings.brush_pressure * random_range(0.8, 1.0);
                    
                    model.current_stroke.points.push(BrushPoint {
                        position: new_pos,
                        pressure,
                        stamp_rotation: rotation,
                    });
                }
            }
        } else {
            model.current_stroke.points.push(BrushPoint {
                position: pos,
                pressure: model.settings.brush_pressure,
                stamp_rotation: random_range(0.0, TAU),
            });
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
