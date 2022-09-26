fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {}", word);
    let reference_to_nothing = dangle();
}

// String 파라미터의 바이트 인덱스 값을 반환
fn first_word(s: &String) -> &str {
    // byte 배열로 변경
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn claculate_length(s: &String) -> usize {
    let size = s.len();
    println!("size : {}", size);
    size
}

fn dangle() -> String {
    // dangle은 String의 참조자를 반환합니다.
    let s = String::from("hello"); // s 는 새로운 String입니다.
    s // String s의 참조자를 반환 합니다.
}

//fn second_word(s: &String) -> &str {}
