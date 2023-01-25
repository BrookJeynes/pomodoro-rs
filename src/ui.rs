use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use figlet_rs::FIGfont;

use crate::{AppState, StudyMode};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState) {
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

    // Todo: convert String to &str
    let create_watermark = |text: String| {
        Paragraph::new(text)
            .alignment(Alignment::Right)
            .style(Style::default().add_modifier(Modifier::ITALIC))
    };

    match app_state.study_mode {
        StudyMode::Normal => {
            let top = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(size);

            // Render border around top
            // Todo: Create as_str() implementation for pomodoro_mode
            f.render_widget(
                create_block(app_state.pomodoro_mode.to_string().as_str()),
                top[0],
            );

            let inner_top = Layout::default()
                .margin(1)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Percentage(30),
                    Constraint::Percentage(35),
                ])
                .split(top[0]);

            let watermark = create_watermark(String::from("Made by Chooky <3"));
            f.render_widget(watermark, inner_top[0]);

            let timer = create_timer(&app_state);
            f.render_widget(timer, inner_top[1]);

            let bottom = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(top[1]);

            f.render_widget(create_block("Tasks"), bottom[0]);
            f.render_widget(create_block("Controls"), bottom[1]);
        }
        StudyMode::Zen => {
            let top = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(100)])
                .split(size);

            // Render border around top
            // Todo: Create as_str() implementation for pomodoro_mode
            f.render_widget(
                create_block(app_state.pomodoro_mode.to_string().as_str()),
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

            let watermark = create_watermark(String::from("Made by Chooky <3"));
            f.render_widget(watermark, inner_top[0]);

            let timer = create_timer(&app_state);
            f.render_widget(timer, inner_top[1]);
        }
    }
}

fn create_timer(app_state: &AppState) -> Paragraph {
    let mut timer_text = render_ascii_text(app_state.timer.mm_ss().as_str());
    timer_text.push_str("Keep it up, you got this!");

    Paragraph::new(timer_text).alignment(Alignment::Center)
}

fn render_ascii_text(text: &str) -> String {
    let standard_font = FIGfont::standard().unwrap();

    match standard_font.convert(text) {
        Some(val) => val.to_string(),
        None => String::new(),
    }
}
