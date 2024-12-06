use nom::{character::complete::*, combinator::*, multi::*, Finish, IResult};

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let (_, reports) = report_list(input)
        .finish()
        .expect("Failed to parse puzzle input");
    reports
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn report(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, number)(input)
}

fn report_list(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(line_ending, report)(input)
}

#[cfg(test)]
mod tests {
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
        let input = "1 2 3 4 5";
        let (rem, report) = report(input).finish().unwrap();

        assert_eq!(report, vec![1, 2, 3, 4, 5]);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_lists() {
        let input = "1 2 3 4 5
5 4 3 2 1";

        let (rem, reports) = report_list(input).finish().unwrap();

        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0], vec![1, 2, 3, 4, 5]);
        assert_eq!(reports[1], vec![5, 4, 3, 2, 1]);
        assert!(rem.is_empty());
    }
}
