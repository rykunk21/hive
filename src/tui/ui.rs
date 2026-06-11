//! TUI rendering — draws the hive dashboard.
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::canvas::{Canvas, Circle, Line as CanvasLine},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::tui::Tui;
use crate::{hive::Hive, tui::TuiState};

pub fn draw(f: &mut Frame, hive: &Hive, tui: &Tui) {
    let area = f.area();

    // Split vertically: main content (top) | input bar (bottom, fixed height)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    // Split main content horizontally: agent status (left) | hive graph (right)
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(rows[0]);

    draw_agent_status(f, columns[0], hive);
    draw_hive_graph(f, columns[1], hive);
    draw_input_bar(f, rows[1], &tui.bar_input);

    match tui.state {
        //spawn
        TuiState::Spawn => draw_spawn_overlay(f, f.area()),
        _ => {}
    }
}

fn draw_spawn_overlay(f: &mut Frame, area: Rect) {
    // Center a box over the full terminal area
    let popup = centered_rect(50, 40, area);

    f.render_widget(Clear, popup); // wipe what's underneath
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(Span::styled(
                " spawn pod ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
        popup,
    );
}

/// Returns a centered Rect of `percent_x` width and `percent_y` height.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(vertical[1])[1]
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

/// Left panel: one row per agent with status indicator and metadata.
///
/// TODO: Query hive.agents() for real agent snapshots.
fn draw_agent_status(f: &mut Frame, area: Rect, _hive: &Hive) {
    // Each agent gets: status dot | name | current task (dimmed)
    let items: Vec<ListItem> = vec![
        agent_item("planner-0", "routing goal", AgentState::Running),
        agent_item("planner-1", "idle", AgentState::Idle),
        agent_item("retriever-0", "fetch context chunk", AgentState::Running),
        agent_item("summarizer-0", "—", AgentState::Error),
    ];

    f.render_widget(List::new(items).block(border("agents")), area);
}

enum AgentState {
    Running,
    Idle,
    Error,
}

fn agent_item(name: &str, task: &str, state: AgentState) -> ListItem<'static> {
    let (dot, color) = match state {
        AgentState::Running => ("● ", Color::Green),
        AgentState::Idle => ("○ ", Color::DarkGray),
        AgentState::Error => ("✖ ", Color::Red),
    };
    ListItem::new(Line::from(vec![
        Span::styled(format!("  {dot}"), Style::default().fg(color)),
        Span::styled(name.to_string(), Style::default().fg(Color::White)),
        Span::raw("  "),
        Span::styled(task.to_string(), Style::default().fg(Color::DarkGray)),
    ]))
}

/// Right panel: Canvas-based graph overlay of the hive topology.
///
/// Nodes are agents; edges represent message channels between them.
/// Coordinates are in a logical [0, 100] space — Canvas scales to fit.
///
/// TODO: Derive positions and edges from hive.topology().
fn draw_hive_graph(f: &mut Frame, area: Rect, _hive: &Hive) {
    // Hardcoded node positions in logical [0,100] space.
    let nodes: &[(&str, f64, f64, Color)] = &[
        ("planner-0", 50.0, 75.0, Color::Green),
        ("planner-1", 30.0, 75.0, Color::DarkGray),
        ("retriever-0", 70.0, 50.0, Color::Green),
        ("summarizer-0", 50.0, 25.0, Color::Red),
    ];

    // Edges as (from_index, to_index)
    let edges: &[(usize, usize)] = &[(0, 2), (1, 2), (2, 3)];

    let canvas = Canvas::default()
        .block(border("hive"))
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Draw edges first so nodes render on top
            for &(a, b) in edges {
                let (_, x1, y1, _) = nodes[a];
                let (_, x2, y2, _) = nodes[b];
                ctx.draw(&CanvasLine {
                    x1,
                    y1,
                    x2,
                    y2,
                    color: Color::DarkGray,
                });
            }

            // Draw nodes
            for &(label, x, y, color) in nodes {
                ctx.draw(&Circle {
                    x,
                    y,
                    radius: 2.0,
                    color,
                });
                // Labels sit slightly above each node
                ctx.print(
                    x - 4.0,
                    y + 4.0,
                    Span::styled(
                        label,
                        Style::default().fg(color).add_modifier(Modifier::BOLD),
                    ),
                );
            }
        });

    f.render_widget(canvas, area);
}

/// Bottom bar: single-line text input with a prompt glyph.
///
/// `input` is the current contents of the input buffer held by app state.
fn draw_input_bar(f: &mut Frame, area: Rect, input: &str) {
    let text = Line::from(vec![
        Span::styled(
            " ❯ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(input),
        Span::styled("█", Style::default().fg(Color::Yellow)), // fake cursor
    ]);

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(paragraph, area);
}
