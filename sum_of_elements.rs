// Write a function that takes a Vec<i32> 
// and returns the sum of all the elements.
fn main() {
    let vector: Vec<i32> = vec![1,2,3,4,5];
    let sum = sum_of_elements(vector);
    println!("Sum of all the elements: {sum}");
}

fn sum_of_elements(vector: Vec<i32>) -> i32 {
    let mut sum = 0;
    for elements in vector {
        sum += elements;
    }
    return sum;
}