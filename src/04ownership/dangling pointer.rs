fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！

//也就是说在函数里面 ，s变量值的所有权为s，当函数结束返回的&s是一个借用所有权的类型，但s所有权本身出了函数，即为消失，借用一个不存在的东西本身毫无意义
//可以返回String类型，将值、所有权返回出函数