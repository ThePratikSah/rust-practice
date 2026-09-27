fn main() {
    let message = String::from("Hello");
    let reverse_str = reverse_string(message);
    println!("{reverse_str}");
}

fn reverse_string(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let charsLength = chars.len();
    let mut result_string: String = String::new();
    for i in (0..charsLength).rev() {
        let character = chars[i];
        result_string.push(character);
    }
    return result_string;
}