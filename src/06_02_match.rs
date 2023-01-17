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

fn main(){
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