fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


//必须处理none的情况，
//Rust 中的匹配是 穷尽的（exhaustive）：必须穷举到最后的可能性来使代码有效。
// 特别的在这个 Option<T> 的例子中，Rust 防止我们忘记明确的处理 None 的情况，这让我们免于假设拥有一个实际上为空的值，从而使之前提到的价值亿万的错误不可能发生。