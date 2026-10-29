use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u32 = 37;
    let mut attempts = 0;
    println!("1~100의 정수를 입력하시오. q를 입력하면 종료됨.");

    loop{
        let mut input = String::new();  // 만일 input 값으로 12 입력
        match io::stdin().read_line(&mut input){ // "12\n"의 문자열로 넘어감
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("입력 실패: {error}");
                break;
            }           
        };
        let text = input.trim(); // "12"의 문자열로 바꾸어줌
        if text == "q" {
            break;
        }

        let guess: u32 = match text.parse() { // 12 라는 숫자값으로 변환.
            Ok(number) => number, // 이미 guess 뒤에 u32라는 값임을 알렸으므로 <u32> 생략 가능
            Err(_) => {
                println!("정수 형식으로 다시 입력함.");
                continue;
            }
        };

        attempts += 1;
        match guess.cmp(&secret) { // guess와, secret을 빌려서 읽은 값을 비교함.
            Ordering::Less => println!("더 큰 수임."),
            Ordering::Greater => println!("더 작은 수임."),
            Ordering::Equal => {
                println!("정답: {guess}, 유효 시도: {attempts}");
                break;
            }
        }
    }
}
