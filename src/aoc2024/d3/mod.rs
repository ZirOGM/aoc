use regex::Regex;

pub fn execute(){
    let input: &str = include_str!("input.txt");
    let regex = Regex::new("mul\\(\\d+,\\d+\\)|do\\(\\)|don't\\(\\)").unwrap();
    let filtered = filter_data(extract_data(input, regex));
    println!("{}", multiply_all(filtered)); 
}

fn filter_data(data: Vec<&str>) -> Vec<&str>{
    let mut insert = true; 
    let mut result: Vec<&str> = Vec::new();
    for oper in data{ 
        match oper { 
            "do()" => insert = true,
            "don't()" => insert = false,
            _ => if insert {result.push(oper)}
        }
    } 
    result    
}
fn multiply_all(parsed: Vec<&str>) -> usize {
    parsed.iter().fold(0, |acc, mul| acc + multiply(mul))
}

fn extract_data(input: &str, regex: Regex) -> Vec<&str> {
    input.lines().flat_map(|x| {
        regex.find_iter(x).map(|m| m.as_str()).collect::<Vec<&str>>()
    }).collect()
}

fn multiply(input: &str) -> usize{
    let regex = Regex::new("\\((\\d*,\\d*)\\)").unwrap();
    regex.captures(input)
        .into_iter().flat_map(|m| m[1].split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
        ).reduce(|a, b| a * b).unwrap()
}