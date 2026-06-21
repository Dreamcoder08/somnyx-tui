// app.rs — Estado de la aplicacion SOMNYX TUI
use crate::data::{self, SystemStats, WorkspaceStats};

pub struct App {
    pub system:     SystemStats,
    pub workspace:  WorkspaceStats,
    pub show_help:  bool,
    pub tick_count: u64,
    pub status_msg: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            system:     SystemStats::default(),
            workspace:  WorkspaceStats::default(),
            show_help:  false,
            tick_count: 0,
            status_msg: None,
        };
        app.refresh();
        app
    }

    pub fn refresh(&mut self) {
        self.system    = data::get_system_stats();
        self.workspace = data::get_workspace_stats();
        self.status_msg = Some(format!(
            "actualizado  {}",
            chrono::Local::now().format("%H:%M:%S")
        ));
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        // Refresh cada ~8 ticks (250ms poll × 8 = ~2s)
        if self.tick_count % 8 == 0 {
            self.workspace = data::get_workspace_stats();
        }
        // CPU/RAM cada ~20 ticks (~5s)
        if self.tick_count % 20 == 0 {
            self.system = data::get_system_stats();
        }
        // Limpiar status msg despues de 12 ticks (~3s)
        if self.tick_count % 12 == 0 {
            self.status_msg = None;
        }
    }
}
