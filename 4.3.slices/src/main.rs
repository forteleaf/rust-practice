// 소유권을 갖지 않는 데이터 타입 slices
fn main() {
    //    let mut s = String::from("hello world");
    //    let word = first_word(&s);
    //    s.clear();
    //    println!("the first word is: {}", word);
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // 바이트 배열의 반복자(iterator)를 생성
        if item == b' ' {
            // 바이트 리터널 문법을 사용하여 공백을 찾는다.
            return &s[0..i];
        }
    }
    &s[..]
}
