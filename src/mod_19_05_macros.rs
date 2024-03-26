use RustBook_Instances_CN::tools;
use proc_macro;

pub fn enter(){
    macros_and_function();
    tools::print_line();
}

//在rust中，有使用macro_rules!的声明宏
//和三种过程宏:
// 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
// 类属性（Attribute-like）宏定义可用于任意项的自定义属性
// 类函数宏看起来像函数不过作用于作为参数传递的 token

//这是vec!宏的部分实现
// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

//注解表明只要导入了定义这个宏的 crate，该宏就应该是可用的。如果没有该注解，这个宏不能被引入作用域。
//(该宏导出可供其他包使用)

//接着使用 macro_rules! 和宏名称开始宏定义，且所定义的宏并 不带 感叹号。名字后跟大括号表示宏定义体，
//在该例中宏名称是 vec 。

//vec! 宏的结构和 match 表达式的结构类似。
//此处有一个分支模式 ( $( $x:expr ),* ) ，
//后跟 => 以及和模式相关的代码块。如果模式匹配，该相关代码块将被执行。
//这里这个宏只有一个模式，那就只有一个有效匹配方向，
//其他任何模式方向（译者注：不匹配这个模式）都会导致错误。更复杂的宏会有多个分支模式。
//记住，宏匹配的是代码结构而不是模式

//首先，一对括号包含了整个模式。使用美元符号（$）在宏系统中声明一个变量来包含匹配该模式的 Rust 代码。
//美元符号明确表明这是一个宏变量而不是普通 Rust 变量。
//之后是一对括号，其捕获了符合括号内模式的值用以在替代代码中使用。
//$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式命名为 $x。

//$() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。
//紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式。
//当以 vec![1, 2, 3]; 调用宏时，$x 模式与三个表达式 1、2 和 3 进行了三次匹配。

macro_rules! hey {
    //一条宏规则
    //左边是匹配器，右边是展开器
    //name表示变量名，将匹配结果放到name中，expr声明要匹配的类型，在这里是一个表达式
    //rust中有众多的选择器
    ($name:expr) => {
        println!("Hey,{}", $name);
    };
}

//重复模式的匹配
macro_rules! hi_to_all {
    //$($x:exrp)表示要重复的模式，后跟一个*表示匹配0次或多次
    //这有点类似于正则表达式，rust中还有符号表示不同的匹配模式和次数
    ($($x:expr),*)=>{
        {
            let mut temp_vec = Vec::new();
            //$(...)*表示重复的代码块，意思是重复匹配到的代码在这里展开
            //当这个宏被调用时，它的代码实际上是这样的：
            //hi_to_all!("Alice","Bob","Charlie","David");
            //let mut temp_vec = Vec::new();
            //temp_vec.push("Alice");
            //temp_vec.push("Bob");
            //temp_vec.push("Charlie");
            //temp_vec.push("David");
            //temp_vec
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

///声明宏
fn macros_and_function(){
    hey!("Rust");
    let mut vec = hi_to_all!("Alice","Bob","Charlie","David");
    for v in vec.iter(){
        println!("v:{}",v);
    }
}

//过程宏接受一段代码，然后对它进行操作，产生另一些代码作为输出，而不是像声明式的宏进行匹配
//过程宏有三种类型：自定义 #[derive] 宏、类属性（Attribute-like）宏和类函数宏
