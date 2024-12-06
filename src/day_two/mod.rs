use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Order {
    Ascending,
    Descending,
    Initial,
    None,
    Invalid,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Order::Ascending => write!(f, "Ascending"),
            Order::Descending => write!(f, "Descending"),
            Order::Initial => write!(f, "Initial"),
            Order::Invalid => write!(f, "Invalid"),
            Order::None => write!(f, "None"),
        }
    }
}

pub mod day_two {
    use crate::day_two::Order;

    pub fn order(last: i32, current: i32) -> Order {
        if last < current {
            Order::Ascending
        } else if last > current {
            Order::Descending
        } else {
            Order::Invalid
        }
    }

    pub fn step_change_is_stable(last: i32, current: i32) -> bool {
        let step_change = (last - current).abs();
        step_change > 0 && step_change < 4
    }

    pub fn part_two(input: Vec<Vec<i32>>) -> i32 {
        input.iter().fold(0, |acc, record| {
            let record_result = record
                .iter()
                .fold((Order::Initial, 0), |acc, current| match acc {
                    (Order::Invalid, _) => (Order::Invalid, *current),
                    (Order::Initial, _) => (Order::None, *current),
                    (Order::None, last) => {
                        if !step_change_is_stable(last, *current) {
                            (Order::Invalid, *current)
                        } else {
                            (order(last, *current), *current)
                        }
                    }
                    (prev_order, last) => {
                        if !step_change_is_stable(last, *current) {
                            (Order::Invalid, *current)
                        } else if prev_order == order(last, *current) {
                            (prev_order, *current)
                        } else {
                            (Order::Invalid, *current)
                        }
                    }
                });

            match record_result {
                (Order::Invalid, _) => acc,
                _ => acc + 1,
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::day_two::day_two::{order, step_change_is_stable, part_two};
    use std::fs;

    #[test]
    fn test_order() {
        assert_eq!(order(1, 2), Order::Ascending);
        assert_eq!(order(2, 1), Order::Descending);
        assert_eq!(order(1, 1), Order::Invalid);
    }

    #[test]
    fn test_step_change_is_stable() {
        assert_eq!(step_change_is_stable(1, 2), true);
        assert_eq!(step_change_is_stable(1, 5), false);
    }

    #[test]
    fn test_part_two() {
        let input = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        assert_eq!(part_two(input), 3);
    }

    #[test]
    fn answer_part_two() {
        let input = fs::read_to_string("src/day_two/day_two_input.txt").unwrap();
        let input: Vec<Vec<i32>> = input
            .lines()
            .map(|line| line.split_whitespace().map(|x| x.parse().unwrap()).collect())
            .collect();
        assert_eq!(part_two(input), 660);
    }

}