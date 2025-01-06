use bevy_egui::egui;

pub struct PaintColor {
    pub name: &'static str,
    pub color: egui::Color32,
}

// Define your constant colors
pub const CADMIUM_YELLOW: PaintColor = PaintColor {
    name: "Cadmium Yellow",
    color: egui::Color32::from_rgb(254, 236, 0),
};

pub const HANSA_YELLOW: PaintColor = PaintColor {
    name: "Hansa Yellow",
    color: egui::Color32::from_rgb(252, 211, 0),
};

pub const CADMIUM_ORANGE: PaintColor = PaintColor {
    name: "Cadmium Orange",
    color: egui::Color32::from_rgb(255, 105, 0),
};

pub const CADMIUM_RED: PaintColor = PaintColor {
    name: "Cadmium Red",
    color: egui::Color32::from_rgb(255, 39, 2),
};

pub const QUINACRIDONE_MAGENTA: PaintColor = PaintColor {
    name: "Quinacridone Magenta",
    color: egui::Color32::from_rgb(128, 2, 46),
};

pub const COBALT_VIOLET: PaintColor = PaintColor {
    name: "Cobalt Violet",
    color: egui::Color32::from_rgb(78, 0, 66),
};

pub const ULTRAMARINE_BLUE: PaintColor = PaintColor {
    name: "Ultramarine Blue",
    color: egui::Color32::from_rgb(25, 0, 89),
};

pub const COBALT_BLUE: PaintColor = PaintColor {
    name: "Cobalt Blue",
    color: egui::Color32::from_rgb(0, 33, 133),
};

pub const PHTHALO_BLUE: PaintColor = PaintColor {
    name: "Phthalo Blue",
    color: egui::Color32::from_rgb(13, 27, 68),
};

pub const PHTHALO_GREEN: PaintColor = PaintColor {
    name: "Phthalo Green",
    color: egui::Color32::from_rgb(0, 60, 50),
};

pub const PERMANENT_GREEN: PaintColor = PaintColor {
    name: "Permanent Green",
    color: egui::Color32::from_rgb(7, 109, 22),
};

pub const SAP_GREEN: PaintColor = PaintColor {
    name: "Sap Green",
    color: egui::Color32::from_rgb(107, 148, 4),
};

pub const BURNT_SIENNA: PaintColor = PaintColor {
    name: "Burnt Sienna",
    color: egui::Color32::from_rgb(123, 72, 0),
};

pub const BLACK: PaintColor = PaintColor {
    name: "Black",
    color: egui::Color32::from_rgb(0, 0, 0),
};

