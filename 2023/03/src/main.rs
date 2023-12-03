use std::fs;



fn main() {
        let inputs = fs::read_to_string("input.txt").expect("Can't read file!");
/*
    let inputs = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
*/
    let lines: Vec<String> = inputs.lines().map(String::from).collect(); 

    let mut sum = 0;
    for i in 0..lines.len() {
        let line = &lines[i];
        let mut part_number: String = "".to_owned();
        for indicie in line.char_indices() {
            if indicie.1.is_digit(10) {
                part_number.push(indicie.1);
            } else {
                let mut prev_line = None;
                let mut next_line = None;
                if i > 0{
                    prev_line = Some(&lines[i-1]);
                }
                if i < lines.len()-1 {
                    next_line = Some(&lines[i+1]);
                }
                sum += validate_part_number(&part_number, indicie.0, &line, prev_line, next_line);
                part_number.clear();
            }        }
        if part_number.len() > 0 {
            let mut prev_line = None;
            let mut next_line = None;
            if i > 0{
                prev_line = Some(&lines[i-1]);
            }
            if i < lines.len()-1 {
                next_line = Some(&lines[i+1]);
            }       
            sum += validate_part_number(&part_number, line.len(), &line, prev_line, next_line)
        }
    }
    println!("Sum is {sum}");
}


fn validate_part_number(part_number: &str, end: usize, line: &str, prev_line: Option<&String>, next_line: Option<&String>)-> i32{
    if part_number.len() == 0 {
        return 0;
    }
    let mut previous = end - part_number.len();
    if previous > 0 {
        previous -= 1; // start of line
    }

    let mut surrounding_chars:Vec<char> = Vec::new();
    surrounding_chars.push(line.chars().nth(previous).unwrap_or('.'));
    surrounding_chars.push(line.chars().nth(end).unwrap_or('.'));
    for j in previous..end+1 {
        if j == line.len(){ // end of line
            continue;
        }
        if let Some(p_line) = &prev_line{
            surrounding_chars.push(p_line.chars().nth(j).unwrap());
        }
        if let Some(n_line) = &next_line{
            surrounding_chars.push(n_line.chars().nth(j).unwrap());
        }
    }
    if surrounding_chars.iter().any(|&c| c != '.' && !c.is_digit(10) ){
        return part_number.parse::<i32>().unwrap();
    }
    return 0;
}
