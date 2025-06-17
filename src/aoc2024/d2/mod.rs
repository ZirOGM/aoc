use std::rc::Rc;

pub fn execute(){
    let input: &str = include_str!("input.txt");
    one(input);
    two(input);
}

fn one(input: &str) {
    let safe_reports: usize = input.lines()
        .map(|x| parse_input(x))
        .filter(|report| is_safe(&report))
        .count();
    println!("{:?}", safe_reports);
}

fn two(input: &str) {
    let safe_reports: usize = input.lines()
        .map(|x| parse_input(x))
        .filter(|report| is_safe(&report) || dampener_check(report))
        .count();
    println!("{:?}", safe_reports);
}

fn parse_input(line: &str) -> Rc<[isize]>{
   line.split(' ').map(|x| x.parse().unwrap()).collect::<Rc<[isize]>>()
}

fn is_safe(report: &Rc<[isize]>) -> bool {
    let diff = report[0] - report[1];
    let direction = calculate_direction(diff);
    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];
        if diff.abs() == 0 || diff.abs() > 3 || isize::from(direction) * diff < 1 { 
            return false
        }
    }
    true
}

fn calculate_direction(diff: isize) -> i8 {
    if diff > 0 { 1_i8 } else if diff < 0 { -1_i8 } else { 0_i8 }
}

fn dampener_check(report: &Rc<[isize]>) -> bool {
    for i in 0..report.len() {
        let sub_report = report.iter().enumerate()
            .filter(|(index, _)| *index != i).map(|(_, x)| *x)
            .collect::<Rc<[isize]>>();
        if is_safe(&sub_report) {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "1 2 3 4 5";
        let report = parse_input(input);
        assert_eq!(is_safe(&report), true);
    }
    #[test]
    fn test_two() {
        let input = "44 4 7 10 13";
        let report = parse_input(input);
        assert_eq!(dampener_check(&report), true);
    }
}
