//! TUI rendering — draws the hive dashboard.
//!
//! The UI is split into three panels: pod registry (left), active pods (right),
/// and activity log (bottom). All data is currently hardcoded; once the hive
/// exposes real state, this module will query it.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::hive::Hive;

/// Draw the full TUI dashboard.
///
/// Layout: top 70% split horizontally (pod registry | active pods),
/// bottom 30% activity log.
///
/// TODO: Replace all hardcoded data with real hive state queries.
pub fn draw(f: &mut Frame, hive: &Hive) {
    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(rows[0]);

    draw_pod_registry(f, top[0], hive);
    draw_active_pods(f, top[1], hive);
    draw_activity_log(f, rows[1], hive);
}

/// Draw a bordered block with a styled title.
fn border(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(Span::styled(
            format!(" {title} "),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
}

/// Draw the pod registry panel (available pod types).
///
/// TODO: Query hive.pod_types() instead of hardcoded list.
fn draw_pod_registry(f: &mut Frame, area: Rect, _hive: &Hive) {
    // TODO: Replace with real pod type list from hive.
    let items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("  ◆ ", Style::default().fg(Color::Cyan)),
            Span::raw("planner"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  ◆ ", Style::default().fg(Color::Cyan)),
            Span::raw("summarizer"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  ◆ ", Style::default().fg(Color::Cyan)),
            Span::raw("retriever"),
        ])),
    ];

    let list = List::new(items).block(border("pod registry"));
    f.render_widget(list, area);
}

/// Draw the active pods panel (currently running pods).
///
/// TODO: Query hive.active_pods() instead of hardcoded list.
fn draw_active_pods(f: &mut Frame, area: Rect, _hive: &Hive) {
    // TODO: Replace with real active pod snapshots from hive.
    let items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("  ● ", Style::default().fg(Color::Green)),
            Span::raw("planner"),
            Span::styled("  ×2", Style::default().fg(Color::DarkGray)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("  ● ", Style::default().fg(Color::Green)),
            Span::raw("retriever"),
            Span::styled("  ×1", Style::default().fg(Color::DarkGray)),
        ])),
    ];

    let list = List::new(items).block(border("active pods"));
    f.render_widget(list, area);
}

/// Draw the activity log panel (recent hive events).
///
/// TODO: Query hive.activity_log() instead of hardcoded lines.
fn draw_activity_log(f: &mut Frame, area: Rect, _hive: &Hive) {
    // TODO: Replace with real activity events from hive.
    let logs = vec![
        Line::from(vec![
            Span::styled("  › ", Style::default().fg(Color::DarkGray)),
            Span::raw("hive started"),
        ]),
        Line::from(vec![
            Span::styled("  › ", Style::default().fg(Color::DarkGray)),
            Span::raw("planner spawned"),
        ]),
        Line::from(vec![
            Span::styled("  › ", Style::default().fg(Color::DarkGray)),
            Span::raw("planner completed: plan route to goal"),
        ]),
    ];

    let paragraph = Paragraph::new(logs).block(border("activity"));
    f.render_widget(paragraph, area);
}
