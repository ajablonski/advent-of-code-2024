use crate::Event;
use crossterm::event;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;
use ratatui::{Frame, Terminal, TerminalOptions, Viewport};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn input_handling(tx: mpsc::Sender<Event>) {
    let tick_rate = Duration::from_millis(200);

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
                    _ => {}
                };
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}

pub struct AppDisplayState {
    pub part_1_result: Option<u128>,
    pub part_2_result: Option<u128>,
}

impl AppDisplayState {
    pub fn part_1_only(i: u128) -> Self {
        AppDisplayState {
            part_1_result: Some(i),
            part_2_result: None,
        }
    }

    pub fn part_2_only(i: u128) -> Self {
        AppDisplayState {
            part_1_result: None,
            part_2_result: Some(i),
        }
    }
}

pub fn run(rx: mpsc::Receiver<Event>) -> crate::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(8),
    });

    let mut redraw = true;

    let mut app_display_state = AppDisplayState {
        part_1_result: None,
        part_2_result: None,
    };

    loop {
        if redraw {
            terminal.draw(|frame| draw(frame, &app_display_state))?;
        }
        redraw = true;

        match rx.recv()? {
            Event::Input(event) => {
                if event.code == event::KeyCode::Char('q') {
                    break;
                }
            }
            Event::Tick => {}
            Event::UpdateAppDisplayState(ads) => {
                if ads.part_1_result.is_some() {
                    app_display_state.part_1_result = ads.part_1_result
                }
                if ads.part_2_result.is_some() {
                    app_display_state.part_2_result = ads.part_2_result
                }
            }
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, app_display_state: &AppDisplayState) {
    let areas = Layout::vertical([Constraint::from(6), Constraint::from(2)]).split(frame.area());


    frame.render_widget(
        Paragraph::new(format!(
            "\
            Part 1: {}\n\
            Part 2: {}",
            app_display_state.part_1_result.unwrap_or(0),
            app_display_state.part_2_result.unwrap_or(0)
        )),
        areas[1],
    );
}
