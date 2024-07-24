fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum_of_odd: i32 = numbers.iter().filter(|&num| num % 2 != 0).sum();

    println!("홀수의 합: {}", sum_of_odd);
}
