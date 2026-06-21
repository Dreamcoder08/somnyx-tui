// main.rs — SOMNYX TUI | Somnus × Nyx | dreamcoder08
use std::io;
use std::time::Duration;
use std::process::Command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod data;
mod ui;

use app::App;

// Accion a ejecutar despues de cerrar el TUI
#[derive(Debug)]
enum PostAction {
    OpenJournal,
    OpenYazi,
    OpenFind,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let action = run_app(&mut terminal, app);

    // Restaurar terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    // Ejecutar accion post-TUI
    match action {
        Ok(Some(PostAction::OpenJournal)) => launch_journal(),
        Ok(Some(PostAction::OpenYazi))    => launch_yazi(),
        Ok(Some(PostAction::OpenFind))    => launch_find(),
        Ok(None) | Err(_)                 => {}
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<Option<PostAction>> {
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Ignorar key-release para evitar eventos duplicados
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Char('r')                => app.refresh(),
                    KeyCode::Char('?')                => app.toggle_help(),
                    KeyCode::Char('j')                => return Ok(Some(PostAction::OpenJournal)),
                    KeyCode::Char('y')                => return Ok(Some(PostAction::OpenYazi)),
                    KeyCode::Char('f')                => return Ok(Some(PostAction::OpenFind)),
                    _ => {}
                }
            }
        }

        app.tick();
    }
}

// ── Launchers post-TUI ────────────────────────────────────────────────────────

fn launch_journal() {
    let home  = std::env::var("HOME").unwrap_or_else(|_| "/home/dreamcoder08".into());
    let notes = std::env::var("SOMNYX_NOTES").unwrap_or_else(|_| format!("{}/notes", home));
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let path  = format!("{}/journal/{}.md", notes, today);

    std::fs::create_dir_all(format!("{}/journal", notes)).ok();

    if !std::path::Path::new(&path).exists() {
        let content = format!("# {}\n\n## Objetivos\n\n-\n\n## Notas tecnicas\n\n## Pendientes\n\n", today);
        std::fs::write(&path, content).ok();
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".into());
    let _ = Command::new(editor).arg(&path).status();
}

fn launch_yazi() {
    let home      = std::env::var("HOME").unwrap_or_else(|_| "/home/dreamcoder08".into());
    let workspace = std::env::var("SOMNYX_WORKSPACE").unwrap_or_else(|_| format!("{}/somnyx", home));
    let _ = Command::new("yazi").arg(&workspace).status();
}

fn launch_find() {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/dreamcoder08".into());
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "fd . '{}' --exclude .git --exclude node_modules --exclude target \
             | fzf --preview 'bat --color=always {{}}' \
               --bind 'enter:execute($EDITOR {{}})'",
            home
        ))
        .status();
}
