// ui.rs — Renderizado TUI SOMNYX | Somnus × Nyx
use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crate::app::App;

// ── Paleta de colores: noche / sueño ─────────────────────────────────────────
const VIOLET:      Color = Color::Rgb(160, 120, 220);
const DARK_VIOLET: Color = Color::Rgb(60,  40,  100);
const DIM_VIOLET:  Color = Color::Rgb(90,  70,  140);
const LAVENDER:    Color = Color::Rgb(200, 170, 255);
const NIGHT:       Color = Color::Rgb(10,  8,   18);
const PANEL:       Color = Color::Rgb(16,  13,  28);
const MUTED:       Color = Color::Rgb(100, 88,  130);
const SUCCESS:     Color = Color::Rgb(80,  200, 120);
const WARNING:     Color = Color::Rgb(255, 190, 80);
const DANGER:      Color = Color::Rgb(220, 80,  80);

// ── Entry point ───────────────────────────────────────────────────────────────
pub fn render(f: &mut Frame, app: &App) {
    let area = f.area();

    // Fondo global
    f.render_widget(
        Block::default().style(Style::default().bg(NIGHT)),
        area,
    );

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(0),     // body
            Constraint::Length(3),  // footer
        ])
        .split(area);

    render_header(f, app, rows[0]);
    render_body(f, app, rows[1]);
    render_footer(f, app, rows[2]);

    if app.show_help {
        render_help_overlay(f, area);
    }
}

// ── Header ────────────────────────────────────────────────────────────────────
fn render_header(f: &mut Frame, _app: &App, area: Rect) {
    let now = Local::now().format("%Y-%m-%d  %H:%M").to_string();

    let line = Line::from(vec![
        Span::raw("  "),
        Span::styled("SOMNYX", Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD)),
        Span::styled("  ─  ", Style::default().fg(DARK_VIOLET)),
        Span::styled("Somnus × Nyx", Style::default().fg(DIM_VIOLET).add_modifier(Modifier::ITALIC)),
        Span::styled("  ─  ", Style::default().fg(DARK_VIOLET)),
        Span::styled("dreamcoder08", Style::default().fg(VIOLET)),
        Span::styled("  ─  ", Style::default().fg(DARK_VIOLET)),
        Span::styled(now, Style::default().fg(MUTED)),
    ]);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(DARK_VIOLET))
        .style(Style::default().bg(NIGHT));

    f.render_widget(Paragraph::new(line).block(block), area);
}

// ── Body: tres columnas ───────────────────────────────────────────────────────
fn render_body(f: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(38),  // workspace
            Constraint::Percentage(30),  // sistema
            Constraint::Percentage(32),  // inbox + journal
        ])
        .split(area);

    render_workspace(f, app, cols[0]);
    render_system(f, app, cols[1]);
    render_right(f, app, cols[2]);
}

// ── Panel izquierdo: Workspace ────────────────────────────────────────────────
fn render_workspace(f: &mut Frame, app: &App, area: Rect) {
    let ws = &app.workspace;

    let block = panel_block(" WORKSPACE ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let lines = vec![
        Line::from(vec![
            Span::styled("  somnyx/", Style::default().fg(VIOLET).add_modifier(Modifier::BOLD)),
            Span::styled(format!("   {}", ws.workspace_size), Style::default().fg(MUTED)),
        ]),
        dim_line("  ├─ dev/"),
        dim_line("  │  ├─ personal/"),
        dim_line("  │  ├─ work/"),
        dim_line("  │  ├─ oss/"),
        Line::from(Span::styled("  │  └─ arkonyx/", Style::default().fg(VIOLET))),
        dim_line("  ├─ lab/"),
        dim_line("  └─ ops/"),
        Line::from(""),
        size_line("  archive/", &ws.archive_size),
        size_line("  vault/  ", &ws.vault_size),
        size_line("  notes/  ", &ws.notes_size),
        size_line("  media/  ", &ws.media_size),
        size_line("  inbox/  ", &ws.inbox_size),
    ];

    f.render_widget(Paragraph::new(lines), inner);
}

// ── Panel central: Sistema ────────────────────────────────────────────────────
fn render_system(f: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    // Stats CPU / RAM / DISCO
    {
        let sys = &app.system;
        let block = panel_block(" SISTEMA ");
        let inner = block.inner(rows[0]);
        f.render_widget(block, rows[0]);

        let cpu_c = threshold_color(sys.cpu_percent, 50.0, 80.0);
        let ram_p = pct(sys.ram_used_gb, sys.ram_total_gb);
        let ram_c = threshold_color(ram_p, 60.0, 85.0);
        let dsk_p = pct(sys.disk_used_gb, sys.disk_total_gb);
        let dsk_c = threshold_color(dsk_p, 70.0, 85.0);

        let lines = vec![
            Line::from(""),
            stat_label("  CPU ", format!("{:.0}%", sys.cpu_percent), cpu_c),
            bar_line(sys.cpu_percent, 24, cpu_c),
            Line::from(""),
            stat_label("  RAM ", format!("{:.1}/{:.1} GB", sys.ram_used_gb, sys.ram_total_gb), ram_c),
            bar_line(ram_p, 24, ram_c),
            Line::from(""),
            stat_label("  DISK", format!("{:.0}/{:.0} GB", sys.disk_used_gb, sys.disk_total_gb), dsk_c),
            bar_line(dsk_p, 24, dsk_c),
            Line::from(""),
            Line::from(vec![
                Span::styled("  UP   ", Style::default().fg(MUTED)),
                Span::styled(&sys.uptime, Style::default().fg(Color::White)),
            ]),
        ];

        f.render_widget(Paragraph::new(lines), inner);
    }

    // Timers
    {
        let ws = &app.workspace;
        let block = panel_block(" TIMERS ");
        let inner = block.inner(rows[1]);
        f.render_widget(block, rows[1]);

        let lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  clean  ", Style::default().fg(MUTED)),
                Span::styled(&ws.timer_clean, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("  alert  ", Style::default().fg(MUTED)),
                Span::styled(&ws.timer_alert, Style::default().fg(Color::White)),
            ]),
        ];

        f.render_widget(Paragraph::new(lines), inner);
    }
}

// ── Panel derecho: Inbox + Journal ────────────────────────────────────────────
fn render_right(f: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Inbox
    {
        let ws = &app.workspace;
        let inbox_color = if ws.inbox_old > 0 { DANGER }
                         else if ws.inbox_count > 0 { WARNING }
                         else { SUCCESS };

        let block = panel_block(" INBOX ");
        let inner = block.inner(rows[0]);
        f.render_widget(block, rows[0]);

        let mut lines = vec![Line::from("")];
        if ws.inbox_count == 0 {
            lines.push(Line::from(Span::styled(
                "  ✓  limpio",
                Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("{} archivo(s)", ws.inbox_count),
                    Style::default().fg(inbox_color).add_modifier(Modifier::BOLD),
                ),
            ]));
            if ws.inbox_old > 0 {
                lines.push(Line::from(Span::styled(
                    format!("  ! {} sin clasificar >7d", ws.inbox_old),
                    Style::default().fg(DANGER),
                )));
            }
        }

        f.render_widget(Paragraph::new(lines), inner);
    }

    // Journal
    {
        let ws = &app.workspace;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let (j_color, j_status) = if ws.journal_today {
            (SUCCESS, "  ✓  entrada de hoy")
        } else {
            (WARNING, "  ✗  sin entrada aun")
        };

        let block = panel_block(" JOURNAL ");
        let inner = block.inner(rows[1]);
        f.render_widget(block, rows[1]);

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(format!("  {}", today), Style::default().fg(MUTED))),
            Line::from(Span::styled(j_status, Style::default().fg(j_color).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled(
                "  [j] abrir journal",
                Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
            )),
        ];

        f.render_widget(Paragraph::new(lines), inner);
    }
}

// ── Footer ────────────────────────────────────────────────────────────────────
fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let mut spans = vec![Span::raw("  ")];

    let keys = [
        ("[r]", " refresh "),
        ("[j]", " journal "),
        ("[y]", " yazi    "),
        ("[f]", " buscar  "),
        ("[?]", " ayuda   "),
        ("[q]", " salir"),
    ];

    for (key, desc) in &keys {
        spans.push(Span::styled(*key, Style::default().fg(VIOLET).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(*desc, Style::default().fg(MUTED)));
    }

    // Status message
    if let Some(msg) = &app.status_msg {
        spans.push(Span::styled(format!("   ✓ {}", msg), Style::default().fg(SUCCESS)));
    }

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(DARK_VIOLET))
        .style(Style::default().bg(NIGHT));

    f.render_widget(Paragraph::new(Line::from(spans)).block(block), area);
}

// ── Help overlay ──────────────────────────────────────────────────────────────
fn render_help_overlay(f: &mut Frame, area: Rect) {
    let popup = centered_rect(58, 75, area);
    f.render_widget(Clear, popup);

    let block = Block::default()
        .title(Span::styled(" SOMNYX — Ayuda ", Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(VIOLET))
        .style(Style::default().bg(NIGHT));

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled("  Atajos de teclado", Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD))),
        Line::from(""),
        help_key("r", "Refrescar datos del sistema"),
        help_key("j", "Abrir journal de hoy en $EDITOR"),
        help_key("y", "Abrir yazi file manager en ~/somnyx"),
        help_key("f", "Buscar archivos con fd + fzf"),
        help_key("?", "Mostrar / ocultar esta ayuda"),
        help_key("q", "Salir"),
        Line::from(""),
        Line::from(Span::styled("  SOMNYX", Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Somnus  ", Style::default().fg(VIOLET)),
            Span::styled("Dios romano del sueño", Style::default().fg(MUTED)),
        ]),
        Line::from(vec![
            Span::styled("  Nyx     ", Style::default().fg(VIOLET)),
            Span::styled("Diosa griega de la noche, madre de los sueños", Style::default().fg(MUTED)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Autor   ", Style::default().fg(VIOLET)),
            Span::styled("dreamcoder08", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("  Version ", Style::default().fg(VIOLET)),
            Span::styled("2026.03", Style::default().fg(MUTED)),
        ]),
    ];

    f.render_widget(Paragraph::new(lines).block(block).wrap(Wrap { trim: false }), popup);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn panel_block(title: &'static str) -> Block<'static> {
    Block::default()
        .title(Span::styled(title, Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(DARK_VIOLET))
        .style(Style::default().bg(PANEL))
}

fn dim_line(s: &'static str) -> Line<'static> {
    Line::from(Span::styled(s, Style::default().fg(MUTED)))
}

fn size_line<'a>(label: &'a str, size: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(label, Style::default().fg(Color::White)),
        Span::styled(format!("  {}", size), Style::default().fg(MUTED)),
    ])
}

fn stat_label(label: &'static str, value: String, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::styled(label, Style::default().fg(MUTED)),
        Span::styled(format!("  {}", value), Style::default().fg(color).add_modifier(Modifier::BOLD)),
    ])
}

fn bar_line(percent: f32, width: usize, color: Color) -> Line<'static> {
    let filled = ((percent / 100.0) * width as f32).round() as usize;
    let empty  = width.saturating_sub(filled);
    let bar = format!("  {}{}", "█".repeat(filled), "░".repeat(empty));
    Line::from(Span::styled(bar, Style::default().fg(color)))
}

fn help_key(key: &'static str, desc: &'static str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("  [{}]  ", key), Style::default().fg(VIOLET).add_modifier(Modifier::BOLD)),
        Span::styled(desc, Style::default().fg(Color::White)),
    ])
}

fn threshold_color(val: f32, warn: f32, crit: f32) -> Color {
    if val >= crit { DANGER } else if val >= warn { WARNING } else { SUCCESS }
}

fn pct(used: f32, total: f32) -> f32 {
    if total <= 0.0 { 0.0 } else { (used / total * 100.0).clamp(0.0, 100.0) }
}

fn centered_rect(pct_x: u16, pct_y: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - pct_y) / 2),
            Constraint::Percentage(pct_y),
            Constraint::Percentage((100 - pct_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - pct_x) / 2),
            Constraint::Percentage(pct_x),
            Constraint::Percentage((100 - pct_x) / 2),
        ])
        .split(vert[1])[1]
}
