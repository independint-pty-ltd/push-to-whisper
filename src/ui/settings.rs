use anyhow::Result;
use log::{error, info};
use std::sync::Arc;
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use std::thread;
use eframe::egui;

use crate::utils::{Args, get_config, save_config, VALID_MODELS, DEFAULT_MODEL, DEFAULT_LONG_PRESS_THRESHOLD, 
                  DEFAULT_HEADPHONE_KEEPALIVE_INTERVAL, DEFAULT_ENABLE_DEBUG_RECORDING, DEFAULT_BEEP_VOLUME};

// Global state for the settings window
static SETTINGS_WINDOW_OPEN: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

struct SettingsApp {
    settings: Args,
    original_settings: Args,
    dirty: bool,
}

impl eframe::App for SettingsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Push to Whisper Settings");
            ui.add_space(10.0);
            
            // General settings
            ui.collapsing("General Settings", |ui| {
                let mut enable_beep = !self.settings.disable_beep;
                let mut enable_tray = !self.settings.disable_tray;
                let mut enable_visual = !self.settings.disable_visual;
                
                ui.checkbox(&mut enable_beep, "Enable audio feedback");
                ui.checkbox(&mut enable_tray, "Enable system tray icon");
                ui.checkbox(&mut enable_visual, "Enable visual feedback");
                
                // Update the settings with the negated values
                self.settings.disable_beep = !enable_beep;
                self.settings.disable_tray = !enable_tray;
                self.settings.disable_visual = !enable_visual;
                
                ui.add_space(5.0);
                ui.label("Beep volume:");
                ui.add(egui::Slider::new(&mut self.settings.beep_volume, 0.0..=1.0)
                    .text("Volume"));
            });
            
            // Audio settings
            ui.collapsing("Audio Settings", |ui| {
                ui.label("Long press threshold (ms):");
                ui.add(egui::Slider::new(&mut self.settings.long_press_threshold, 100..=2000)
                    .text("ms"));
                
                ui.label("Headphone keepalive interval (s):");
                ui.add(egui::Slider::new(&mut self.settings.headphone_keepalive_interval, 0..=120)
                    .text("seconds"));
                
                ui.checkbox(&mut self.settings.enable_debug_recording, "Enable debug recording");
            });
            
            // Whisper settings
            ui.collapsing("Whisper Settings", |ui| {
                ui.label("Model size:");
                egui::ComboBox::from_label("Select model")
                    .selected_text(&self.settings.model_size)
                    .show_ui(ui, |ui| {
                        for model in VALID_MODELS.iter() {
                            ui.selectable_value(&mut self.settings.model_size, model.to_string(), *model);
                        }
                    });
                
                ui.checkbox(&mut self.settings.force_cpu, "Force CPU mode (disable GPU)");
            });
            
            ui.add_space(20.0);
            
            // Check if settings have changed
            self.dirty = self.settings != self.original_settings;
            
            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    if let Err(e) = self.save_settings() {
                        error!("Failed to save settings: {}", e);
                    } else {
                        info!("Settings saved successfully");
                        *SETTINGS_WINDOW_OPEN.lock() = false;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                
                if ui.button("Cancel").clicked() {
                    *SETTINGS_WINDOW_OPEN.lock() = false;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                
                if ui.button("Reset to Defaults").clicked() {
                    self.reset_to_defaults();
                }
            });
            
            if self.dirty {
                ui.label("You have unsaved changes.");
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
        }
    }
    
    fn save_settings(&self) -> Result<()> {
        // Save settings to the config file
        save_config(&self.settings)
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
        };
        self.dirty = true;
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
            Box::new(|_cc| Box::new(SettingsApp::new())),
        ).expect("Failed to start settings window");
        
        // When the window is closed, set is_open back to false
        *SETTINGS_WINDOW_OPEN.lock() = false;
    });
    
    Ok(())
}

/// Checks if the settings window is currently open
pub fn is_settings_window_open() -> bool {
    *SETTINGS_WINDOW_OPEN.lock()
}

/// Closes the settings window if it's open
pub fn close_settings() -> Result<()> {
    let mut is_open = SETTINGS_WINDOW_OPEN.lock();
    
    if !*is_open {
        return Ok(());
    }
    
    // TODO: Implement actual window closing
    // For now, we'll just set the flag to false
    info!("Settings window would be closed here");
    
    *is_open = false;
    
    Ok(())
} 