use RustBook_Instances_CN::tools;

pub fn enter(){
    type_alias();
    tools::print_line();
    never_type();
    tools::print_line();
    dynamically_sized_types();
}


//可以使用type关键字给现有类型一个别的名字
//Kilometers是i32的别名
type Kilometers = i32;
//通过类型别名，可以减少重复的代码，减少冗长的类型名，比如下面这样
type Thunk = Box<dyn Fn() + Send + 'static>;
///类型别名
fn type_alias(){
    let x :i32 = 5;
    let y : Kilometers = 5;
    println!("x = {},y = {}",x,y); 
    //因为是同类型，所以可以相加
    println!("x + y = {}",x+y);

    let f: Thunk = Box::new(|| println!("hi"));
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
}

fn takes_long_type(f: Thunk){
    // --snip--
}

//类型别名也经常与 Result<T, E> 结合使用来减少重复。
//考虑一下标准库中的 std::io 模块。I/O 操作通常会返回一个 Result<T, E>，
//因为这些操作可能会失败。标准库中的 std::io::Error 结构体代表了所有可能的 I/O 错误。
//std::io 中大部分函数会返回 Result<T, E>，其中 E 是 std::io::Error
//为此，std::io 有这个类型别名声明：
//type Result<T> = std::result::Result<T, std::io::Error>;
//这样就可以在函数签名中使用 Result<T> 而不是 std::result::Result<T, std::io::Error>。

//rust有一个特殊类型，叫做never type(!)
//作用是：在函数不返回值的时候作为返回值
// fn bar()->!{

// }

fn never_type() {
    //考虑如下代码：
    // let guess: u32 = match guess.trim().parse() {
    // Ok(num) => num,
    // Err(_) => continue,

    //rust要求guess.trim().parse()的匹配必须返回一个u32
    //而continue的人值是!

    //也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。
    //前者是 u32 值，而后者是 ! 值。因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。
    //!可以强转为任何类型
    
    //nerver type的另一个用途是panic!
    //具体参看Option<T>的unwrap方法
}

///动态大小类型
fn dynamically_sized_types(){
    //str是一个动态大小类型
    //直到运行时都不知道字符串有多长
    //因为直到运行前都不确定str的大小，所以无法创建str类型的变量，就像下面一样
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    //Rust 需要知道应该为特定类型的值分配多少内存，
    //同时所有同一类型的值必须使用相同数量的内存。
    //如果允许编写这样的代码，也就意味着这两个 str 需要占用完全相同大小的空间，
    //不过它们有着不同的长度。这也就是为什么不可能创建一个存放动态大小类型的变量的原因(？没理解这块)

    //不同于&T，只存储了T的内存地址
    //&str是一个双字结构体，第一个字是一个指向字符串的地址，第二个字是字符串的长度
    //换言之，&str存放了str的地址和长度
    //这样就可以在编译期知道它的大小(unsize长度的两倍)

    //***这里是 Rust 中动态大小类型的常规用法：
    //它们有一些额外的元信息来储存动态信息的大小。
    //这引出了动态大小类型的黄金规则：必须将动态大小类型的值置于某种指针之后。

    //也可以将指针和str结合，比如Box<str>或Rc<str>

    //trait也是一种动态大小类型
    //可以通过trait的名称来引用
}

//为了处理 DST，Rust 提供了 Sized trait 来决定一个类型的大小是否在编译时可知。
//这个 trait 自动为编译器在编译时就知道大小的类型实现。
//另外，Rust 隐式的为每一个泛型函数增加了 Sized bound。也就是说，对于如下泛型函数定义：
fn generic<T>(t: T) {
    // --snip--
}
//实际上被当作如下处理：
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }

//泛型函数默认只能用于在编译时已知大小的类型。然而可以使用如下特殊语法来放宽这个限制，
//意为：?Sized 上的 trait bound 意味着 T 可能是也可能不是 Sized
// fn generic<T: ?Sized>(t: &T) {
//     // --snip--
// }

