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
    let most_biggest = get_largest_acceptable(&mut vector);
    let second_biggest = get_largest_acceptable(&mut vector);

    println!("{}", (most_biggest * second_biggest));
    std::process::exit(0);
}

fn get_largest_acceptable(vector: &mut Vec<i64>) -> i64
{
    let mut value = 0;
    loop {
        if vector.is_empty() {
            return value;
        }

        let next = vector.pop();
        if next.is_none() {
            return 0;
        }

        value = next.unwrap();
        if value <= 0 { continue; }
        if value >= 200000 { continue; }
        return value;
    }
}


fn read_input() -> String {
    let mut stdin = String::new();
    std::io::stdin().read_line(&mut stdin);
    return stdin.replace("\n", "");
}
