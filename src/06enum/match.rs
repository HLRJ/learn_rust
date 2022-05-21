enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }  //如果使用{}的话  不需要加，
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
//列出 match 关键字后跟一个表达式，在这个例子中是 coin 的值。
// 这看起来非常像 if 使用的表达式，不过这里有一个非常大的区别：对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型的。
//当 match 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。如果模式匹配了这个值，这个模式相关联的代码将被执行。
// 如果模式并不匹配这个值，将继续执行下一个分支，非常类似一个硬币分类器。可以拥有任意多的分支

//如果分支代码较短的话通常不使用大括号，正如上示例中的每个分支都只是返回一个值。如果想要在分支中运行多行代码，可以使用大括号。

