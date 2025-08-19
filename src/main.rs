#![windows_subsystem = "windows"]

use eframe::egui;
use rfd::FileDialog;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 375.0])
            .with_min_inner_size([500.0, 375.0]),
        ..Default::default()
    };

    eframe::run_native(
        "redesigned-fishstick - bspatch/bsdiff GUI",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    // Create Patch tab
    old_file_path: Option<PathBuf>,
    old_file_text: String,
    new_file_path: Option<PathBuf>,
    new_file_text: String,
    // create_status: String,

    // Apply Patch tab
    file_to_patch_path: Option<PathBuf>,
    file_to_patch_text: String,
    patch_file_path: Option<PathBuf>,
    patch_file_text: String,
    apply_status: String,

    // Global status bar
    global_status: String,
    status_color: egui::Color32,

    // UI state
    current_tab: Tab,
}

#[derive(PartialEq)]
enum Tab {
    CreatePatch,
    ApplyPatch,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            old_file_path: None,
            old_file_text: String::new(),
            new_file_path: None,
            new_file_text: String::new(),
            // create_status: String::new(),
            file_to_patch_path: None,
            file_to_patch_text: String::new(),
            patch_file_path: None,
            patch_file_text: String::new(),
            apply_status: String::new(),
            global_status: "Ready".to_string(),
            status_color: egui::Color32::from_rgb(100, 150, 200),
            current_tab: Tab::CreatePatch,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();

            // Tab selection with improved styling
            ui.horizontal(|ui| {
                ui.spacing_mut().button_padding = egui::vec2(16.0, 8.0);
                ui.spacing_mut().item_spacing.x = 4.0;

                // Create Patch tab
                let create_selected = self.current_tab == Tab::CreatePatch;
                let mut create_button =
                    egui::Button::new("📄 Create Patch").min_size(egui::vec2(120.0, 32.0));

                if create_selected {
                    create_button = create_button.fill(ui.visuals().selection.bg_fill);
                }

                if ui.add(create_button).clicked() {
                    self.current_tab = Tab::CreatePatch;
                }

                // Apply Patch tab
                let apply_selected = self.current_tab == Tab::ApplyPatch;
                let mut apply_button =
                    egui::Button::new("🔧 Apply Patch").min_size(egui::vec2(120.0, 32.0));

                if apply_selected {
                    apply_button = apply_button.fill(ui.visuals().selection.bg_fill);
                }

                if ui.add(apply_button).clicked() {
                    self.current_tab = Tab::ApplyPatch;
                }
            });

            ui.separator();

            // Main content area with reserved space for status bar
            let available_height = ui.available_height() - 40.0; // Increase reserved space for status bar
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), available_height),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| match self.current_tab {
                    Tab::CreatePatch => self.create_patch_tab(ui),
                    Tab::ApplyPatch => self.apply_patch_tab(ui),
                },
            );

            // Status bar at the bottom
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label(egui::RichText::new(&self.global_status).color(self.status_color));
            });
        });
    }
}

impl MyApp {
    fn create_patch_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(15.0);

        // Section header with icon
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("📄").size(20.0));
            ui.label(egui::RichText::new("Create Patch").size(18.0).strong());
        });

        ui.add_space(15.0);

        // Old file input with improved styling
        egui::Frame::none()
            .fill(ui.visuals().faint_bg_color)
            .inner_margin(egui::Margin::same(12.0))
            .rounding(6.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Old file:").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📁 Browse...").clicked()
                            && let Some(path) =
                                FileDialog::new().set_title("Select old file").pick_file()
                        {
                            self.old_file_text = path.display().to_string();
                            self.old_file_path = Some(path);
                        }
                    });
                });

                ui.add_space(5.0);

                // Editable text input for file path
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.old_file_text)
                        .hint_text("Enter file path or use Browse...")
                        .desired_width(f32::INFINITY),
                );

                // Update path when text changes
                if response.changed() {
                    let path = PathBuf::from(&self.old_file_text);
                    if path.exists() && path.is_file() {
                        self.old_file_path = Some(path);
                    } else if self.old_file_text.is_empty() {
                        self.old_file_path = None;
                    }
                }
            });

        ui.add_space(10.0);

        // New file input with improved styling
        egui::Frame::none()
            .fill(ui.visuals().faint_bg_color)
            .inner_margin(egui::Margin::same(12.0))
            .rounding(6.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("New file:").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📁 Browse...").clicked()
                            && let Some(path) =
                                FileDialog::new().set_title("Select new file").pick_file()
                        {
                            self.new_file_text = path.display().to_string();
                            self.new_file_path = Some(path);
                        }
                    });
                });

                ui.add_space(5.0);

                // Editable text input for file path
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.new_file_text)
                        .hint_text("Enter file path or use Browse...")
                        .desired_width(f32::INFINITY),
                );

                // Update path when text changes
                if response.changed() {
                    let path = PathBuf::from(&self.new_file_text);
                    if path.exists() && path.is_file() {
                        self.new_file_path = Some(path);
                    } else if self.new_file_text.is_empty() {
                        self.new_file_path = None;
                    }
                }
            });

        ui.add_space(20.0);

        // Create patch button with improved styling
        let can_create = self.old_file_path.is_some() && self.new_file_path.is_some();
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 150.0) / 2.0); // Center manually
            let mut button = egui::Button::new(egui::RichText::new("🚀 Create Patch").size(16.0))
                .min_size(egui::vec2(150.0, 40.0));

            if can_create {
                button = button.fill(egui::Color32::from_rgb(0, 120, 215));
            }

            if ui.add_enabled(can_create, button).clicked() {
                self.create_patch();
            }
        });
    }

    fn apply_patch_tab(&mut self, ui: &mut egui::Ui) {
        ui.add_space(15.0);

        // Section header with icon
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("🔧").size(20.0));
            ui.label(egui::RichText::new("Apply Patch").size(18.0).strong());
        });

        ui.add_space(15.0);

        // File to patch input with improved styling
        egui::Frame::none()
            .fill(ui.visuals().faint_bg_color)
            .inner_margin(egui::Margin::same(12.0))
            .rounding(6.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("File to patch:").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📁 Browse...").clicked()
                            && let Some(path) = FileDialog::new()
                                .set_title("Select file to patch")
                                .pick_file()
                        {
                            self.file_to_patch_text = path.display().to_string();
                            self.file_to_patch_path = Some(path);
                        }
                    });
                });

                ui.add_space(5.0);

                // Editable text input for file path
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.file_to_patch_text)
                        .hint_text("Enter file path or use Browse...")
                        .desired_width(f32::INFINITY),
                );

                // Update path when text changes
                if response.changed() {
                    let path = PathBuf::from(&self.file_to_patch_text);
                    if path.exists() && path.is_file() {
                        self.file_to_patch_path = Some(path);
                    } else if self.file_to_patch_text.is_empty() {
                        self.file_to_patch_path = None;
                    }
                }
            });

        ui.add_space(10.0);

        // Patch file input with improved styling
        egui::Frame::none()
            .fill(ui.visuals().faint_bg_color)
            .inner_margin(egui::Margin::same(12.0))
            .rounding(6.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Patch file:").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("📁 Browse...").clicked()
                            && let Some(path) = FileDialog::new()
                                .set_title("Select patch file")
                                .add_filter("bsdiff patch", &["bsdiff"])
                                .pick_file()
                        {
                            self.patch_file_text = path.display().to_string();
                            self.patch_file_path = Some(path);
                        }
                    });
                });

                ui.add_space(5.0);

                // Editable text input for file path
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.patch_file_text)
                        .hint_text("Enter patch file path or use Browse...")
                        .desired_width(f32::INFINITY),
                );

                // Update path when text changes
                if response.changed() {
                    let path = PathBuf::from(&self.patch_file_text);
                    if path.exists() && path.is_file() {
                        self.patch_file_path = Some(path);
                    } else if self.patch_file_text.is_empty() {
                        self.patch_file_path = None;
                    }
                }
            });

        ui.add_space(20.0);

        // Apply patch button with improved styling
        let can_apply = self.file_to_patch_path.is_some() && self.patch_file_path.is_some();
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 150.0) / 2.0); // Center manually
            let mut button = egui::Button::new(egui::RichText::new("⚡ Apply Patch").size(16.0))
                .min_size(egui::vec2(150.0, 40.0));

            if can_apply {
                button = button.fill(egui::Color32::from_rgb(0, 150, 100));
            }

            if ui.add_enabled(can_apply, button).clicked() {
                self.apply_patch();
            }
        });
    }

    fn create_patch(&mut self) {
        let old_path = self.old_file_path.as_ref().unwrap();
        let new_path = self.new_file_path.as_ref().unwrap();

        // Determine default save location (same directory as new file)
        let default_dir = new_path.parent().unwrap_or(new_path);

        if let Some(patch_path) = FileDialog::new()
            .set_title("Save patch file as")
            .set_directory(default_dir)
            .add_filter("bsdiff patch", &["bsdiff"])
            .set_file_name("patch.bsdiff")
            .save_file()
        {
            // Update status to show we're working
            self.global_status = "Creating patch...".to_string();
            self.status_color = egui::Color32::from_rgb(255, 165, 0); // Orange for in-progress

            match self.create_patch_internal(old_path, new_path, &patch_path) {
                Ok(_) => {
                    self.global_status = format!(
                        "✅ Patch created: {}",
                        patch_path.file_name().unwrap_or_default().to_string_lossy()
                    );
                    self.status_color = egui::Color32::from_rgb(0, 150, 0); // Green for success
                    // Auto-populate the apply tab with the created patch
                    self.patch_file_path = Some(patch_path.clone());
                    self.patch_file_text = patch_path.display().to_string();
                }
                Err(e) => {
                    self.global_status = format!("❌ Error creating patch: {}", e);
                    self.status_color = egui::Color32::from_rgb(200, 0, 0); // Red for error
                }
            }
        } else {
            self.global_status = "Patch creation cancelled".to_string();
            self.status_color = egui::Color32::from_rgb(150, 150, 150); // Gray for cancelled
        }
    }

    fn create_patch_internal(
        &self,
        old_path: &PathBuf,
        new_path: &PathBuf,
        patch_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_data = fs::read(old_path)?;
        let new_data = fs::read(new_path)?;

        let mut patch_data = Vec::new();
        bsdiff::diff(&old_data, &new_data, &mut patch_data)?;

        fs::write(patch_path, patch_data)?;
        Ok(())
    }

    fn apply_patch(&mut self) {
        let file_path = self.file_to_patch_path.as_ref().unwrap();
        let patch_path = self.patch_file_path.as_ref().unwrap();

        // Determine default save location (same directory as file to patch)
        let default_dir = file_path.parent().unwrap_or(file_path);
        let default_name = format!(
            "{}_patched",
            file_path.file_stem().unwrap_or_default().to_string_lossy()
        );

        if let Some(output_path) = FileDialog::new()
            .set_title("Save patched file as")
            .set_directory(default_dir)
            .set_file_name(&default_name)
            .save_file()
        {
            // Update status to show we're working
            self.global_status = "Applying patch...".to_string();
            self.status_color = egui::Color32::from_rgb(255, 165, 0); // Orange for in-progress

            match self.apply_patch_internal(file_path, patch_path, &output_path) {
                Ok(_) => {
                    self.global_status = format!(
                        "✅ Patch applied: {}",
                        output_path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                    );
                    self.status_color = egui::Color32::from_rgb(0, 150, 0); // Green for success
                }
                Err(e) => {
                    self.global_status = format!("❌ Error applying patch: {}", e);
                    self.status_color = egui::Color32::from_rgb(200, 0, 0); // Red for error
                }
            }
        } else {
            self.apply_status = "Patch application cancelled".to_string();
            self.global_status = "Patch application cancelled".to_string();
            self.status_color = egui::Color32::from_rgb(150, 150, 150); // Gray for cancelled
        }
    }

    fn apply_patch_internal(
        &self,
        file_path: &PathBuf,
        patch_path: &PathBuf,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_data = fs::read(file_path)?;
        let patch_data = fs::read(patch_path)?;

        // Create a cursor for the patch data (implements Read trait)
        let mut patch_cursor = Cursor::new(patch_data);

        // Create output vector
        let mut new_data = Vec::new();

        // Apply the patch - bsdiff::patch takes (old_data, patch_reader, output_vec)
        bsdiff::patch(&old_data, &mut patch_cursor, &mut new_data)?;

        fs::write(output_path, new_data)?;
        Ok(())
    }
}
