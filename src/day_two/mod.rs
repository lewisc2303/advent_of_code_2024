use std::fmt;

pub mod day_two {

    fn parse_report_into_vec(input: &str) -> Vec<u32> {
        input
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect()
    }

    fn read_reports(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .collect::<Vec<_>>()
            .iter()
            .map(|&s| parse_report_into_vec(s))
            .collect::<Vec<_>>()
    }

    #[derive(Clone, Copy)]
    enum Order {
        Increasing,
        Decreasing,
    }

    fn get_difference(prev: u32, curr: u32, direction: Order) -> Option<u32> {
        match direction {
            Order::Increasing => curr.checked_sub(prev),
            Order::Decreasing => prev.checked_sub(curr),
        }
    }
    fn is_safe(report: &[u32], use_dampener: bool) -> bool {
        match use_dampener {
            false => {
                find_unsafe_level(report, Order::Increasing).is_none()
                    || find_unsafe_level(report, Order::Decreasing).is_none()
            }
            true => {
                is_safe_with_dampener(report, Order::Increasing)
                    || is_safe_with_dampener(report, Order::Decreasing)
            }
        }
    }

    fn is_safe_with_dampener(report: &[u32], direction: Order) -> bool {
        let bad_level_index = find_unsafe_level(report, direction);
        if bad_level_index.is_none() {
            return true;
        }
        let bad_level_index = bad_level_index.unwrap();
        for possible_bad_level in
            bad_level_index.saturating_sub(1usize)..bad_level_index.saturating_add(1usize)
        {
            if try_remove_level(report, possible_bad_level, direction) {
                return true;
            }
        }
        false
    }

    fn try_remove_level(report: &[u32], level_to_remove: usize, direction: Order) -> bool {
        if level_to_remove >= report.len() {
            return false;
        }
        let mut report = report.to_vec();
        report.remove(level_to_remove);
        find_unsafe_level(&report, direction).is_none()
    }

    fn find_unsafe_level(report: &[u32], direction: Order) -> Option<usize> {
        let mut prev_level: Option<u32> = None;
        for (index, level) in report.iter().enumerate() {
            if prev_level.is_none() {
                prev_level = Some(*level);
                continue;
            }
            let difference = get_difference(prev_level.unwrap(), *level, direction);
            if difference.is_none_or(|x| !(1..=3).contains(&x)) {
                return Some(index);
            }
            prev_level = Some(*level);
        }
        None
    }

    pub fn part_one(input: &str) -> Option<u32> {
        let reports = read_reports(input);
        Some(reports.iter().filter(|&v| is_safe(v, false)).count() as u32)
    }

    pub fn part_two(input: &str) -> Option<u32> {
        let reports = read_reports(input);
        print!("{:?}", reports.iter().len());
        Some(reports.iter().filter(|&v| is_safe(v, true)).count() as u32)
    }
}

#[cfg(test)]
mod tests {

    use crate::day_two::day_two::{part_one, part_two};
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = "1 2 3
        1 2 3
        1 2 3";
        assert_eq!(part_two(&input), Some(3));
    }

    #[test]
    fn answer_part_one() {
        let input = fs::read_to_string("src/day_two/day_two_input.txt").unwrap();

        assert_eq!(part_one(&input), Some(660));
    }

    #[test]
    fn test_part_two() {
        let input = "1 2 7 8 9 
        9 7 6 2 1
        1 3 2 4 5";
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn answer_part_two() {
        let input = fs::read_to_string("src/day_two/day_two_input.txt").unwrap();
        println!("{}", input.len());
        assert_eq!(part_two(&input), Some(689));
    }
}
