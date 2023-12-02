use std::fs::read_to_string;
fn main() {
    let filename = "./input.txt";
    let mut final_sum = 0;

    for line in read_to_string(filename).unwrap().lines() {
        let mut numbers: Vec<char> = Vec::new();
        for character in line.chars() {
            if character.is_numeric() {
                if numbers.len() > 1 {
                    numbers.pop();
                    numbers.push(character);
                } else {
                    numbers.push(character)
                }
            }
        }
        if numbers.len() == 1 {
            numbers.push(numbers[0]);
        }

        let number = numbers
            .iter()
            .fold(0, |acc, &c| acc * 10 + c.to_digit(10).unwrap_or(0) as i32);

        final_sum += number;

        // println!("Array - {:?} for line - {:?}", numbers, line);
        println!("Final sum = {:?}", final_sum);
    }
}
