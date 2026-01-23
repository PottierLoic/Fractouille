use crate::export::save_image;
use crate::fractal::{Fractal, Set};
use crate::palette::{InterpolationMode, Palette};
use eframe::egui;

pub fn start_gui_app() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Fractouille GUI"),
        ..Default::default()
    };

    eframe::run_native(
        "gui_app",
        options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(cc)))),
    )
}

struct PaletteCreator {
    is_open: bool,
    name: String,
    colors: Vec<(u8, u8, u8)>,
    cycle_speed: f64,
    interpolation: InterpolationMode,
}

impl Default for PaletteCreator {
    fn default() -> Self {
        Self {
            is_open: false,
            name: "New Palette".to_string(),
            colors: vec![(0, 0, 0)],
            cycle_speed: 1.0,
            interpolation: InterpolationMode::Linear,
        }
    }
}

struct ScreenshotCreator {
    is_open: bool,
    width: u32,
    height: u32,
    fractal: Fractal,
}

struct GuiApp {
    fractal: Fractal,
    texture: Option<egui::TextureHandle>,
    dirty: bool,
    res_scale: f32,

    palette_creator: PaletteCreator,
    screenshot_creator: ScreenshotCreator,
}

impl GuiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            fractal: Fractal::default(),
            texture: None,
            dirty: true,
            res_scale: 0.125,
            palette_creator: PaletteCreator::default(),
            screenshot_creator: ScreenshotCreator {
                is_open: false,
                width: 1920,
                height: 1080,
                fractal: Fractal::default(),
            },
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context, width: u32, height: u32) {
        let grid = self.fractal.render_frame(width, height, false);

        let mut pixels= Vec::with_capacity((width * height * 4) as usize);
        for pixel in grid {
            pixels.push(pixel[0]);
            pixels.push(pixel[1]);
            pixels.push(pixel[2]);
            pixels.push(255);
        }

        let image =
            egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &pixels);

        if let Some(tex) = &mut self.texture {
            tex.set(image, egui::TextureOptions::LINEAR);
        } else {
            self.texture = Some(ctx.load_texture("fractal", image, egui::TextureOptions::LINEAR));
        }
    }

    fn show_screenshot_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("📷 Create Screenshot").show(ctx, |ui| {
            ui.label("Dimensions:");
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(egui::DragValue::new(&mut self.screenshot_creator.width).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Height:");
                ui.add(egui::DragValue::new(&mut self.screenshot_creator.height).speed(1.0));
            });

            if ui.button("Use current view settings").clicked() {
                self.screenshot_creator.fractal = self.fractal.clone();
            }

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    self.screenshot_creator.is_open = false;
                }
                if ui.button("Render & Save").clicked() {
                    let _ = save_image(
                        &self.screenshot_creator.fractal,
                        Some(self.screenshot_creator.width),
                        Some(self.screenshot_creator.height),
                    );
                    self.screenshot_creator.is_open = false;
                }
            });
        });
    }

    fn show_palette_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("🎨 Create New Palette").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.palette_creator.name);
            });

            ui.add(
                egui::Slider::new(&mut self.palette_creator.cycle_speed, 0.1..=100.0).text("Cycle Speed"),
            );

            egui::ComboBox::from_label("Interpolation")
                .selected_text(format!("{:?}", self.palette_creator.interpolation))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.palette_creator.interpolation,
                        InterpolationMode::Linear,
                        "Linear",
                    );
                    ui.selectable_value(
                        &mut self.palette_creator.interpolation,
                        InterpolationMode::Cosine,
                        "Cosine",
                    );
                    ui.selectable_value(
                        &mut self.palette_creator.interpolation,
                        InterpolationMode::Hsv,
                        "HSV",
                    );
                    ui.selectable_value(
                        &mut self.palette_creator.interpolation,
                        InterpolationMode::HsvCyclic,
                        "HSV Cyclic",
                    );
                });

            ui.separator();
            ui.label("Colors:");
            let mut to_remove = None;
            for (i, color) in self.palette_creator.colors.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let mut color_f32 = [
                        color.0 as f32 / 255.0,
                        color.1 as f32 / 255.0,
                        color.2 as f32 / 255.0,
                    ];
                    if ui.color_edit_button_rgb(&mut color_f32).changed() {
                        color.0 = (color_f32[0] * 255.0) as u8;
                        color.1 = (color_f32[1] * 255.0) as u8;
                        color.2 = (color_f32[2] * 255.0) as u8;
                    }
                    if ui.button("🗑").clicked() {
                        to_remove = Some(i);
                    }
                });
            }

            if let Some(i) = to_remove {
                self.palette_creator.colors.remove(i);
            }
            if ui.button("+ Add Color").clicked() {
                self.palette_creator.colors.push((255, 255, 255));
            }

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    self.palette_creator.is_open = false;
                }
                if ui.button("Create").clicked() {
                    let new_palette = Palette {
                        name: self.palette_creator.name.clone(),
                        stops: self.palette_creator.colors.clone(),
                        cycle_speed: self.palette_creator.cycle_speed,
                        interpolation: self.palette_creator.interpolation.clone(),
                    };
                    self.fractal.palette.push(new_palette);
                    self.fractal.current_palette = self.fractal.palette.len() - 1;
                    self.dirty = true;
                    self.palette_creator.is_open = false;
                }
            });
        });
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.screenshot_creator.is_open {
            self.show_screenshot_window(ctx);
        }
        if self.palette_creator.is_open {
            self.show_palette_window(ctx);
        }

        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            ui.heading("Parameters");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Set:");

                egui::ComboBox::from_id_salt("set_selector")
                    .selected_text(format!("{:?}", self.fractal.set))
                    .show_ui(ui, |ui| {
                        let sets = [Set::Mandelbrot, Set::Julia, Set::BurningShip, Set::Phoenix];
                        for set in sets {
                            if ui
                                .selectable_value(&mut self.fractal.set, set.clone(), format!("{:?}", set))
                                .changed()
                            {
                                self.dirty = true;
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Max iterations:");
                if ui
                    .add(egui::DragValue::new(&mut self.fractal.max_iterations).speed(1))
                    .changed()
                {
                    self.dirty = true;
                }
            });

            if self.fractal.set == Set::Julia {
                ui.label("Julia Constant");
                ui.add_space(4.0);

                egui::Grid::new("julia_consts")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Real:");
                        if ui
                            .add(egui::DragValue::new(&mut self.fractal.julia_c.re).speed(0.0005))
                            .changed()
                        {
                            self.dirty = true;
                        }
                        ui.end_row();

                        ui.label("Imaginary:");
                        if ui
                            .add(egui::DragValue::new(&mut self.fractal.julia_c.im).speed(0.0005))
                            .changed()
                        {
                            self.dirty = true;
                        }
                        ui.end_row();
                    });
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Palette:");
                egui::ComboBox::from_id_salt("palette_selector")
                    .selected_text(&self.fractal.palette[self.fractal.current_palette].name)
                    .show_ui(ui, |ui| {
                        for i in 0..self.fractal.palette.len() {
                            if ui
                                .selectable_value(
                                    &mut self.fractal.current_palette,
                                    i,
                                    &self.fractal.palette[i].name,
                                )
                                .changed()
                            {
                                self.dirty = true;
                            }
                        }
                    });
                if ui
                    .button("➕")
                    .on_hover_text("Create custom palette")
                    .clicked()
                {
                    self.palette_creator = PaletteCreator::default();
                    self.palette_creator.is_open = true;
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("Zoom: {:.2e}", self.fractal.scale));
                if ui.button("Reset").clicked() {
                    self.fractal.scale = 1.0;
                    self.dirty = true;
                }
            });

            ui.separator();
            if ui.button("Screenshot").clicked() {
                self.screenshot_creator.fractal = self.fractal.clone();
                self.screenshot_creator.is_open = true;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.available_size();
            let w = size.x;
            let h = size.y;

            if self.dirty {
                self.res_scale = 0.125;
                self.dirty = false;
            }

            if self.res_scale <= 1.0 {
                let rw = (w * self.res_scale).max(1.0) as u32;
                let rh = (h * self.res_scale).max(1.0) as u32;
                self.update_texture(ctx, rw, rh);

                if self.res_scale < 0.25 {
                    self.res_scale = 0.25;
                } else if self.res_scale < 0.5 {
                    self.res_scale = 0.5;
                } else if self.res_scale < 1.0 {
                    self.res_scale = 1.0;
                } else {
                    self.res_scale = 2.0;
                }

                ctx.request_repaint();
            }

            if let Some(texture) = &self.texture {
                let img_widget = egui::Image::from_texture(texture)
                    .fit_to_exact_size(size)
                    .sense(egui::Sense::click_and_drag());

                let response = ui.add(img_widget);

                if response.dragged() {
                    let delta = response.drag_delta();
                    let aspect = w as f64 / h as f64;
                    self.fractal.z.re -= (delta.x as f64 / w as f64) * (aspect * 3.5 / self.fractal.scale);
                    self.fractal.z.im -= (delta.y as f64 / h as f64) * (3.5 / self.fractal.scale);
                    self.dirty = true;
                }

                let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    let zoom_speed = 0.002;
                    let factor = (scroll_delta * zoom_speed).exp() as f64;

                    if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
                        let rect = response.rect;
                        if rect.contains(pointer_pos) {
                            let screen_dx = (pointer_pos.x - rect.center().x) as f64;
                            let screen_dy = (pointer_pos.y - rect.center().y) as f64;
                            let units_per_pixel = (3.5 / self.fractal.scale) / w as f64;
                            let world_dx = screen_dx * units_per_pixel;
                            let world_dy = screen_dy * units_per_pixel;
                            self.fractal.scale *= factor;
                            self.fractal.z.re += world_dx * (1.0 - 1.0 / factor);
                            self.fractal.z.im += world_dy * (1.0 - 1.0 / factor);

                            self.dirty = true;
                        }
                    }
                }
            }
        });
    }
}
