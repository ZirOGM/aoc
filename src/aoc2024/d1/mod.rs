use std::iter::zip;

pub fn execute(){
    let input: &str = include_str!("input.txt");

    println!("one: {}", one(input));
    println!("two: {}", two(input));
}

fn one(input: &str) -> usize {
    let (mut left, mut right) = input.lines().into_iter().fold((Vec::with_capacity(1000), Vec::with_capacity(1000)), |mut acc, line| {
        let values: Vec<usize> = line.split("   ").map(|value| value.trim().parse::<usize>().unwrap()).collect();
        acc.0.push(values[0]);
        acc.1.push(values[1]);
        acc
    });
    left.sort();
    right.sort();

    let total_distance = zip(left, right).fold(0_usize, |acc, (first, second)| {
        acc + first.abs_diff(second)
    });
   total_distance
}

fn two(input: &str) -> usize {
    let (left, right) = input.lines().into_iter().fold((Vec::with_capacity(1000), Vec::with_capacity(1000)), |mut acc, line| {
        let values: Vec<usize> = line.split("   ").map(|value| value.trim().parse::<usize>().unwrap()).collect();
        acc.0.push(values[0]);
        acc.1.push(values[1]);
        acc
    });
    
    left.iter().fold(0_usize, |acc, next| {
        acc + right.iter().fold( 0_usize, |acc, rnext| {
            if *rnext == *next {
                acc + rnext 
            } else {
                acc
            }
        })
    })
}