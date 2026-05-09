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

    match app.focus {
        LaunchFocus::Help => draw_help_overlay(frame, app),
        LaunchFocus::EditPrompt => draw_prompt_overlay(frame, app),
        LaunchFocus::Search => draw_search_overlay(frame, app),
        LaunchFocus::Error => draw_error_overlay(frame, app),
        LaunchFocus::Artifact => draw_artifact_overlay(frame, app),
        _ => {}
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
        "mission root: {}  |  active runs: {}  |  scope: {}  |  focus: {}",
        app.config.launch_root.to_string_lossy(),
        app.active_run_count(),
        app.queue_scope.label(),
        app.active_tab().label()
    );
    frame.render_widget(
        Paragraph::new(context).style(Style::default().fg(Color::DarkGray)),
        rows[1],
    );
}

fn draw_tabs(frame: &mut Frame, area: Rect, app: &App) {
    let tabs = Tabs::new(
        app.tab_labels()
            .into_iter()
            .map(Line::from)
            .collect::<Vec<_>>(),
    )
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
    let mux_lines = app.mux_status_lines();
    let polarize_lines = app.polarize_status_lines();
    let mux_height = if mux_lines.is_empty() {
        0
    } else {
        // header + entries + 2 (top + bottom border). Capped so a noisy mux
        // setup with many services cannot starve the run table; the panel
        // scrolls with `Wrap` past the cap.
        (mux_lines.len() as u16 + 2).clamp(3, 10)
    };
    let polarize_height = if polarize_lines.is_empty() {
        0
    } else {
        (polarize_lines.len() as u16 + 2).clamp(3, 9)
    };

    let mut constraints = vec![Constraint::Length(5)];
    if !mux_lines.is_empty() {
        constraints.push(Constraint::Length(mux_height));
    }
    if !polarize_lines.is_empty() {
        constraints.push(Constraint::Length(polarize_height));
    }
    constraints.push(Constraint::Min(8));
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    draw_stat_strip(
        frame,
        rows[0],
        [
            (
                "Monitor pulse",
                vec![
                    format!("{} runs visible", app.runs.len()),
                    format!("{} active or stalled", app.active_run_count()),
                ],
                Color::Green,
            ),
            (
                "Selection",
                app.selected_run()
                    .map(|run| {
                        vec![
                            run.snapshot.run_id.clone(),
                            format!(
                                "{} / {}",
                                run.kind.label(),
                                run.snapshot.agent.as_deref().unwrap_or("unknown")
                            ),
                        ]
                    })
                    .unwrap_or_else(|| {
                        vec![
                            "No run selected".to_string(),
                            "Dispatch a worker to populate the board".to_string(),
                        ]
                    }),
                Color::Yellow,
            ),
            (
                "Filter",
                vec![
                    format!("{} scope", app.queue_scope.label()),
                    if app.search_query.is_empty() {
                        "f cycles live/history/all".to_string()
                    } else {
                        format!("/ {}", app.search_query)
                    },
                ],
                Color::Cyan,
            ),
        ],
    );

    let mut body_idx = 1;
    if !mux_lines.is_empty() {
        let state = app
            .mux_subscriber
            .as_ref()
            .and_then(|sub| sub.state.read().ok())
            .map(|s| s.clone());
        draw_mux_panel(
            frame,
            rows[body_idx],
            &mux_lines,
            app.mux_summaries.len(),
            state.as_ref(),
        );
        body_idx += 1;
    }
    if !polarize_lines.is_empty() {
        draw_polarize_panel(
            frame,
            rows[body_idx],
            &polarize_lines,
            app.polarize_intents.len(),
        );
        body_idx += 1;
    }

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
        .split(rows[body_idx]);

    draw_runs(frame, body[0], app, true);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(body[1]);
    draw_detail(frame, right[0], app, "Run dossier");
    draw_events(frame, right[1], app, "Recent timeline");
}

fn draw_mux_panel(
    frame: &mut Frame,
    area: Rect,
    lines: &[String],
    total_services: usize,
    state: Option<&crate::mux::SubscriberState>,
) {
    let any_unhealthy = lines.iter().any(|line| line.contains("! "));
    let title_text = match state {
        Some(crate::mux::SubscriberState::Connected) => {
            format!(" rust-mux ({total_services}) [Connected] ")
        }
        Some(crate::mux::SubscriberState::Reconnecting) => {
            format!(" rust-mux ({total_services}) [Reconnecting] ")
        }
        Some(crate::mux::SubscriberState::Polling) => {
            format!(" rust-mux ({total_services}) [Polling] ")
        }
        Some(crate::mux::SubscriberState::Failed) => {
            format!(" rust-mux ({total_services}) [Failed] ")
        }
        None => format!(" rust-mux ({total_services}) "),
    };
    let title_color = match state {
        Some(crate::mux::SubscriberState::Connected) => Color::Green,
        Some(crate::mux::SubscriberState::Reconnecting)
        | Some(crate::mux::SubscriberState::Polling) => Color::Yellow,
        Some(crate::mux::SubscriberState::Failed) => Color::Red,
        None => {
            if any_unhealthy {
                Color::Red
            } else {
                Color::Green
            }
        }
    };
    let block = Block::default()
        .title(Span::styled(
            title_text,
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);
    let body_lines: Vec<Line> = lines
        .iter()
        .map(|raw| {
            if let Some(rest) = raw.strip_prefix("  ! ") {
                Line::from(vec![
                    Span::styled("  ! ", Style::default().fg(Color::Red)),
                    Span::raw(rest.to_string()),
                ])
            } else if let Some(rest) = raw.strip_prefix("  • ") {
                Line::from(vec![
                    Span::styled("  • ", Style::default().fg(Color::Green)),
                    Span::raw(rest.to_string()),
                ])
            } else {
                Line::from(raw.clone())
            }
        })
        .collect();
    let para = Paragraph::new(body_lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}

fn draw_polarize_panel(frame: &mut Frame, area: Rect, lines: &[String], total_intents: usize) {
    let has_doctrine = lines.iter().any(|line| line.contains("doctrine"));
    let title_color = if has_doctrine {
        Color::Magenta
    } else {
        Color::Yellow
    };
    let title_text = format!(" polarize ({total_intents}) ");
    let block = Block::default()
        .title(Span::styled(
            title_text,
            Style::default()
                .fg(title_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);
    let body_lines: Vec<Line> = lines
        .iter()
        .map(|raw| {
            if raw.contains(" doctrine ") {
                Line::from(vec![
                    Span::styled("* ", Style::default().fg(Color::Magenta)),
                    Span::raw(raw.trim_start_matches("  * ").to_string()),
                ])
            } else if raw.contains(" pass ") {
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Green)),
                    Span::raw(raw.trim_start_matches("  > ").to_string()),
                ])
            } else {
                Line::from(raw.clone())
            }
        })
        .collect();
    let para = Paragraph::new(body_lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(para, area);
}

fn draw_dispatch(frame: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(12)])
        .split(area);

    draw_stat_strip(
        frame,
        rows[0],
        [
            (
                "Mission",
                vec![
                    app.launch_kind.human_title().to_string(),
                    app.launch_kind.human_description().to_string(),
                ],
                Color::Yellow,
            ),
            (
                "Operator",
                vec![
                    format!("agent {}", app.selected_agent()),
                    format!("runtime {}", app.launch_runtime.label()),
                ],
                Color::Blue,
            ),
            (
                "Prompt",
                vec![
                    if app.focus == LaunchFocus::EditPrompt {
                        "Editing live prompt".to_string()
                    } else {
                        "Ready to launch".to_string()
                    },
                    format!("{} chars staged", app.launch_prompt.chars().count()),
                ],
                Color::Magenta,
            ),
        ],
    );

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(rows[1]);

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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dispatch playbook"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(guide, right[0]);

    draw_launch_history(frame, right[1], app);
}

fn draw_controls(frame: &mut Frame, area: Rect, app: &App) {
    let actions = app.deep_actions();
    let selected_action = app
        .selected_deep_action()
        .map(|action| action.label())
        .unwrap_or_else(|| "No action primed".to_string());
    let artifact_count = actions
        .iter()
        .filter(|action| {
            matches!(
                action,
                crate::app::DeepAction::OpenReport(_)
                    | crate::app::DeepAction::OpenTranscript(_)
                    | crate::app::DeepAction::OpenRoot(_)
                    | crate::app::DeepAction::PolarizeIntent { .. }
            )
        })
        .count();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(12)])
        .split(area);

    draw_stat_strip(
        frame,
        rows[0],
        [
            (
                "Run access",
                app.selected_run()
                    .map(|run| vec![run.snapshot.run_id.clone(), run.snapshot.display_state()])
                    .unwrap_or_else(|| {
                        vec![
                            "No run selected".to_string(),
                            "Monitor chooses the source run".to_string(),
                        ]
                    }),
                Color::Yellow,
            ),
            (
                "Action deck",
                vec![
                    format!("{} actions available", actions.len()),
                    selected_action,
                ],
                Color::Cyan,
            ),
            (
                "Artifacts",
                vec![
                    format!("{artifact_count} file surfaces"),
                    "reports / transcripts / roots".to_string(),
                ],
                Color::Green,
            ),
        ],
    );

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
        .split(rows[1]);

    draw_deep_controls(frame, body[0], app);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(body[1]);

    draw_detail(frame, right[0], app, "Artifact access");
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
            "Monitor: ↑/↓ runs  / search  f scope  x archive  d controls  ? help"
        }
        (AppTab::Dispatch, LaunchFocus::EditPrompt) => {
            "Dispatch edit: type prompt  Enter newline  Ctrl+S/Esc save"
        }
        (_, LaunchFocus::Error) => "Error: Enter/Esc closes the failure details",
        (_, LaunchFocus::Artifact) => "Artifact viewer: Enter/Esc closes the native viewer",
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

    let shortcuts = "Global: q quit  r refresh  a cycle agent  v cycle runtime  y copy  Ctrl+L clear search  ? help";
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

fn draw_runs(frame: &mut Frame, area: Rect, app: &App, emphasize_live: bool) {
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

    let title = if emphasize_live && !app.search_query.is_empty() {
        format!("{} (/ {})", app.queue_scope.title(), app.search_query)
    } else if emphasize_live {
        app.queue_scope.title().to_string()
    } else {
        "Runs".to_string()
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
        "Dispatch deck (editing prompt)"
    } else {
        "Dispatch deck"
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
        .block(Block::default().borders(Borders::ALL).title("Launch trail"))
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Control actions"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(panel, area);
}

fn draw_stat_strip(frame: &mut Frame, area: Rect, cards: [(&str, Vec<String>, Color); 3]) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(area);

    for ((title, lines, accent), column) in cards.into_iter().zip(columns.iter().copied()) {
        let content = lines.into_iter().map(Line::from).collect::<Vec<_>>();
        let panel = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(accent)),
            )
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: false });
        frame.render_widget(panel, column);
    }
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

fn draw_search_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(64, 24, frame.area());
    frame.render_widget(Clear, area);
    let query = if app.search_query.is_empty() {
        "type to filter runs".to_string()
    } else {
        app.search_query.clone()
    };
    let lines = vec![
        Line::from(vec![
            Span::styled("/", Style::default().fg(Color::Yellow)),
            Span::raw(query),
        ]),
        Line::from(""),
        Line::from(format!(
            "{} runs visible in {} scope",
            app.runs.len(),
            app.queue_scope.label()
        )),
        Line::from("Enter/Esc closes. Ctrl+L clears search from browse."),
    ];
    let search = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Run search")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(search, area);
}

fn draw_prompt_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(76, 60, frame.area());
    frame.render_widget(Clear, area);
    let lines = app
        .prompt_edit_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let prompt = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Prompt editor")
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(prompt, area);
}

fn draw_error_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(76, 56, frame.area());
    frame.render_widget(Clear, area);
    let lines = app
        .error_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let error = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Launch error")
                .border_style(Style::default().fg(Color::Red)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(error, area);
}

fn draw_artifact_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(82, 72, frame.area());
    frame.render_widget(Clear, area);
    let lines = app
        .artifact_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    let artifact = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Artifact viewer")
                .border_style(Style::default().fg(Color::Green)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(artifact, area);
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

pub fn draw_client_drift_overlay(frame: &mut Frame, area: Rect, halt: &crate::launch::VerifyHalt) {
    let block = Block::default()
        .title(" Client Drift Detected ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red).bg(Color::Black));
    let area = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage(20),
            ratatui::layout::Constraint::Percentage(60),
            ratatui::layout::Constraint::Percentage(20),
        ])
        .split(area)[1];
    let area = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(10),
            ratatui::layout::Constraint::Percentage(80),
            ratatui::layout::Constraint::Percentage(10),
        ])
        .split(area)[1];

    let mut lines = vec![
        ratatui::text::Line::from(
            "Dispatch halted because client configuration does not route through rust-mux.",
        ),
        ratatui::text::Line::from(""),
    ];

    match halt {
        crate::launch::VerifyHalt::Drift(servers) => {
            lines.push(ratatui::text::Line::from("Non-mux servers found:"));
            for s in servers {
                lines.push(ratatui::text::Line::from(format!(
                    "  {} ({}:{})",
                    s.client, s.path, s.line
                )));
            }
        }
        crate::launch::VerifyHalt::Timeout => {
            lines.push(ratatui::text::Line::from(
                "Timeout waiting for verify response.",
            ));
        }
    }

    lines.push(ratatui::text::Line::from(""));
    lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(
        "Press F to auto-fix (spawns wizard)",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(ratatui::text::Line::from("Press Esc to cancel."));

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(ratatui::widgets::Clear, area);
    frame.render_widget(para, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{DispatchFocus, LaunchFocus, QueueScope};
    use crate::config::AppConfig;
    use crate::launch::{LaunchKind, LaunchRuntime};
    use crate::state::{ControlPlaneState, RenderedRun, RunKind, RunSnapshot};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use std::time::Duration;

    fn sample_run(run_id: &str, agent: &str, session: &str) -> RenderedRun {
        RenderedRun {
            snapshot: RunSnapshot {
                run_id: run_id.to_string(),
                session_id: Some(format!("sess-{run_id}")),
                agent: Some(agent.to_string()),
                skill: Some("workflow".to_string()),
                mode: Some("implement".to_string()),
                state: Some("running".to_string()),
                status: None,
                started_at: Some("2026-04-19T10:00:00Z".to_string()),
                updated_at: Some("2026-04-19T10:01:00Z".to_string()),
                last_heartbeat: Some("2026-04-19T10:01:30Z".to_string()),
                root: Some(format!("/tmp/{run_id}")),
                operator_session: Some(session.to_string()),
                latest_report: Some(format!("/tmp/{run_id}/report.md")),
                latest_transcript: Some(format!("/tmp/{run_id}/transcript.log")),
                last_error: None,
                extra: Default::default(),
            },
            kind: RunKind::Active,
            age_label: "just now".to_string(),
            recent_events: Vec::new(),
        }
    }

    fn sample_app() -> App {
        App {
            mux_subscriber: None,
            config: AppConfig {
                no_verify_gate: false,
                state_root: "/tmp/state".into(),
                command_deck: "/usr/bin/vibecrafted".into(),
                launch_root: "/tmp/repo".into(),
                launch_runtime: LaunchRuntime::Terminal,
                terminal_binary: "zellij".into(),
                tick_rate: Duration::from_millis(250),
            },
            state: ControlPlaneState::empty("/tmp/state"),
            runs: vec![
                sample_run("run-1", "codex", "operator-1"),
                sample_run("run-2", "claude", "operator-2"),
            ],
            selected: 0,
            active_tab: AppTab::Monitor.index(),
            launch_kind: LaunchKind::Workflow,
            launch_agent: 0,
            launch_prompt: "Ship the operator surface.".to_string(),
            launch_runtime: LaunchRuntime::Terminal,
            dispatch_selected: DispatchFocus::Kind as usize,
            focus: LaunchFocus::Browse,
            status_line: String::new(),
            launch_history: vec!["vc workflow --agent codex".to_string()],
            deep_selected: 0,
            queue_scope: QueueScope::Live,
            search_query: String::new(),
            error_title: String::new(),
            error_lines: Vec::new(),
            artifact_title: String::new(),
            artifact_lines: Vec::new(),
            mux_summaries: Vec::new(),
            polarize_intents: Vec::new(),
        }
    }

    fn render_to_string(app: &App) -> String {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| draw(frame, app)).unwrap();
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>()
    }

    #[test]
    fn monitor_tab_renders_monitor_surface() {
        let app = sample_app();
        let rendered = render_to_string(&app);

        assert!(rendered.contains("Monitor pulse"));
        assert!(rendered.contains("Live queue"));
        assert!(rendered.contains("Run dossier"));
        assert!(rendered.contains("Recent timeline"));
        assert!(!rendered.contains("Dispatch playbook"));
    }

    #[test]
    fn dispatch_tab_renders_dispatch_surface() {
        let mut app = sample_app();
        app.set_active_tab(AppTab::Dispatch);

        let rendered = render_to_string(&app);

        assert!(rendered.contains("Dispatch deck"));
        assert!(rendered.contains("Dispatch playbook"));
        assert!(rendered.contains("Launch trail"));
        assert!(!rendered.contains("Control actions"));
    }

    #[test]
    fn monitor_tab_renders_mux_panel_when_summaries_exist() {
        use crate::mux::{MuxStatusSnapshot, MuxSummary};
        use std::path::PathBuf;
        let healthy_json = r#"{
            "service_name": "general-memory",
            "server_status": "Running",
            "restarts": 0,
            "connected_clients": 2,
            "active_clients": 1,
            "max_active_clients": 5,
            "pending_requests": 0,
            "cached_initialize": true,
            "initializing": false,
            "queue_depth": 0,
            "child_pid": 4242,
            "max_request_bytes": 1048576,
            "restart_backoff_ms": 1000,
            "restart_backoff_max_ms": 30000,
            "max_restarts": 5
        }"#;
        let failed_json = r#"{
            "service_name": "brave-search",
            "server_status": {"Failed": "max restarts reached"},
            "restarts": 5,
            "connected_clients": 0,
            "active_clients": 0,
            "max_active_clients": 5,
            "pending_requests": 0,
            "cached_initialize": false,
            "initializing": false,
            "queue_depth": 0,
            "max_request_bytes": 1048576,
            "restart_backoff_ms": 1000,
            "restart_backoff_max_ms": 30000,
            "max_restarts": 5
        }"#;

        let mut app = sample_app();
        app.mux_summaries = vec![
            MuxSummary::from_path_and_result(
                PathBuf::from("/tmp/memory.json"),
                MuxStatusSnapshot::from_json(healthy_json),
            ),
            MuxSummary::from_path_and_result(
                PathBuf::from("/tmp/brave.json"),
                MuxStatusSnapshot::from_json(failed_json),
            ),
        ];

        let rendered = render_to_string(&app);

        assert!(
            rendered.contains("rust-mux"),
            "panel title must mark this as the rust-mux surface"
        );
        assert!(
            rendered.contains("(2)") || rendered.contains("(1/2 need attention)"),
            "panel must surface either total count or attention header: {rendered}"
        );
        assert!(
            rendered.contains("general-memory"),
            "healthy service must render verbatim"
        );
        assert!(
            rendered.contains("brave-search"),
            "failed service must render verbatim"
        );
        assert!(
            rendered.contains("Failed"),
            "failed status must surface in the panel"
        );
        // Existing Monitor sections must still be present underneath.
        assert!(rendered.contains("Run dossier"));
        assert!(rendered.contains("Recent timeline"));
    }

    #[test]
    fn monitor_tab_renders_polarize_intent_panel() {
        use crate::polarize::{PolarizeBand, PolarizeIntent};
        use std::path::PathBuf;

        let mut app = sample_app();
        app.polarize_intents = vec![PolarizeIntent {
            band: PolarizeBand::Doctrine,
            score: 14,
            run_id: "polr-123".to_string(),
            prism_path: PathBuf::from("/tmp/polarize/polr-123/prism.json"),
        }];

        let rendered = render_to_string(&app);

        assert!(rendered.contains("polarize"));
        assert!(rendered.contains("doctrine"));
        assert!(rendered.contains("score 14"));
        assert!(rendered.contains("polr-123"));
    }

    #[test]
    fn monitor_tab_skips_mux_panel_when_summaries_empty() {
        let app = sample_app();
        let rendered = render_to_string(&app);
        assert!(
            !rendered.contains("rust-mux"),
            "no panel should render when there are no mux summaries"
        );
    }

    #[test]
    fn controls_tab_renders_controls_surface() {
        let mut app = sample_app();
        app.set_active_tab(AppTab::Controls);

        let rendered = render_to_string(&app);

        assert!(rendered.contains("Action deck"));
        assert!(rendered.contains("Control actions"));
        assert!(rendered.contains("Artifact access"));
        assert!(rendered.contains("Selected timeline"));
        assert!(!rendered.contains("Dispatch playbook"));
    }
}
