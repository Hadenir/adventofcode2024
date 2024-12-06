use nom::{character::complete::*, combinator::*, multi::*, sequence::*, Finish, IResult};

pub fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (_, lists) = lists(input).finish().expect("Failed to parse puzzle input");
    lists
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn number_pair(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(number, space1, number)(input)
}

fn lists(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    map(separated_list0(line_ending, number_pair), |list| {
        list.into_iter().collect()
    })(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_parse_number() {
        let input = "12";
        let (rem, num) = number(input).finish().unwrap();

        assert_eq!(num, 12);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_number_pair() {
        let input = "4   3";
        let (rem, nums) = number_pair(input).finish().unwrap();

        assert_eq!(nums, (4, 3));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_lists() {
        let input = "1   2
3   4";

        let (rem, lists) = lists(input).finish().unwrap();

        assert_eq!(lists, (vec![1, 3], vec![2, 4]));
        assert!(rem.is_empty());
    }
}
