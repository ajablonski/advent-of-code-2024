use crate::display::AppDisplayState;
use crate::problems::get_all_problems;
use chrono::{TimeZone, Utc};
use crossterm::event;
use ratatui::text::Line;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;

mod problems;

mod display;

const MAX_PROBLEM: u32 = 25;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn solve(problem_number: usize) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let all_problems = get_all_problems(&tx);
    if let Some(problem) = all_problems.get(problem_number - 1) {
        let input = std::fs::read_to_string(format!("data/{problem_number}.txt")).unwrap();

        display::input_handling(tx.clone());

        let part_1_result = (**problem).part1(input.as_str());
        let part_2_result = (**problem).part2(input.as_str());

        tx.send(Event::UpdateAppDisplayState(AppDisplayState {
            part_1_result: Some(part_1_result),
            part_2_result: Some(part_2_result),
            rows: VecDeque::new(),
            grid: None
        })).unwrap();
        tx.send(Event::Input(event::KeyCode::Char('q').into())).unwrap();

        let app_result = display::run(rx);

        ratatui::restore();

        app_result
    } else {
        println!("No solution found for {}", problem_number);

        Ok(())
    }
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
    UpdateAppDisplayState(AppDisplayState),
    NewRowEvent(Line<'static>)
}
