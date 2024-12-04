use crate::problems::Problem;


#[derive(Clone)]
pub struct Problem4 {}


impl Problem<u128> for Problem4 {
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use super::*;
    const P: Problem4 = Problem4 {};

    #[test]
    #[should_panic]
    fn test_part1() {
        assert_eq!(P.part1("", mpsc::channel().0), 0);
    }

    #[test]
    #[should_panic]
    fn test_part2() {
        assert_eq!(P.part2("", mpsc::channel().0), 0);
    }
}
