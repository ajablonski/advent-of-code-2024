use crate::problems::problem1::Problem1;
use crate::problems::problem10::Problem10;
use crate::problems::problem11::Problem11;
use crate::problems::problem12::Problem12;
use crate::problems::problem13::Problem13;
use crate::problems::problem14::Problem14;
use crate::problems::problem2::Problem2;
use crate::problems::problem3::Problem3;
use crate::problems::problem4::Problem4;
use crate::problems::problem5::Problem5;
use crate::problems::problem6::Problem6;
use crate::problems::problem7::Problem7;
use crate::problems::problem8::Problem8;
use crate::problems::problem9::Problem9;
use crate::Event;
use std::sync::mpsc::Sender;

pub mod common;
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

pub trait Problem<T> {
    fn part1(&self, _input: &str) -> T {
        todo!()
    }

    fn part2(&self, _input: &str) -> T {
        todo!()
    }
}

pub fn get_all_problems(tx: &Sender<Event>) -> Vec<Box<dyn Problem<u128>>> {
    vec![
        Box::new(Problem1 {}),
        Box::new(Problem2::new(tx)),
        Box::new(Problem3::new(tx)),
        Box::new(Problem4 {}),
        Box::new(Problem5::new(tx)),
        Box::new(Problem6::new(tx)),
        Box::new(Problem7 {} ),
        Box::new(Problem8 {} ),
        Box::new(Problem9 {} ),
        Box::new(Problem10 {} ),
        Box::new(Problem11 {} ),
        Box::new(Problem12 {} ),
        Box::new(Problem13 {} ),
        Box::new(Problem14 {tx: tx.clone()} ),
    ]
}
