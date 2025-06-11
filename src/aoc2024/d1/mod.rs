pub fn execute(){
    let input: &str = include_str!("input.txt");
    let (first, second) = input.lines().into_iter().try_fold(( Vec::with_capacity(1000), Vec::with_capacity(1000) ), |acc , line| {
        let values: Vec<usize> = line.split(" ").map(|value| value.parse::<usize>().unwrap()).collect();
        acc.0.push(values[0]);
        acc.1.push(values[1]);
        acc
    });

    println!("{:?}", first);
    
}