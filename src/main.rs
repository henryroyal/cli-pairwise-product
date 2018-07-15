fn main() {
    let mut vector: Vec<i64> = Vec::new();

    let input = read_input();
    let mut items = input.split_whitespace().peekable();
    if items.peek().is_none() {
        std::process::exit(0);
    }

    for item in items {
        let int: i64 = item.parse().unwrap();

        vector.push(int);
    }

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

    println!("{}", (most_biggest * second_biggest));
    std::process::exit(0);
}


fn in_bounds(int: i64) -> bool
{
    let mut inbounds = true;
    if int <= 0 { inbounds = false; }
    if int >= 200000 { inbounds = false; }
    return inbounds;
}


fn read_input() -> String {
    let mut stdin = String::new();
    std::io::stdin().read_line(&mut stdin);
    return stdin.replace("\n", "");
}
