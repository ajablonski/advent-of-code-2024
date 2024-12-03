use crate::problem1::Problem1;
use crate::problem2::Problem2;
use crate::problem3::Problem3;
use crate::problem4::Problem4;
use chrono::{TimeZone, Utc};
use crossterm::event;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Paragraph;
use ratatui::{Frame, Terminal, TerminalOptions, Viewport};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

pub trait Problem<T> {
    fn part1(&self, _input: &str, _tx: Sender<Event>) -> T {
        todo!()
    }

    fn part2(&self, _input: &str, _tx: Sender<Event>) -> T {
        todo!()
    }
}

pub mod problem1;
pub mod problem10;
pub mod problem11;
pub mod problem12;
pub mod problem13;
pub mod problem14;
pub mod problem15;
pub mod problem16;
pub mod problem17;
pub mod problem18;
pub mod problem19;
pub mod problem2;
pub mod problem20;
pub mod problem21;
pub mod problem22;
pub mod problem23;
pub mod problem24;
pub mod problem25;
pub mod problem3;
pub mod problem4;
pub mod problem5;
pub mod problem6;
pub mod problem7;
pub mod problem8;
pub mod problem9;

const MAX_PROBLEM: u32 = 25;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn solve(problem_number: usize) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut problem_lookup: Vec<Box<dyn Problem<u128> + Send>> = vec![
        Box::new(Problem1 {}),
        Box::new(Problem2::new()),
        Box::new(Problem3 {}),
        Box::new(Problem4 {}),
    ];

    let problem = problem_lookup.get_mut(problem_number - 1).unwrap();
    let input = std::fs::read_to_string(format!("data/{problem_number}.txt")).unwrap();

    input_handling(tx.clone());

    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(8),
    });

    let part_1_result = (**problem).part1(input.as_str(), tx.clone());
    let part_2_result = (**problem).part2(input.as_str(), tx.clone());

    tx.send(Event::UpdatePart1Result(part_1_result)).unwrap();
    tx.send(Event::UpdatePart2Result(part_2_result)).unwrap();

    let app_result = run(&mut terminal, rx);

    ratatui::restore();

    app_result
}

pub fn fetch_data() {
    let session_token = std::fs::read_to_string("session_id_file.txt")
        .or(std::env::var("SESSION_TOKEN"))
        .unwrap();

    for problem_number in 1..=MAX_PROBLEM {
        let now = Utc::now();
        let problem_ready_time = Utc
            .with_ymd_and_hms(2024, 12, problem_number, 5, 0, 0)
            .unwrap();
        if now >= problem_ready_time {
            println!("Fetching data for problem {problem_number}...");
            if let Ok(body) = reqwest::blocking::Client::new()
                .get(format!(
                    "https://adventofcode.com/2024/day/{problem_number}/input"
                ))
                .header("Cookie", format!("session={session_token}"))
                .send()
            {
                let t = body.text();
                let file = File::create(format!("data/{problem_number}.txt"));

                match (t, file) {
                    (Ok(tt), Ok(mut f)) => {
                        let _ = f.write_all(tt.as_bytes());
                    }
                    (Err(e), _) => println!("Error in AOC Response: {e}"),
                    (Ok(_), Err(e)) => println!("Error in file opening: {e}"),
                }
            }
        } else {
            println!(
                "Data not yet available for problem {problem_number}. Current time is {now}, will be ready at {problem_ready_time}"
            )
        }
    }
}

pub enum Event {
    Tick,
    Input(event::KeyEvent),
    UpdatePart1Result(u128),
    UpdatePart2Result(u128),
}

fn input_handling(tx: mpsc::Sender<Event>) {
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

struct AppDisplayState {
    part_1_result: u128,
    part_2_result: u128,
}

fn run(terminal: &mut Terminal<impl Backend>, rx: mpsc::Receiver<Event>) -> Result<()> {
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
