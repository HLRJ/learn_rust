
fn main() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {   //如果有一个字符串 slice，可以直接传递它。
// 如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。这种灵活性利用了 deref coercions 隐式传递的优势
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//这里面主要是将s与word的所有权绑定，而不是新生成一个所有权，由于函数返回的类型都相同，用了变量赋值隐藏的性质，多次赋值