pub fn execute(){
    let input: &str = include_str!("input.txt");
    let chars = get_matrix(input);
    // println!("{:?}", chars);
    println!("{}", find_word(&chars))
}

fn get_matrix(input: &str) -> Vec<Vec<char>>{
    input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn find_word(char_matrix: &Vec<Vec<char>>) -> usize{
    let mut word_cnt: usize = 0;
    for y in 0..char_matrix.len() {
        for x in 0..char_matrix[y].len() {
            if *char_matrix.get(y).unwrap().get(x).unwrap() == 'X'{
                word_cnt += check_dir(char_matrix, (x,y))
            }
        }
    }
    word_cnt
}

fn check_dir(matrix: &Vec<Vec<char>>, pos: (usize, usize)) -> usize{
    let mut cnt = 0;
    for x in -1..=1{
        for y in -1..=1{
           if x == 0 && y == 0 {
               continue;
           }
           if scan_dir(matrix, pos, (x, y)) == true {
               cnt += 1;
           } 
        }
    }
    cnt
}
fn scan_dir(matrix: &Vec<Vec<char>>, pos: (usize, usize), dir: (isize, isize)) -> bool{
    let word = vec!['M', 'A', 'S'];
    let (dx, dy) = dir;
    let mut i: isize = pos.0 as isize;
    let mut j: isize = pos.1 as isize;
    let mut n = 0;
    while n < 3 {
        i += dx;
        j += dy;
        if j < 0 || j >= matrix.len() as isize || i < 0  || i >= matrix[j as usize].len() as isize { 
            return false   
        }
        if *matrix.get(j as usize).unwrap().get(i as usize).unwrap() == *word.get(n).unwrap() {
           n += 1; 
        }
        else { 
            return false
        }
    }
    true
}