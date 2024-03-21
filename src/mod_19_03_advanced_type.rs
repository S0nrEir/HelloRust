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
    //如果允许编写这样的代码，也就意味着这两个 str 需要占用完全相同大小的空间
    //也即：相同的类型，占据的空间必须要一样。s1和s2都是str类型，但是它们的长度是不一样的，占据的空间不一致，所以无法通过编译
    //不过它们有着不同的长度。这也就是为什么不可能创建一个存放动态大小类型的变量的原因

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
    
    //***摘自Rust course:
    //Rust 需要明确地知道一个特定类型的值占据了多少内存空间，
    //同时该类型的所有值都必须使用相同大小的内存。
    //如果 Rust 允许我们使用这种动态类型，
    //那么这两个 str 值就需要占用同样大小的内存，这显然是不现实的: 
    //s1 占用了 12 字节，s2 占用了 15 字节，
    //总不至于为了满足同样的内存大小，用空白字符去填补字符串吧？

    //所以，我们只有一条路走，那就是给它们一个固定大小的类型：&str。
    //那么为何字符串切片 &str 就是固定大小呢？
    //因为它的引用存储在栈上，具有固定大小(类似指针)，
    //同时它指向的数据存储在堆中，也是已知的大小，
    //再加上 &str 引用中包含有堆上数据内存地址、长度等信息，
    //因此最终可以得出字符串切片是固定大小类型的结论。

    //与 &str 类似，String 字符串也是固定大小的类型。

    //正是因为 &str 的引用有了底层堆数据的明确信息，它才是固定大小类型。
    //假设如果它没有这些信息呢？那它也将变成一个动态类型。
    //因此，将动态数据固定化的秘诀就是使用引用指向这些动态数据，
    //然后在引用中存储相关的内存位置、长度等信息。
    //Rust 中常见的 DST 类型有: str、[T]、dyn Trait，它们都无法单独被使用，
    //必须要通过引用或者 Box 来间接使用（使用Box<T>将一个没有固定大小的trait变成一个有固定大小的trait对象）
    
    //Box 背后是调用 jemalloc 来做内存管理，所以堆上的空间无需我们的手动管理。
    //与此类似，带 GC 的语言中的对象也是借助于 Box 概念来实现的，
    //一切皆对象 = 一切皆 Box， 只不过我们无需自己去 Box 罢了。
    
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

