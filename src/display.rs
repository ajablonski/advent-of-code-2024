use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread;
use crossterm::event;
use ratatui::{Frame, Terminal};
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;
use crate::Event;

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
    part_1_result: u128,
    part_2_result: u128,
}

pub fn run(terminal: &mut Terminal<impl Backend>, rx: mpsc::Receiver<Event>) -> crate::Result<()> {
    let mut redraw = true;

    let mut app_display_state = AppDisplayState {
        part_1_result: 0,
        part_2_result: 0,
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
            Event::UpdatePart1Result(i) => {
                app_display_state.part_1_result = i;
            }
            Event::UpdatePart2Result(i) => {
                app_display_state.part_2_result = i;
            }
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, app_display_state: &AppDisplayState) {
    let areas = Layout::vertical([Constraint::from(2), Constraint::from(2)]).split(frame.area());

    frame.render_widget(
        Paragraph::new(format!("Part 1: {}", app_display_state.part_1_result)),
        areas[0],
    );
    frame.render_widget(
        Paragraph::new(format!("Part 2: {}", app_display_state.part_2_result)),
        areas[1],
    );
}