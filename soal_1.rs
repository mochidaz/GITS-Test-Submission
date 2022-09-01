use std::cmp::max;

fn count_digits(x: &Vec<i32>) -> i32 {

    let mut counter = 0;

    let mut counter2 = 0;

    for i in 0..x.len() - 1 {
        if x[i] != 1 {
            counter = max(counter, counter2);
            counter2 = 0;
        }
        counter2 += 1;
    }

    max(counter, counter2)
}

fn main() {
    let vec1 = vec![1, 1, 0, 1, 1, 1];
    let vec2 = vec![1, 0, 0, 1, 0, 1, 1];

    let result1 = count_digits(&vec1);
    let result2 = count_digits(&vec2);

    println!("{}", result1);
    println!("{}", result2);
}
