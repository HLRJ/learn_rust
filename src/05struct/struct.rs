struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}



fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

//编译不会通过，因为所有权和生命周期的问题   在结构体里面定义的&str字符串slice
//尝试在结构体中存储一个引用而不指定生命周期将是无效的
