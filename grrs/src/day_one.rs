
pub mod day_one{
    use std::fs;

    #[derive(Clone)]
    struct CoOrds {
        one: Vec<i32>,
        two: Vec<i32>
    }

    fn get_sorted_co_ords(co_ords: &mut CoOrds) ->  &mut CoOrds {
        let data = fs::read_to_string("/Users/lewis.coleman/advent_of_code_2024/grrs/src/day_1_input.txt").expect("Unable to read file");
    
        let new_co_cords = data.lines().fold(co_ords, |acc, line| {
            let mut line_split = line.split_whitespace();
            acc.one.append(& mut vec!(line_split.next().unwrap().trim().parse::<i32>().unwrap()));
            acc.two.append(& mut vec!(line_split.next().unwrap().trim().parse::<i32>().unwrap()));
            acc
        });

        new_co_cords.one.sort_by(|a, b| a.cmp(b));
        new_co_cords.two.sort_by(|a, b| a.cmp(b));

        new_co_cords
    }


    #[allow(dead_code)]
    pub fn part_one() -> i32 {
        let mut co_ords = CoOrds {
            one: Vec::new(),
            two: Vec::new()
        };

        let sorted_co_ords = get_sorted_co_ords(&mut co_ords);

        let result = sorted_co_ords.one.iter().zip(sorted_co_ords.two.clone()).fold(0, |acc, (one, two)| {
            let x = one - two;
            x.abs() + acc
        });

        result
    }

    #[allow(dead_code)]
    pub fn part_two() -> i32 {
        let mut co_ords = CoOrds {
            one: Vec::new(),
            two: Vec::new()
        };

        let sorted_co_ords = get_sorted_co_ords(&mut co_ords);
        let similarity_score: Vec<(i32, i32)> = sorted_co_ords.one.iter().map(|&x| {
            let count = sorted_co_ords.two.iter().filter(|&&y| y == x).count() as i32;
            (x, count)
        }).collect();

        similarity_score.iter().map(|(x, y)| {x * y}).sum()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one_part_one() {
        assert_eq!(day_one::part_one(), 1258579);
    }

    #[test]
    fn test_day_one_part_two() {
        assert_eq!(day_one::part_two(), 23981443);
    }
}
