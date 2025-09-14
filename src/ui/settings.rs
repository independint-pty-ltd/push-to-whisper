use anyhow::Result;
use log::{error, info, warn};
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use std::thread;
use std::process::Command;
use eframe::egui;

use crate::utils::{Args, get_config, save_config, VALID_MODELS, DEFAULT_MODEL, DEFAULT_LONG_PRESS_THRESHOLD, 
                  DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL, DEFAULT_ENABLE_DEBUG_RECORDING, DEFAULT_BEEP_VOLUME, DEFAULT_TRANSCRIPTION_PRIORITY, request_exit};

// Global state for the settings window
static SETTINGS_WINDOW_OPEN: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

struct SettingsApp {
    settings: Args,
    original_settings: Args,
    dirty: bool,
    restart_required: bool,
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎙️ Push to Whisper Settings");
            ui.add_space(10.0);
            
            // Show restart warning if needed
            if self.restart_required {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), "⚠️ Application restart required for some changes to take effect");
                ui.add_space(5.0);
            }
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                // General settings
                ui.collapsing("🔧 General Settings", |ui| {
                    let mut enable_beep = !self.settings.disable_beep;
                    let mut enable_tray = !self.settings.disable_tray;
                    let mut enable_visual = !self.settings.disable_visual;
                    
                    if ui.checkbox(&mut enable_beep, "🔊 Enable audio feedback").changed() {
                        self.check_for_changes();
                    }
                    if ui.checkbox(&mut enable_tray, "📍 Enable system tray icon").changed() {
                        self.restart_required = true;
                        self.check_for_changes();
                    }
                    if ui.checkbox(&mut enable_visual, "👁️ Enable visual feedback").changed() {
                        self.check_for_changes();
                    }
                    
                    // Update the settings with the negated values
                    self.settings.disable_beep = !enable_beep;
                    self.settings.disable_tray = !enable_tray;
                    self.settings.disable_visual = !enable_visual;
                    
                    ui.add_space(5.0);
                    ui.label("🔊 Beep volume:");
                    if ui.add(egui::Slider::new(&mut self.settings.beep_volume, 0.0..=1.0)
                        .text("Volume")).changed() {
                        self.check_for_changes();
                    }
                });
                
                ui.add_space(10.0);
                
                // Audio settings
                ui.collapsing("🎵 Audio Settings", |ui| {
                    ui.label("⏱️ Long press threshold (ms):");
                    ui.label("How long to hold the key before recording starts");
                    if ui.add(egui::Slider::new(&mut self.settings.long_press_threshold, 10..=2000)
                        .text("ms")).changed() {
                        self.check_for_changes();
                    }
                    
                    ui.add_space(5.0);
                    ui.label("🎛️ Activation hotkey:");
                    let old_hotkey = self.settings.hotkey.clone();
                    egui::ComboBox::from_label("Select hotkey")
                        .selected_text(match self.settings.hotkey.as_str() { "right_ctrl" => "Right Ctrl", _ => "Right Alt" })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.settings.hotkey, "right_alt".to_string(), "Right Alt (default)");
                            ui.selectable_value(&mut self.settings.hotkey, "right_ctrl".to_string(), "Right Ctrl");
                        });
                    if old_hotkey != self.settings.hotkey {
                        self.check_for_changes();
                    }

                    ui.add_space(5.0);
                    ui.label("🎧 Headphone keepalive interval (s):");
                    ui.label("Prevents wireless headphones from disconnecting (0 = disabled)");
                    if ui.add(egui::Slider::new(&mut self.settings.headphone_keepalive_interval, 0..=120)
                        .text("seconds")).changed() {
                        self.check_for_changes();
                    }
                    
                    ui.add_space(5.0);
                    if ui.checkbox(&mut self.settings.enable_debug_recording, "🐛 Enable debug recording").changed() {
                        self.check_for_changes();
                    }
                    ui.label("Saves audio to debug_recording.wav for troubleshooting");
                });
                
                ui.add_space(10.0);
                
                // Whisper settings
                ui.collapsing("🤖 Whisper AI Settings", |ui| {
                    ui.label("📦 Model size:");
                    ui.label("Larger models are more accurate but slower");
                    
                    let old_model = self.settings.model_size.clone();
                    egui::ComboBox::from_label("Select model")
                        .selected_text(&self.settings.model_size)
                        .show_ui(ui, |ui| {
                            for model in VALID_MODELS.iter() {
                                let model_info = match *model {
                                    "tiny.en" => "Tiny (~75MB) - Fastest, least accurate",
                                    "base.en" => "Base (~150MB) - Fast, decent accuracy", 
                                    "small.en" => "Small (~500MB) - Good balance",
                                    "medium.en" => "Medium (~1.5GB) - High accuracy (Default)",
                                    "large" => "Large (~3GB) - Highest accuracy, all languages",
                                    _ => model,
                                };
                                ui.selectable_value(&mut self.settings.model_size, model.to_string(), model_info);
                            }
                        });
                    
                    if old_model != self.settings.model_size {
                        self.restart_required = true;
                        self.check_for_changes();
                    }
                    
                    ui.add_space(5.0);
                    if ui.checkbox(&mut self.settings.force_cpu, "💻 Force CPU mode (disable GPU acceleration)").changed() {
                        self.restart_required = true;
                        self.check_for_changes();
                    }
                    ui.label("Use this if you have GPU issues or want to save power");
                });
            });
            
            ui.add_space(20.0);
            
            // Check if settings have changed
            self.dirty = self.settings != self.original_settings;
            
            // Action buttons
            ui.horizontal(|ui| {
                let save_button = ui.add_enabled(self.dirty, egui::Button::new("💾 Save"));
                if save_button.clicked() {
                    if let Err(e) = self.save_settings() {
                        error!("Failed to save settings: {}", e);
                    } else {
                        info!("Settings saved successfully");
                        if self.restart_required {
                            self.show_restart_dialog(ctx);
                        } else {
                            *SETTINGS_WINDOW_OPEN.lock() = false;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                }
                
                if ui.button("❌ Cancel").clicked() {
                    *SETTINGS_WINDOW_OPEN.lock() = false;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                
                if ui.button("🔄 Reset to Defaults").clicked() {
                    self.reset_to_defaults();
                }
                
                if self.restart_required && ui.button("🔄 Restart Now").clicked() {
                    self.restart_application();
                }
            });
            
            if self.dirty {
                ui.colored_label(egui::Color32::from_rgb(255, 255, 0), "⚠️ You have unsaved changes.");
            }
        });
    }
}

impl SettingsApp {
    fn new() -> Self {
        let settings = get_config();
        Self {
            original_settings: settings.clone(),
            settings,
            dirty: false,
            restart_required: false,
        }
    }
    
    fn save_settings(&self) -> Result<()> {
        // Save settings to the config file
        save_config(&self.settings)?;
        info!("Configuration saved to file");
        Ok(())
    }
    
    fn reset_to_defaults(&mut self) {
        self.settings = Args {
            disable_beep: false,
            disable_tray: false,
            disable_visual: false,
            model_size: DEFAULT_MODEL.to_string(),
            long_press_threshold: DEFAULT_LONG_PRESS_THRESHOLD,
            headphone_keepalive_interval: DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL,
            enable_debug_recording: DEFAULT_ENABLE_DEBUG_RECORDING,
            force_cpu: false,
            beep_volume: DEFAULT_BEEP_VOLUME,
            transcription_priority: DEFAULT_TRANSCRIPTION_PRIORITY.to_string(),
            hotkey: crate::utils::DEFAULT_HOTKEY.to_string(),
        };
        self.dirty = true;
        self.restart_required = true;
    }
    
    fn check_for_changes(&mut self) {
        self.dirty = self.settings != self.original_settings;
    }
    
    fn show_restart_dialog(&self, ctx: &egui::Context) {
        // For now, just close the window and let the user restart manually
        // In the future, we could implement automatic restart
        warn!("Settings saved. Application restart required for some changes to take effect.");
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    
    fn restart_application(&self) {
        info!("Attempting to restart application...");
        
        // Get the current executable path
        if let Ok(current_exe) = std::env::current_exe() {
            // Start a new instance
            match Command::new(&current_exe).spawn() {
                Ok(_) => {
                    info!("New instance started, requesting exit of current instance");
                    request_exit();
                },
                Err(e) => {
                    error!("Failed to start new instance: {}", e);
                }
            }
        } else {
            error!("Could not determine current executable path for restart");
        }
    }
}

/// Opens the settings window if it's not already open
pub fn open_settings() -> Result<()> {
    let mut is_open = SETTINGS_WINDOW_OPEN.lock();
    
    if *is_open {
        info!("Settings window is already open");
        return Ok(());
    }
    
    // Set the window as open
    *is_open = true;
    
    // Spawn a thread to handle the settings window
    thread::spawn(move || {
        let options = eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([400.0, 500.0])
                .with_resizable(true),
            ..Default::default()
        };
        
        eframe::run_native(
            "Push to Whisper Settings",
            options,
            Box::new(|_cc| Ok(Box::new(SettingsApp::new()))),
        ).expect("Failed to start settings window");
        
        // When the window is closed, set is_open back to false
        *SETTINGS_WINDOW_OPEN.lock() = false;
    });
    
    Ok(())
}

// Removed unused helper functions - settings window management is handled internally 