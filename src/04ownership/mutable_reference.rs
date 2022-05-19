fn main() {
    let mut s = String::from("hello");

    change(&mut s);//函数借用完后 归还所有权
    change(&mut s); //接着借
    let r1 = &mut s; //借用了 但是没还
    let r2 = &mut s; //已经有人借了，暂时接不了
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}