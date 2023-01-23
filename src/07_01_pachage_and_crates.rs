use std::io;


fn ch_07_01_enter(){
    //crate是rust编译时的最小单位，
    //crate可以包含模块，模块可以定义在其他文件
    //crate有两种形式：二进制项和库
    //二进制项目可被编译为.exe，它必须有一个主函数作为入口
    //而库不需要，也不会被编译为可执行程序，他们提供一些函数之类的东西供外部调用，比如前面猜字游戏中的rand这个crate
    //大多数时候谈到crate指的都是库，这与其它语言中的library一致。

    //package可以包含多个create，但至少一个，并且还会有一个Cargo.toml来决定如何构建程序
    
}