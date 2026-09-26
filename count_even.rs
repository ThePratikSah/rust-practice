// Return how many even numbers exist in a vector
fn main() {
    let vector: Vec<i32> = vec![1, 2, 4, 7, 9, 10];
    let result = count_even(vector);
    println!("{}", result);
}

fn count_even(vector: Vec<i32>) -> i32 {
    let mut count = 0;
    for item in vector {
        if item % 2 == 0 {
            count += 1;
        }
    }
    return count;
}