use crate::mod_06_enum_match_if_let::MessageEnum;


pub fn enter(){
    _fn();
    print_line();
    fn_();
    print_line();
    __fn();
    print_line();
    fn__();
    print_line();
    __fn_();
    _fn__();
    print_line();
    ignore_value(3, 4);
    print_line();
    nested_ignore_value();
    print_line();
    nested_ignore_value();
    print_line();
    ignore_prefix();
    print_line();
    ignore_left_values();
    print_line();
    match_guard();
    print_line();
    bind();
}

///匹配字面值，可以匹配特定值，并且使用弃元来匹配不关心的其他值
fn _fn(){
    let x = 1;
    match x{
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _=> println!("anything"),
    }
}

///匹配命名变量
fn fn_(){
    // let x = Some(5);
    //如果x为none，则不会匹配任何some
    let x = None;
    let y = 10;

    match x {
        Some(50) => print!("got 50"),
        //这里的y会覆盖作用域之外的y
        //这里其实是引入了一个新的变量y
        //这个y会匹配任何Some中任意的值
        //换句话说只要x为Some(i32)，那么不管其内容值是什么都会匹配到这里
        Some(y) => println!("matched y, y = {y}"),
        _=> println!("default case ,x = {:?}",x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

///匹配多个模式
fn _fn_(){
    let x = 1;
    match x {
        //可以使用|符号匹配多个模式，他代表或(or)运算符模式
        //下一行代码的意思是：如果匹配到1 或 2 => println1(xxx)
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("anything"),
    }
}

///匹配值范围
fn __fn(){
    let x = 5;
    match x {
        //..=语法可以匹配闭区间范围内值
        //如果x是1，2，3，4或5，则匹配第一个分支
        1..=5 => println!("1 through 5") ,
        _ => println!("something else"),
    }

    //这是一个匹配字符的例子
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

///解构并分解值
fn fn__(){

    let p = Point { x: 0, y: 7 };
    //让变量ab匹配结构体中的xy
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //匹配结构体
    //分别匹配结构体中的值
    let p = Point { x: 0, y: 7 };
    match p {
        // Point { x:0, y:7 } => println!("yes"),
        // Point {x:0,y:7} => println!("yes"),
        //匹配到y为0，x随意
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        //匹配到x为0，y随意
        Point { x: 0, y } => println!("On the y axis at {y}"),
        //任意匹配
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

///解构枚举
fn __fn_(){
    // let msg = MessageEnum::ChangeColor(0, 160, 255);
    let msg = MessageEnum::ChangeColor(1, 160, 255);
    match msg {
        MessageEnum::Quit =>{
            println!("The Quit variant has no data to destructure.");
        },
        MessageEnum::Move { x, y } =>{
            println!("Move in the x direction {x} and in the y direction {y}");
        },
        MessageEnum::Write(text) =>{
            println!("Text message: {text}");
        },
        MessageEnum::ChangeColor(r,g ,b )=>{
            //嵌套匹配，匹配ChangeColor.r
            // match r {
            //     1 => println!("r is 1"),
            //     _ => println!("r is other"),
            // }

            //也可以像前面一样解构匹配
            match (r,g,b) {
                (1,g,b) => println!("r is 1"),
                _ => println!("r is other"),
            }
            println!("Change the color to red {r}, green {g}, and blue {b}");
        },
        _ => println!("none"),
    }

}

///
fn _fn__(){
    let msg = MessageEnum::OtherColor(Color::HSV(0,160,255));
    match msg {
        //第一个分支匹配MessageEnum.OtherColor.Color.RGB
        MessageEnum::OtherColor(Color::RGB(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        //第二个分支匹配MessageEnum.OtherColor.HSV
        MessageEnum::OtherColor(Color::HSV(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => {
        },
    }
}

///通过使用
//弃元会完全忽略第一个参数
//一个情况是，比如实现 trait 时，你需要特定类型签名但是方法实现并不需要某个参数时。
//这样可以避免一个存在未使用的方法参数的编译警告，就跟使用命名参数一样。
fn ignore_value(_:i32,y:i32){
    println!("This code only uses the y parameter: {}", y);
}

///嵌套忽略值
fn nested_ignore_value(){

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    //匹配setting_value和new_setting_value但忽略其匹配的值
    match (setting_value, new_setting_value) {
        //这里实际上是要匹配setting_value和new_setting_value都为some的情况，但并不关心其值
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        //也可以使用弃元忽略元组中特定位置的元素
        //这里忽略了第二和第四个值
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

}

//在名字前加下划线来忽略未使用的变量
fn ignore_prefix(){
    //如果你创建了一个变量但不使用它，Rust会给你一个警告
    //但在某些情况下这样的变量是有用的，比如只是暂时没有用到，日后会用到
    //如果希望rust不做警告，则可以在变量前加一个下划线
    let _x = 5;
    let y = 10;

    //注意，只使用 _ 和使用以下划线开头的名称有些微妙的不同
    //比如 _x 仍会将值绑定到变量，而 _ 则完全不会绑定。
    let s = Some(String::from("Hello!"));
    
    //下面的代码会编译报错，因为s的值会移动进_s
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    //但使用下划线就没有问题，因为没有把s绑定到任何变量，s没有被移动
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

}

///忽略剩余的值
fn ignore_left_values(){
    struct Point {x: i32, y: i32, z: i32,};
    let origin = Point { x: 1, y: 0, z: 0 };
    //只匹配origin.x，而忽略其他值
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        //也可以使用..来匹配元组的第一个和最后一个值
        //..将会忽略中间的值
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    //然而，下面的代码将编译报错
    //Rust不知道在second之前应该忽略多少值，以及在second之后忽略多少值
    //这样使用..是有歧义的，所以不行
    // let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
}

fn print_line(){
    println!("---------");
}

///匹配守卫
fn match_guard(){
    //是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。
    //匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。

    let num = Some(5);
    // let num = Some(4);
    match num {
        //一个模式Some(x)，和一个守卫if x % 2 == 0
        //匹配到第一个分支后还要检查是否为偶数如果不为偶数，则尝试匹配别的分支
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        //5 != 10
        //因为y没有被模式匹配覆盖，所以这里的匹配守卫可以使用外部的y作为检查条件
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    
    let x = 4;
    let y = false;
    match x {
        //这个分支匹配的情况是：x的值为4，5，6同时y为tru的情况
        4 | 5 | 6 if y => println!("yes"),
        // 4 | 5 | 6 if !y => println!("yes"),
        _ => println!("no"),
    }

}

fn bind(){
    let msg = Message::Hello { id: 5 };
    match msg {
        //匹配Message::Hello的范围是否在3~7的范围内
        //同时希望将其值绑定到id_variable变量中以便后续使用
        //在 3..=7 之前指定 id_variable @，捕获了任何匹配此范围的值并同时测试其值匹配这个范围模式。
        Message::Hello { id:id_variable @ 3..=7 } =>{
            println!("found an id in range {}",id_variable);
        },
        //第二个分支只在模式中指定了一个范围，
        //分支相关代码没有一个包含 id 字段实际值的变量。
        //id 字段的值可以是 10、11 或 12，
        //不过这个模式的代码并不知情也不能使用 id 字段中的值，因为没有将 id 值保存进一个变量。
        Message::Hello { id:10..=12 } =>{
            println!("found an id in another range");
        }
        //最后一个分支指定了一个没有范围的变量，
        //此时确实拥有可以用于分支代码的变量 id，
        //因为这里使用了结构体字段简写语法。
        //不过此分支中没有像头两个分支那样对 id 字段的值进行测试：任何值都会匹配此分支。
        Message::Hello { id } => println!("found some other id:{}",id),
        //使用 @ 可以在一个模式中同时测试和保存变量值。
    }

    match msg {
        Message::Hello { id } =>{
            println!("{}",id);
        },
        _ => {},
        
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub enum Color{
    RGB(i32,i32,i32),
    HSV(i32,i32,i32),
}

enum Message{
    Hello{id:i32}
}