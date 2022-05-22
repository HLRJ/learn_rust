use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

//如果对io::Resuslt 不重命名的话，因为两个不同库的Result函数名字相同，会引起冲突，但是通过as重命名的话，可以解决冲突问题，不过最好是带上前面的模块名，这样可以区分是使用的外部的函数还是本地创建的函数。
