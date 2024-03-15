use std::io;
use std:: cmp::Ordering;
use crate::mod_18_03_patterns_syntax::Color;

pub fn enter(){
    ch_06_01_enter();
    ch_06_02_enter();
    ch_06_03_enter();
}
/// Rust中使用enum关键字定义一个枚举
enum IpAddrKindEnum{
    None,
    V4(String),//Rust中的枚举允许指定类型
    V6(String),
    V4_u8(u8,u8,u8,u8),//另一种参数类型的V4枚举
}

/// 包含四个不同类型成员的枚举
pub enum MessageEnum{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
    OtherColor(Color),
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
    //IpAddrKindEnum::V4是一个获取string参数然后返回一个IpAddrKindEnum类型实例的方法
    let four : IpAddrKindEnum = IpAddrKindEnum::V4(String::from("123.0.0.1"));
    let four_another : IpAddrKindEnum = IpAddrKindEnum::V4_u8( 127 , 0 , 0 , 0 );
    let six : IpAddrKindEnum = IpAddrKindEnum::V6(String::from("::1"));
    let write : MessageEnum = MessageEnum::Write(String::from("hello"));
    //调用枚举方法
    write.call();
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    //snip
    //...
    //...
    //...
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    //绑定值的枚举类型：
    Quarter(UsState),
    Other,
    Another
}

fn ch_06_02_enter(){
    // let coin:Coin = Coin::Quarter(UsState::Alabama);
    // println!("result is {}",value_of_cents(&coin));

    let roll_result : u8 = 1;
    dice_roll(&roll_result);
}

fn value_of_cents(coin:&Coin)->u8{
    //使用match检查coin，并且返回不同的值
    //与if的区别是，if的表达式必须返回一个bool值，而match可以是任何类型
    //***对于枚举，match必须检查枚举中的所有值***
    //类似于switch，不过在Rust中没有switch
    match coin {
        Coin::Penny => return 1,
        Coin::Nickel => return 5,
        Coin::Dime => return 10,
        
        //对于绑定了值的枚举类型，可以这样做检查
        Coin::Quarter(state) => {
            //match也是可以嵌套的
            match state {
                UsState::Alabama => {
                    println!("state quarter from alabama!");
                    return 25;
                }
                UsState::Alaska => {
                    println!("state quarter from alaska!");
                    return 25;
                }
                //上面的代码仅仅是为了展示match的嵌套，处于方便，也可以写成下面这样
                // println!("state quarter from {}",state);
                // return 25;

            }//end match
        }//end quarter

        //match也是支持多语句和花括号作用域的
        Coin::Other =>{
            println!("other!");
            return 0;
        } 
        Coin::Another => return 0,
    }
}

//获取一个Option(i32)，如果它内部有值，将其加1，如果没有，返回None
fn plus_one(x:&Option<i32>) -> Option<i32>{
    //使用match配合option
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}

fn dice_roll(roll_result:&u8){
    match roll_result {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        
        //不过即使在match需要穷举匹配的情况，对于一些默认或者不想去手动处理的情况，可以使用【other】这种模式来指定，
        //它表示了不符合前面任何模式的一种匹配模式
        // other => move_player(&roll_result),

        //如果不想使用通配模式获取的值时候，可以使用_，在C#中，他是弃元
        //它可以匹配任意值而不是用他们，这表示：告诉编译器明确的忽略其他值
        // _ => re_roll(),

        //使用单元值（空元组）作为_的分支代码，这将告诉编译器，在_匹配的情况下，将不运行任何代码
        _ => (),
    }
}

fn add_fancy_hat(){}
fn remove_fancy_hat(){}
fn move_player(num_space:&u8){
    println!("num_space is {}",num_space);
}
fn re_roll(){
    println!("re roll!");
}

fn ch_06_03_enter() {
    let config_max : Option<u8> = Option::Some(3u8);//u前面写多少 值就是多少，不知道这是什么骚语法（
    // match config_max {
    //     //匹配max，max被绑定为Some中的值
    //     Some(max ) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    //下面的代码等价于上面
    //if let语法通过等号分隔模式和表达式，这里的表达式对应match的第一个分支
    //模式不匹配时if中的代码将不会执行
    //这样可以减少模式匹配的代码量，只处理你想要进行模式匹配的代码
    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max);
    }else{
        //可以在if let中带一个else，这表示模式匹配中_部分的代码，即所有指定模式不匹配时的默认分支
        println!("else!");
    }   
}