#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle { //所有在 impl 块中定义的函数被称为 关联函数（associated functions），
        // 因为它们与 impl 后面命名的类型相关。我们可以定义不以 self 为第一参数的关联函数（因此不是方法）
        Rectangle {
            width: size,
            height: size,
        }
    }
}
//这里创建一个相同长和宽的图形，然后返回结构体实例和所有权
fn main() {
    let sq = Rectangle::square(3);
}
//使用结构体名和 :: 语法来调用这个关联函数：比如 let sq = Rectangle::square(3);。这个方法位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间。