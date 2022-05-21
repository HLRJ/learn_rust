fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //对于前两个分支，匹配模式是字面值 3 和 7，最后一个分支则涵盖了所有其他可能的值，
        // 模式是我们命名为 other 的一个变量。other 分支的代码通过将其传递给 move_player 函数来使用这个变量。  在这里other指的是9
        //这种通配模式满足了 match 必须被穷尽的要求
        //将9绑定到other上，去执行一个新的函数
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}


fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        //Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。
        // 这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。
        //这里面，执行reroll函数，但是不用管9的事儿
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //当然 也可以什么都不做
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

