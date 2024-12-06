use std::collections::VecDeque;
use crate::Event;
use crossterm::event;
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::{List, Paragraph};
use ratatui::{Frame, TerminalOptions, Viewport};
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::problems::common::Grid;

pub fn input_handling(tx: mpsc::Sender<Event>) {
    let tick_rate = Duration::from_millis(200);
    let new_tx = tx.clone();

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key) => new_tx.send(Event::Input(key)).unwrap(),
                    _ => {}
                };
            }
            if last_tick.elapsed() >= tick_rate {
                new_tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}

pub struct AppDisplayState {
    pub rows: VecDeque<Line<'static>>,
    pub part_1_result: Option<u128>,
    pub part_2_result: Option<u128>,
    pub grid: Option<Grid>
}

impl AppDisplayState {
    pub fn part_1_only(i: u128) -> Self {
        AppDisplayState {
            part_1_result: Some(i),
            part_2_result: None,
            rows: VecDeque::new(),
            grid: None
        }
    }

    pub fn part_2_only(i: u128) -> Self {
        AppDisplayState {
            part_1_result: None,
            part_2_result: Some(i),
            rows: VecDeque::new(),
            grid: None
        }
    }

    pub fn grid_update(g: Grid) -> Self {
        AppDisplayState {
            part_1_result: None,
            part_2_result: None,
            rows: VecDeque::new(),
            grid: Some(g)
        }
    }
}

pub fn run(rx: mpsc::Receiver<Event>) -> crate::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(150),
    });

    let mut redraw = true;

    let mut app_display_state = AppDisplayState {
        part_1_result: None,
        part_2_result: None,
        rows: VecDeque::new(),
        grid: None
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
                if ads.grid.is_some() {
                    app_display_state.grid = ads.grid;
                }
            }
            Event::NewRowEvent(line) => {
                app_display_state.rows.push_front(line);
                sleep(Duration::from_millis(5));
            }
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, app_display_state: &AppDisplayState) {
    match &app_display_state.grid {
        Some(g) => {
            let areas = Layout::vertical([Constraint::from(140), Constraint::from(2)]).split(frame.area());

            frame.render_widget(
                Paragraph::new(format!("{g:?}")),
                areas[0]
            );

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
        None => {
            let areas = Layout::vertical([Constraint::from(6), Constraint::from(2)]).split(frame.area());

            frame.render_widget(
                List::new(app_display_state.rows.clone()),
                areas[0]
            );

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
    }

}
