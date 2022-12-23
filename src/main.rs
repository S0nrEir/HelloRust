use std::io;//基础的io库

fn main() 
{
    println!("guess the number!");
    println!("input your guess");

    //使用let关键字创建变量，在Rust中变量默认是不可变的，除非在声明时再加上mut关键字
    //返回一个String类型的新实例到guess
    //::new:静态函数
    let mut guess = String::new();
    //stdin函数，它返回一个Stdin的实例
    //read_line返回一个Result类型，这是一个枚举，如果程序正常，expect会返回read_line读到的字节数
    //io::Result实例的值是Err和ok，如果是Err则会中断程序并且输出错误信息，如果是ok则会返回read_line读取到的字节数
    io::stdin()
        .read_line(&mut guess)
        .expect("faild to read line!");

    //Rust还未包含随机数的库，然而，有一个类似的方案，叫做crate，
    //这是一个基于社区的Rust代码包，可能类似于代码编译后的DLL
    //在toml中添加“rand”依赖，为本地rust工程引入相应的包
    println!("you guessed:{guess}");
}

