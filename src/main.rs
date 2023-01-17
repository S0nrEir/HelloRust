use std::io;//基础的io库
use rand::Rng;//随机库
use std::cmp::Ordering;//比较和排序

fn main(){
    main_enter();
}


fn main_enter() {
    //Rust还未包含随机数的库，然而，有一个类似的方案，叫做crate，
    //这是一个基于社区的Rust代码包，可能类似于代码编译后的DLL
    //在toml中添加“rand”依赖，为本地rust工程引入相应的包
    //rand::thread_rng()函数提供一个随机数生成器，它位于当前线程，从系统获取随机种子，接着调用gen_range函数来生成一个指定范围的随机数
    //gen_range的参数是一个范围表达式，他的表现形式为:start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("the secret number is :{secret_number}");
    println!("input guess:");
    // println!("you guessed:{guess}");

    //loop关键字创建一个无限循环
    loop {

        //使用let关键字创建变量，在Rust中变量默认是不可变的，除非在声明时再加上mut关键字
        //返回一个String类型的新实例到guess
        //::new:静态函数
        let mut guess = String::new();
        //stdin函数，它返回一个Stdin的实例
        //read_line返回一个Result类型，这是一个枚举，如果程序正常，expect会返回read_line读到的字节数
        //io::Result实例的值是Err和ok，如果是Err则会中断程序并且输出错误信息，如果是ok则会返回read_line读取到的字节数
        //当然，不调用expect也可以正常编译，但会出现一个警告
        println!("input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("faild to read line!");
        
        //Rust允许用一个新值来隐藏（shadow）变量之前的值，而非创建一个新的
        //通过guess的指定类型（:u32）来告诉parse函数要转换成什么类型
        let guess:u32 = guess.trim().parse().expect("parse faild");

        //cmp函数用来比较两个值，它接受一个引用作为参数，他会返回一个Ordering枚举
        //Ordering也是一个结果枚举，它有三种结果，分别是Less、Equal和Greater
        //使用match表达式对cmp返回的Ordering决定接下来的行为
        match guess.cmp(&secret_number) {
            Ordering::Less=>println!("Small"),//一个分支包含一个模式(pattern)
            // Ordering::Equal=>println!("Equal"),
            Ordering::Greater=>println!("Greater"),
            Ordering::Equal=>{
                println!("Equal");
                break;
            }
        }
    }
}

