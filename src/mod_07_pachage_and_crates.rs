use std::io;

pub fn enter(){
    println!("------------------------");
    ch_07_01_enter();
    ch_07_02_enter();
    ch_07_03_enter();
}

fn ch_07_01_enter(){
    println!("
    //crate是rust编译时的最小单位，
    //crate可以包含模块，模块可以定义在其他文件
    //crate有两种形式：二进制项和库
    //二进制项目可被编译为.exe，它必须有一个主函数作为入口
    //而库不需要，也不会被编译为可执行程序，他们提供一些函数之类的东西供外部调用，
    //比如前面猜字游戏中的rand这个crate
    //crate可以看作是：lib.rs（库形式的crate），main.rs或bin/*.rs（二进制形式的crate）

    //大多数时候谈到crate指的都是库，这与其它语言中的library一致。

    //package可以包含多个create，但至少有一个，并且还会有一个Cargo.toml来决定如何构建程序");
    //crat 根（crate root）是一个源文件（src/lib.rs、src/main.rs、src/bin/*.rs 等），
    //Rust 编译器以它为起始，并构成你的 crate 的根模块
    
    //如果一个包同时含有 src/main.rs 和 src/lib.rs，
    //则它有两个 crate：一个库和一个二进制项，且名字都与包相同。
    //通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：
    //每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
    
}

fn ch_07_02_enter(){
    println!("//通过执行cargo new --lib 【lib_name】来创建一个名为lib_name的库
    //具体示例可以查看restaurant库");
    //rust有这样一条规则：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
}

fn ch_07_03_enter(){
    println!("
    //rust使用路径的方式来找到一个模块中具体项的位置，比如为了调用一个函数，就要知道他的位置
    //路径分为绝对路径和相对路径
    //绝对路径以crate的根为开头的全路径
    //相对路径则从当前模块开始，以self,super或当前模块的标识符为开头
    //这两种路径后都跟一个或多个双冒号::来分隔
    
    //关于具体的调用，请查看lib.rs")
}