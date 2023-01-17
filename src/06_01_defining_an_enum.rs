use std:: cmp::Ordering;
use std::io;


/// Rust中使用enum关键字定义一个枚举
enum IpAddrKindEnum{
    None,
    V4(String),//Rust中的枚举允许指定类型
    V6(String),
    V4_u8(u8,u8,u8,u8),//另一种参数类型的V4枚举
}

/// 包含四个不同类型成员的枚举
enum MessageEnum{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

/// 在Rust中也可以为枚举定义方法
impl MessageEnum{
    fn call(&self){
        //在这里定义方法体
        println!("MessageEnum.call!");
    }
}

fn ch_06_01_enter(){
    //使用这样的方式声明枚举变量
    let none : IpAddrKindEnum = IpAddrKindEnum::None;
    //对指定了具体类型的变量，以这样的方式声明
    //IpAddrKindEnum::V4是一个获取string参数然后返回一个IpAddrKindEnum类型实例的函数
    let four : IpAddrKindEnum = IpAddrKindEnum::V4(String::from("123.0.0.1"));
    let four_another : IpAddrKindEnum = IpAddrKindEnum::V4_u8( 127 , 0 , 0 , 0 );
    let six : IpAddrKindEnum = IpAddrKindEnum::V6(String::from("::1"));

    let write : MessageEnum = MessageEnum::Write(String::from("hello"));
    //调用枚举方法
    write.call();

}