fn main() {
    let (one, two) = read_input();
    let count = parse_line_one(one);
    let mut vector = parse_line_two(two);
    vector.sort();

    let mut most_biggest = 0;
    loop {
        if vector.is_empty() {
            break;
        }

        most_biggest = vector.pop().unwrap();
        if in_bounds(most_biggest) {
            break;
        }
    }

    let mut second_biggest = 0;
    loop {
        if vector.is_empty() {
            break;
        }
        second_biggest = vector.pop().unwrap();
        if in_bounds(second_biggest) {
            break;
        }
    }

    let product: i64 = (most_biggest * second_biggest);
    println!("{}", product)
}


fn in_bounds(int: i64) -> bool
{
    let mut inbounds = true;
    if int <= 0 { inbounds = false; }
    if int >= 200000 { inbounds = false; }
    return inbounds;
}


fn read_input() -> (String, String) {
    let mut one = String::new();
    std::io::stdin().read_line(&mut one);

    let mut two = String::new();
    std::io::stdin().read_line(&mut two);

    return (one.replace("\n", ""),
            two.replace("\n", ""));
}


fn parse_line_one(input: String) -> i64 {
    let output: i64 = input.parse().unwrap();
    return output;
}


fn parse_line_two(input: String) -> Vec<i64> {
    let mut int_array_output: Vec<i64> = Vec::new();

    for string in input.split(" ") {
        let int: i64 = string.parse().unwrap();
        int_array_output.push(int);
    }

    return int_array_output;
}
