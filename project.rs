use std::io;

fn main() {
    // 정수 입력 받기
    let mut input = String::new();
    println!("첫번째 정수를 입력하세요:");
    io::stdin().read_line(&mut input).expect("입력을 읽지 못했습니다.");
    let first: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");
    
    input.clear();
    println!("두번째 정수를 입력하세요:");
    io::stdin().read_line(&mut input).expect("입력을 읽지 못했습니다.");
    let second: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");
    
    input.clear();
    println!("세번째 정수를 입력하세요:");
    io::stdin().read_line(&mut input).expect("입력을 읽지 못했습니다.");
    let third: i32 = input.trim().parse().expect("정수로 변환할 수 없습니다.");
    
    // 함수 호출 및 결과 출력
    let result = process_numbers(first, second, third);
    println!("결과: {}", result);
}

// 입력된 숫자에 따라 처리하는 함수
fn process_numbers(first: i32, second: i32, third: i32) -> i32 {
    match first {
        1 => second + third,
        2 => second - third,
        _ => {
            println!("첫번째 숫자가 1 또는 2가 아닙니다. 기본값으로 0을 반환합니다.");
            0
        }
    }
}