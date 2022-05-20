struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
//对于user2的创建来说，由于用到了user1里的属性，且由于有String类型的，此时的=只是将换了个指向，此时不能使用user1，会造成二次释放问题