#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  //在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写。
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
//使用impl 关键字   创建方法  也就是结构体的专属函数
//在方法中  &self必不可少 如果少了&self，就不是方法了 是关联函数