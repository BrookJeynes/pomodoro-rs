use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use figlet_rs::FIGfont;

use crate::{AppState, StudyMode};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();

    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    };

    let create_gauge = || {
        Gauge::default()
            .block(create_block(""))
            .gauge_style(Style::default().fg(Color::White))
            .percent(app_state.timer.percentage)
    };

    let create_timer = || {
        let mut timer_text = render_ascii_text(
            // 60 * 60 = 60 minutes in seconds
            if app_state.timer.time_remaining.as_secs() >= (60 * 60) {
                app_state.timer.hh_mm_ss()
            } else {
                app_state.timer.mm_ss()
            }
            .as_str(),
        );
        timer_text.push_str("Keep it up, you got this!");

        Paragraph::new(timer_text).alignment(Alignment::Center)
    };

    let create_watermark = |text: &str| {
        Paragraph::new(text.to_string())
            .alignment(Alignment::Right)
            .style(Style::default().add_modifier(Modifier::ITALIC))
    };

    let create_control_text = |control: &str, action: &str| {
        vec![
            Span::styled(
                format!("{}: ", control.to_string()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::from(action.to_string()),
        ]
    };

    // Todo: Clean this up, possible a Help struct
    let controls = Paragraph::new(vec![
        Spans::from(Span::styled(
            "Timer:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
        Spans::from(create_control_text("p", "Pomodoro timer")),
        Spans::from(create_control_text("s", "Short break timer")),
        Spans::from(create_control_text("l", "Long break timer")),
        Spans::from(create_control_text("Space", "Pause/Unpause timer")),
        Spans::from(create_control_text("r", "Reset timer")),
        Spans::from(""),
        Spans::from(Span::styled(
            "Tasks:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
        Spans::from(create_control_text("j/k", "Scroll task list")),
        Spans::from(create_control_text("S", "Save tasks")),
        Spans::from(create_control_text("Enter", "Mark/Unmark task as complete")),
        Spans::from(create_control_text(
            "+/-",
            "Increase/Decrease pomodoros taken for task",
        )),
        Spans::from(""),
        Spans::from(Span::styled(
            "Misc:",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )),
        Spans::from(create_control_text("q", "Quit application")),
    ])
    .wrap(Wrap { trim: false })
    .block(create_block("Controls"));

    match app_state.study_mode {
        StudyMode::Normal => {
            let top = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(size);

            // Todo: Create as_str() implementation for pomodoro_mode
            f.render_widget(
                create_block(
                    format!(
                        "{} - Press ? for help",
                        app_state.timer.pomodoro_mode.to_string()
                    )
                    .as_str(),
                ),
                top[0],
            );

            let inner_top = Layout::default()
                .margin(1)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Percentage(40),
                    Constraint::Percentage(35),
                ])
                .split(top[0]);

            let watermark = create_watermark("Made by Chooky <3");
            f.render_widget(watermark, inner_top[0]);

            let timer = create_timer();
            f.render_widget(timer, inner_top[1]);

            let bottom = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)])
                .split(top[1]);

            let tasks: Vec<ListItem> = app_state
                .tasks
                .items
                .iter()
                .map(|task| ListItem::new(task.list_print()))
                .collect();

            let tasks_list = List::new(tasks)
                .block(create_block("Tasks"))
                .highlight_style(Style::default().fg(Color::LightGreen))
                .start_corner(Corner::TopLeft);

            f.render_stateful_widget(tasks_list, bottom[0], &mut app_state.tasks.state);

            if app_state.show_help_menu {
                // Todo: conditionally render the size of this popup
                let area = centered_rect(60, 70, size);
                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(controls, area);
            }
        }
        StudyMode::Zen => {
            let top = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            // Todo: Create as_str() implementation for pomodoro_mode
            f.render_widget(
                create_block(
                    format!(
                        "{} - Press ? for help",
                        app_state.timer.pomodoro_mode.to_string()
                    )
                    .as_str(),
                ),
                top[0],
            );

            let inner_top = Layout::default()
                .margin(1)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(5),
                ])
                .split(top[0]);

            let gauge = create_gauge();
            f.render_widget(gauge, inner_top[2]);

            let watermark = create_watermark("Made by Chooky <3");
            f.render_widget(watermark, inner_top[0]);

            let timer = create_timer();
            f.render_widget(timer, inner_top[1]);

            // Todo: Code duplication - find a way to move this logic
            if app_state.show_help_menu {
                // Todo: conditionally render the size of this popup
                let area = centered_rect(60, 70, size);
                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(controls, area);
            }
        }
    }
}

fn render_ascii_text(text: &str) -> String {
    let standard_font = FIGfont::standard().unwrap();

    match standard_font.convert(text) {
        Some(val) => val.to_string(),
        None => String::new(),
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
