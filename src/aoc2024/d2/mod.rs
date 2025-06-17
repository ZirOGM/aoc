use std::rc::Rc;

pub fn execute(){
    let input: &str = include_str!("input.txt");
    let reports: usize = input.lines()
        .map(|x| parse_input(x))
        .map(|report| is_safe(&report, 0, 0))
        .reduce(|a, b| {a+b}).unwrap();
    println!("{:?}", reports);
}

fn parse_input(line: &str) -> Rc<[isize]>{
   line.split(' ').map(|x| x.parse().unwrap()).collect::<Rc<[isize]>>()
}

fn is_safe(report: &Rc<[isize]>, index: usize, dir: i8) -> usize {
    if index + 2 >= report.len() {
        return 1_usize
    }
    let current = report.get(index).unwrap();
    let next = report.get(index + 1).unwrap();
    let diff = next - current;
    if diff > 0 && diff < 4 && dir >= 0{
        is_safe(report, index + 1, 1_i8)
    }else if diff < 0 && diff > -4 && dir <= 0{
        is_safe(report, index + 1, -1_i8)
    }else {
        0_usize
    }
}
