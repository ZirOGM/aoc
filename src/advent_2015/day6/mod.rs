use regex::Regex;

pub fn execute() {
    let input = include_str!("input.txt");
    let mut lights = vec![vec![0; 1000]; 1000];
    let instructions: Vec<Instruction> = input.lines().map(|line| {
        parse_instructions(line)
    }).collect();
    
    instructions.iter().for_each(|i| apply_instruction(i, &mut lights));
    
    let on_lights = lights.iter().flatten().sum::<i32>();
    
    println!("{}", on_lights);
}

fn parse_instructions(input: &str) ->  Instruction{
   
    let regex = Regex::new(".* (\\d*,\\d*) .* (\\d*,\\d*)").unwrap();
    
    let action = if input.contains("turn on") {
        1_i8
    } else if  input.contains("toggle") {
        2_i8
    } else { 
        -1_i8 
    };
    let Some(caps) = regex.captures(input) else { panic!() };
    
    Instruction{
        action,
        points: (parse_point(&caps[1]),  parse_point(&caps[2])),
    }
}

fn parse_point(point: &str) -> Point {
    let coordinates: Vec<usize> =  point.split(',')
       .map(|point| {
           point.parse::<usize>().unwrap()
       }).collect();
    
    Point{x: coordinates[0], y: coordinates[1]}
}

fn apply_instruction(instruction: &Instruction, lights: &mut Vec<Vec<i32>>) -> () {
    
    for x  in instruction.points.0.x..=instruction.points.1.x {
        for y in instruction.points.0.y..=instruction.points.1.y{
            lights[x][y] = lights[x][y] + instruction.action as i32;
            if lights[x][y] < 0 {
                lights[x][y] = 0;
            }
        }
    }
    
}

struct Point {
    x: usize,
    y: usize,
}

struct Instruction{
    action: i8,
    points: (Point, Point)
}