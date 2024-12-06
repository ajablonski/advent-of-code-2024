use crate::problems::Problem;
use crate::Event;
use std::cmp::Ordering;
use std::sync::mpsc::Sender;
use itertools::Itertools;

pub struct Problem5 {
    _tx: Sender<Event>,
}

impl Problem5 {
    fn parse(data: &str) -> (Vec<Rule>, Vec<Update>) {
        if let Some((l, r)) = data.split_once("\n\n") {
            (
                l.lines()
                    .filter_map(|l| match l.split_once("|") {
                        Some((l, r)) => Some(Rule(l.parse().unwrap(), r.parse().unwrap())),
                        None => None,
                    })
                    .collect(),
                r.lines()
                    .map(|l| Update(l.split(",").map(|ui| ui.parse::<u32>().unwrap()).collect()))
                    .collect(),
            )
        } else {
            (vec![], vec![])
        }
    }
}

impl Problem<u128> for Problem5 {
    fn part1(&self, input: &str) -> u128 {
        let (rules, updates) = Self::parse(input.trim());

        Problem5::sort_updates(&updates, &rules)
            .iter()
            .filter_map(|(unsorted, sorted)| {
                if *unsorted == sorted {
                    Some(unsorted)
                } else {
                    None
                }
            })
            .map(|u| u.0[u.0.len() / 2] as u128)
            .sum()
    }

    fn part2(&self, input: &str) -> u128 {
        let (rules, updates) = Self::parse(input.trim());

        Problem5::sort_updates(&updates, &rules)
            .iter()
            .filter_map(|(unsorted, sorted)| {
                if *unsorted == sorted {
                    None
                } else {
                    Some(sorted)
                }
            })
            .map(|u| u.0[u.0.len() / 2] as u128)
            .sum()
    }
}

impl Problem5 {
    pub fn new(tx: &Sender<Event>) -> Self {
        Problem5 { _tx: tx.clone() }
    }

    fn sort_updates<'a>(
        updates: &'a Vec<Update>,
        rules: &'a Vec<Rule>,
    ) -> Vec<(&'a Update, Update)> {
        updates
            .iter()
            .map(|u| {
                let new_vec = u.0.to_vec()
                    .into_iter()
                    .sorted_by(|&l, &r| {
                        let applicable_rule = rules
                            .iter()
                            .find(|&rule| (rule.0 == l && rule.1 == r) || (rule.1 == l && rule.0 == r));

                        match applicable_rule {
                            Some(Rule(rule_l, _)) if *rule_l == l => Ordering::Less,
                            Some(Rule(rule_l, _)) if *rule_l == r => Ordering::Greater,
                            _ => Ordering::Equal,
                        }
                    })
                    .collect();

                (u, Update(new_vec))
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct Rule(u32, u32);

#[derive(Debug, PartialEq)]

struct Update(Vec<u32>);

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn load_sample_data() -> &'static str {
        include_str!("../../../sample_data/5.txt")
    }

    mod parser {
        use super::*;

        #[test]
        fn should_correctly_parse_input() {
            let data = "\
            1|2\n\
            3|4\n\
            5|6\n\
            \n\
            1,7,8\n\
            2,9,8\n\
            3,10,2";

            let (rules, updates) = Problem5::parse(data);

            assert_eq!(rules, vec![Rule(1, 2), Rule(3, 4), Rule(5, 6)]);
            assert_eq!(
                updates,
                vec![
                    Update(vec![1, 7, 8]),
                    Update(vec![2, 9, 8]),
                    Update(vec![3, 10, 2])
                ]
            );
        }
    }

    #[test]
    fn should_produce_correct_answer_for_part_1() {
        let p: Problem5 = Problem5::new(&mpsc::channel().0);

        assert_eq!(p.part1(load_sample_data()), 143);
    }

    #[test]
    fn should_produce_correct_answer_for_part_2() {
        let p: Problem5 = Problem5::new(&mpsc::channel().0);

        assert_eq!(p.part2(load_sample_data()), 123);
    }
}
