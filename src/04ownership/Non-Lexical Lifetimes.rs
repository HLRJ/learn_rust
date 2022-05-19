fn main() {
    fn main() {
        let mut s = String::from("hello");

        // let r1 = &s; // 没问题
        // let r2 = &s; // 没问题
        // let r3 = &mut s; // 大问题   能在拥有不可变引用的同时拥有可变引用。
        //
        // println!("{}, {}, and {}", r1, r2, r3);
        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用   借用后 使用过 相当于换回去了
        //编译器在作用域结束之前判断不再使用的引用的能力被称为 非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）

        let r3 = &mut s; // 没问题
        println!("{}", r3);
    }

}