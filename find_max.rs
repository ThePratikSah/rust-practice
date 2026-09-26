// Write a function that finds the largest number in a vector.

fn main() {
    let vector: Vec<i32> = vec![10, 4, 25, 7, 2];
    let result = find_max(vector);
    println!("{:?}", result);
}

fn find_max(vector: Vec<i32>) -> i32 {
    let mut max_num = 0;
    for element in vector {
        if element > max_num {
            max_num = element;
        }
    }
    return max_num;
}