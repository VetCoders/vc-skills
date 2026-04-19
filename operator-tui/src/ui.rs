use crate::app::{App, AppTab, LaunchFocus};
use crate::state::RunKind;
use ratatui::prelude::*;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap};

pub fn draw(frame: &mut Frame, app: &App) {
    let root = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(root);

    draw_header(frame, layout[0], app);
    draw_tabs(frame, layout[1], app);
    draw_body(frame, layout[2], app);
    draw_footer(frame, layout[3], app);

    if app.focus == LaunchFocus::Help {
        draw_help_overlay(frame, app);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    let title = Line::from(vec![
        Span::styled(
            "Vibecrafted Operator Console",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(app.status_summary(), Style::default().fg(Color::Gray)),
    ]);
    frame.render_widget(Paragraph::new(title), rows[0]);

    let context = format!(
        "mission root: {}  |  active runs: {}",
        app.config.launch_root.to_string_lossy(),
        app.active_run_count()
    );
    frame.render_widget(
        Paragraph::new(context).style(Style::default().fg(Color::DarkGray)),
        rows[1],
    );
}

fn draw_tabs(frame: &mut Frame, area: Rect, app: &App) {
    let tabs = Tabs::new(AppTab::TITLES)
        .block(Block::default().borders(Borders::ALL).title("Surface"))
        .select(app.active_tab)
        .divider("│")
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs, area);
}

fn draw_body(frame: &mut Frame, area: Rect, app: &App) {
    match app.active_tab() {
        AppTab::Monitor => draw_monitor(frame, area, app),
        AppTab::Dispatch => draw_dispatch(frame, area, app),
        AppTab::Controls => draw_controls(frame, area, app),
    }
}

fn draw_monitor(frame: &mut Frame, area: Rect, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
        .split(area);

    draw_runs(frame, body[0], app);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(body[1]);
    draw_detail(frame, right[0], app, "Run detail");
    draw_events(frame, right[1], app, "Recent events");
}

fn draw_dispatch(frame: &mut Frame, area: Rect, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_launch(frame, body[0], app);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(44), Constraint::Percentage(56)])
        .split(body[1]);

    let guide_lines = vec![
        Line::from("Dispatch posture"),
        Line::from(""),
        Line::from("Shape the next worker before you launch it."),
        Line::from("Use mission kind for intent, agent for style, runtime for surface."),
        Line::from("Prompt edit is the last mile: keep it sharp and bounded."),
    ];
    let guide = Paragraph::new(guide_lines)
        .block(Block::default().borders(Borders::ALL).title("Playbook"))
        .wrap(Wrap { trim: false });
    frame.render_widget(guide, right[0]);

    draw_launch_history(frame, right[1], app);
}

fn draw_controls(frame: &mut Frame, area: Rect, app: &App) {
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
        .split(area);

    draw_deep_controls(frame, body[0], app);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(body[1]);

    draw_detail(frame, right[0], app, "Selected run");
    draw_events(frame, right[1], app, "Selected timeline");
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let nav_hint = match (app.active_tab(), app.focus) {
        (AppTab::Monitor, _) => {
            "Monitor: ↑/↓ runs  Tab/Shift+Tab switch tabs  f filter  d controls  ? help"
        }
        (AppTab::Dispatch, LaunchFocus::EditPrompt) => {
            "Dispatch edit: type prompt  Backspace delete  Enter/Esc finish  Tab switch tabs"
        }
        (AppTab::Dispatch, _) => {
            "Dispatch: ↑/↓ field  ←/→ change  e edit prompt  Enter launch  1-4 presets"
        }
        (AppTab::Controls, _) => {
            "Controls: ↑/↓ action  ←/→ run selection  Enter open  d jump here from Monitor"
        }
    };
    frame.render_widget(
        Paragraph::new(nav_hint).style(Style::default().fg(Color::Cyan)),
        rows[0],
    );

    let shortcuts = "Global: q quit  r refresh  a cycle agent  v cycle runtime  ? help";
    frame.render_widget(
        Paragraph::new(shortcuts).style(Style::default().fg(Color::DarkGray)),
        rows[1],
    );

    let status = if app.status_line.is_empty() {
        format!("state root: {}", app.config.state_root.to_string_lossy())
    } else {
        app.status_line.clone()
    };
    frame.render_widget(
        Paragraph::new(status).style(Style::default().fg(Color::Gray)),
        rows[2],
    );
}

fn draw_runs(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = if app.runs.is_empty() {
        vec![ListItem::new("No run snapshots found.")]
    } else {
        app.runs
            .iter()
            .enumerate()
            .map(|(idx, run)| {
                let snapshot = &run.snapshot;
                let status = status_style(run.kind);
                let selected = idx == app.selected;
                let label = format!(
                    "{} {} / {} / {}",
                    snapshot.run_id,
                    run.kind.label(),
                    snapshot.agent.as_deref().unwrap_or("unknown"),
                    snapshot.mode.as_deref().unwrap_or("unknown")
                );
                let detail = format!(
                    "{}  {}",
                    run.age_label,
                    snapshot.last_error.as_deref().unwrap_or("")
                );
                let mut spans = vec![
                    Span::styled(label, status),
                    Span::raw("\n"),
                    Span::styled(detail, Style::default().fg(Color::DarkGray)),
                ];
                if selected {
                    spans.insert(0, Span::styled("▶ ", Style::default().fg(Color::Yellow)));
                } else {
                    spans.insert(0, Span::raw("  "));
                }
                ListItem::new(Line::from(spans))
            })
            .collect()
    };

    let title = if app.filter_active_only {
        "Runs (Active/Stalled/Paused)"
    } else {
        "Runs (All)"
    };
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(list, area);
}

fn draw_detail(frame: &mut Frame, area: Rect, app: &App, title: &str) {
    let lines = app
        .detail_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let detail = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false });
    frame.render_widget(detail, area);
}

fn draw_events(frame: &mut Frame, area: Rect, app: &App, title: &str) {
    let lines = app
        .event_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let events = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false });
    frame.render_widget(events, area);
}

fn draw_launch(frame: &mut Frame, area: Rect, app: &App) {
    let lines = app
        .prompt_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();

    let title = if app.focus == LaunchFocus::EditPrompt {
        "Launch panel (editing prompt)"
    } else {
        "Launch panel"
    };

    let launch = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false });
    frame.render_widget(launch, area);
}

fn draw_launch_history(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = if app.launch_history.is_empty() {
        vec![
            Line::from("No launches from this session yet."),
            Line::from(""),
            Line::from("Use Dispatch to stage a worker, then press Enter."),
        ]
    } else {
        app.launch_history
            .iter()
            .rev()
            .map(|entry| Line::from(entry.clone()))
            .collect::<Vec<_>>()
    };
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "selected run: {}",
        app.selected_run()
            .map(|run| run.snapshot.run_id.as_str())
            .unwrap_or("none")
    )));
    let panel = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Launch history"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(panel, area);
}

fn draw_deep_controls(frame: &mut Frame, area: Rect, app: &App) {
    let lines = app
        .deep_control_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let panel = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(Wrap { trim: false });
    frame.render_widget(panel, area);
}

fn status_style(kind: RunKind) -> Style {
    match kind {
        RunKind::Active => Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
        RunKind::Recent | RunKind::Completed => Style::default().fg(Color::Blue),
        RunKind::Failed => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        RunKind::Stalled => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        RunKind::Paused => Style::default().fg(Color::Magenta),
        RunKind::Unknown => Style::default().fg(Color::Gray),
    }
}

fn draw_help_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(72, 70, frame.area());
    frame.render_widget(Clear, area);
    let lines = app
        .help_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let help = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(help, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
