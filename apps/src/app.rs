use crate::{Locale, set_current_locale};
use image::DynamicImage;
use rfd::FileDialog;
use rust_intl::t;
use transpic_core::{blur, brighten, grayscale, huerotate, invert, pixelate, resize, rotate};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    image_path: Option<String>,
    blur: f32,
    brightness: i32,
    grayscale: bool,
    huerotate: i32,
    invert: bool,
    pixelate: u32,
    resize_width: u32,
    resize_height: u32,
    rotate: i32,

    #[serde(skip)]
    show_blur: bool,
    #[serde(skip)]
    show_brightness: bool,
    #[serde(skip)]
    show_grayscale: bool,
    #[serde(skip)]
    show_huerotate: bool,
    #[serde(skip)]
    show_invert: bool,
    #[serde(skip)]
    show_pixelate: bool,
    #[serde(skip)]
    show_resize: bool,
    #[serde(skip)]
    show_rotate: bool,

    #[serde(skip)]
    processed_image: Option<DynamicImage>,
    #[serde(skip)]
    processed_texture: Option<egui::TextureHandle>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            image_path: None,
            blur: 0.0,
            brightness: 0,
            grayscale: false,
            huerotate: 0,
            invert: false,
            pixelate: 1,
            resize_width: 0,
            resize_height: 0,
            rotate: 0,
            show_blur: false,
            show_brightness: false,
            show_grayscale: false,
            show_huerotate: false,
            show_invert: false,
            show_pixelate: false,
            show_resize: false,
            show_rotate: false,
            processed_image: None,
            processed_texture: None,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn any_panel_open(&self) -> bool {
        self.show_blur
            || self.show_brightness
            || self.show_grayscale
            || self.show_huerotate
            || self.show_invert
            || self.show_pixelate
            || self.show_resize
            || self.show_rotate
    }

    fn apply(&mut self, ctx: &egui::Context) {
        let Some(path) = &self.image_path else { return };
        let Ok(mut img) = image::open(path) else {
            return;
        };

        if self.show_blur && self.blur > 0.0 {
            img = blur::blur(img, self.blur);
        }
        if self.show_brightness && self.brightness != 0 {
            if let Ok(i) = brighten::brighten(img.clone(), self.brightness) {
                img = i;
            }
        }
        if self.show_grayscale && self.grayscale {
            img = grayscale::grayscale(img);
        }
        if self.show_huerotate && self.huerotate != 0 {
            if let Ok(i) = huerotate::huerotate(img.clone(), self.huerotate) {
                img = i;
            }
        }
        if self.show_invert && self.invert {
            img = invert::invert(img);
        }
        if self.show_pixelate && self.pixelate > 1 {
            if let Ok(i) = pixelate::pixelate(img.clone(), self.pixelate) {
                img = i;
            }
        }
        if self.show_resize && self.resize_width > 0 && self.resize_height > 0 {
            let s = format!("{}x{}", self.resize_width, self.resize_height);
            if let Ok(i) = resize::resize(img.clone(), &s) {
                img = i;
            }
        }
        if self.show_rotate && self.rotate != 0 {
            if let Ok(i) = rotate::rotate(img.clone(), self.rotate as u32) {
                img = i;
            }
        }

        let rgba = img.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let color = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        let handle = ctx.load_texture("processed", color, egui::TextureOptions::LINEAR);

        self.processed_texture = Some(handle);
        self.processed_image = Some(img);
    }

    fn download_dialog(img: &DynamicImage) {
        if let Some(save) = FileDialog::new()
            .add_filter("PNG", &["png"])
            .add_filter("JPEG", &["jpg", "jpeg"])
            .add_filter("WebP", &["webp"])
            .add_filter("BMP", &["bmp"])
            .add_filter("TIFF", &["tiff", "tif"])
            .add_filter("ICO", &["ico"])
            .add_filter("QOI", &["qoi"])
            .add_filter("PNM", &["ppm", "pgm", "pbm", "pam"])
            .add_filter("TGA", &["tga"])
            .add_filter("Farbfeld", &["ff", "farbfeld"])
            .add_filter("OpenEXR", &["exr"])
            .set_file_name("output")
            .save_file()
        {
            let _ = img.save(save);
        }
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        egui::Panel::top("top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button(t!("app.menu.settings"), |ui| {
                    if ui.button("English").clicked() {
                        set_current_locale(Locale::En);
                        ui.close();
                    }
                    if ui.button("Français").clicked() {
                        set_current_locale(Locale::Fr);
                        ui.close();
                    }
                });

                ui.menu_button(t!("app.menu.file"), |ui| {
                    if ui.button(t!("app.menu.import")).clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter(
                                "Images",
                                &[
                                    "avif", "bmp", "exr", "ff", "farbfeld", "gif", "hdr", "ico",
                                    "jpeg", "jpg", "png", "pnm", "qoi", "tga", "tiff", "tif",
                                    "webp",
                                ],
                            )
                            .pick_file()
                        {
                            self.image_path = Some(path.to_string_lossy().to_string());
                            self.processed_texture = None;
                            self.processed_image = None;
                        }
                        ui.close();
                    }
                    if ui.button(t!("app.menu.download")).clicked() {
                        if let Some(img) = &self.processed_image {
                            Self::download_dialog(img);
                        }
                        ui.close();
                    }
                });

                ui.menu_button(t!("app.menu.operations"), |ui| {
                    if ui.button(t!("app.operations.blur")).clicked() {
                        self.show_blur = !self.show_blur;
                        ui.close();
                    }
                    if ui.button(t!("app.operations.brightness")).clicked() {
                        self.show_brightness = !self.show_brightness;
                        ui.close();
                    }
                    if ui.button(t!("app.operations.grayscale")).clicked() {
                        self.show_grayscale = !self.show_grayscale;
                        ui.close();
                    }
                    if ui.button(t!("app.operations.hue_rotate")).clicked() {
                        self.show_huerotate = !self.show_huerotate;
                        ui.close();
                    }
                    if ui.button(t!("app.operations.invert")).clicked() {
                        self.show_invert = !self.show_invert;
                        ui.close();
                    }
                    if ui.button(t!("app.operations.pixelate")).clicked() {
                        self.show_pixelate = !self.show_pixelate;
                        ui.close();
                    }
                    if ui.button(t!("app.operations.resize")).clicked() {
                        self.show_resize = !self.show_resize;
                        if self.show_resize {
                            if let Some(img) = &self.processed_image {
                                self.resize_width = img.width();
                                self.resize_height = img.height();
                            } else if let Some(path) = &self.image_path {
                                if let Ok(img) = image::open(path) {
                                    self.resize_width = img.width();
                                    self.resize_height = img.height();
                                }
                            }
                        }
                        ui.close();
                    }
                    if ui.button(t!("app.operations.rotate")).clicked() {
                        self.show_rotate = !self.show_rotate;
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut theme = ui.ctx().options(|opt| opt.theme_preference);
                    let mut changed = false;

                    changed |= ui
                        .selectable_value(
                            &mut theme,
                            egui::ThemePreference::System,
                            t!("theme.system"),
                        )
                        .changed();
                    changed |= ui
                        .selectable_value(
                            &mut theme,
                            egui::ThemePreference::Light,
                            t!("theme.light"),
                        )
                        .changed();
                    changed |= ui
                        .selectable_value(
                            &mut theme,
                            egui::ThemePreference::Dark,
                            t!("theme.dark"),
                        )
                        .changed();

                    if changed {
                        ui.ctx().set_theme(theme);
                    }
                });
            });
        });

        if self.any_panel_open() {
            egui::Panel::left("ops_panel")
                .resizable(false)
                .exact_size(240.0)
                .show(ui, |ui| {
                    egui::Panel::bottom("apply_panel")
                        .frame(egui::Frame::NONE)
                        .show(ui, |ui| {
                            ui.add_space(6.0);
                            if ui
                                .add_sized(
                                    [ui.available_width(), 32.0],
                                    egui::Button::new(t!("app.panel.apply")),
                                )
                                .clicked()
                            {
                                self.apply(&ctx);
                            }
                            ui.add_space(6.0);

                            let has_image = self.processed_image.is_some();
                            ui.add_enabled_ui(has_image, |ui| {
                                if ui
                                    .add_sized(
                                        [ui.available_width(), 32.0],
                                        egui::Button::new(t!("app.panel.download")),
                                    )
                                    .clicked()
                                {
                                    if let Some(img) = &self.processed_image {
                                        Self::download_dialog(img);
                                    }
                                }
                            });
                            ui.add_space(6.0);
                        });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(6.0);

                        if self.show_blur {
                            section_header(ui, &t!("app.operations.blur"), &mut self.show_blur);
                            ui.add(egui::Slider::new(&mut self.blur, 0.0..=50.0).text("sigma"));
                            ui.separator();
                        }
                        if self.show_brightness {
                            section_header(
                                ui,
                                &t!("app.operations.brightness"),
                                &mut self.show_brightness,
                            );
                            ui.add(
                                egui::Slider::new(&mut self.brightness, -100..=100)
                                    .text(t!("app.panel.amount")),
                            );
                            ui.separator();
                        }
                        if self.show_grayscale {
                            section_header(
                                ui,
                                &t!("app.operations.grayscale"),
                                &mut self.show_grayscale,
                            );
                            ui.checkbox(&mut self.grayscale, "Enable");
                            ui.separator();
                        }
                        if self.show_huerotate {
                            section_header(
                                ui,
                                &t!("app.operations.hue_rotate"),
                                &mut self.show_huerotate,
                            );
                            ui.add(egui::Slider::new(&mut self.huerotate, 0..=360).text("°"));
                            ui.separator();
                        }
                        if self.show_invert {
                            section_header(ui, &t!("app.operations.invert"), &mut self.show_invert);
                            ui.checkbox(&mut self.invert, t!("app.panel.enable"));
                            ui.separator();
                        }
                        if self.show_pixelate {
                            section_header(
                                ui,
                                &t!("app.operations.pixelate"),
                                &mut self.show_pixelate,
                            );
                            ui.add(
                                egui::Slider::new(&mut self.pixelate, 1..=50)
                                    .text(t!("app.panel.px")),
                            );
                            ui.separator();
                        }
                        if self.show_resize {
                            section_header(ui, &t!("app.operations.resize"), &mut self.show_resize);
                            ui.add(
                                egui::DragValue::new(&mut self.resize_width)
                                    .prefix(t!("app.panel.width_prefix")),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.resize_height)
                                    .prefix(t!("app.panel.height_prefix")),
                            );
                            ui.label(t!("app.panel.resize_note"));
                            ui.separator();
                        }
                        if self.show_rotate {
                            section_header(ui, &t!("app.operations.rotate"), &mut self.show_rotate);
                            ui.add(
                                egui::Slider::new(&mut self.rotate, 0..=270)
                                    .step_by(90.0)
                                    .text(t!("app.panel.degree")),
                            );
                            ui.separator();
                        }
                    });
                });
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                if let Some(tex) = &self.processed_texture {
                    ui.add(
                        egui::Image::new(tex)
                            .shrink_to_fit()
                            .max_size(egui::vec2(512.0, 512.0)),
                    );
                } else if let Some(path) = &self.image_path {
                    ui.add(
                        egui::Image::new(format!("file:///{}", path.replace('\\', "/")))
                            .shrink_to_fit()
                            .max_size(egui::vec2(512.0, 512.0)),
                    );
                } else {
                    ui.label(t!("app.central.placeholder"));
                }
            });
        });
    }
}

fn section_header(ui: &mut egui::Ui, title: &str, flag: &mut bool) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(title).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(t!("app.panel.close")).clicked() {
                *flag = false;
            }
        });
    });
}
