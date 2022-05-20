struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 { //借用一下所有权  而不是拿走所有权，这样结构体实例在main函数里面依然可以使用。
    rectangle.width * rectangle.height
}
