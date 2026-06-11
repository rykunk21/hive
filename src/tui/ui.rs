use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::hive::Hive;

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

    draw_pod_registry(f, top[0]);
    draw_active_pods(f, top[1]);
    draw_activity_log(f, rows[1]);
}

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

fn draw_pod_registry(f: &mut Frame, area: Rect) {
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

fn draw_active_pods(f: &mut Frame, area: Rect) {
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

fn draw_activity_log(f: &mut Frame, area: Rect) {
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
