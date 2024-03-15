use RustBook_Instances_CN::tools;
use std::{ptr, slice};

//本章的关键在于：你要保证在不安全的代码块中访问的数据和操作都是有效、安全的（比如保证裸指针指向了有效地址）
//使用extern关键字集成C标准库中的abs方法
//extern 块中声明的方法在 Rust 代码中总是不安全的。
//因为其他语言不会强制执行 Rust 的规则且 Rust 无法检查它们，所以要确保extern中的内容是安全的
extern "C"{
    fn abs(input:i32)->i32;
}

//也可以使用 extern 来创建一个允许其他语言调用 Rust 方法的接口
// /在如下的例子中，一旦其编译为动态库并从 C 语言中链接，call_from_c 方法就能够在 C 代码中访问
//另外还要增加[no_mangle]注解来告诉编译器不要修改此方法的名称
#[no_mangle]
extern "C"{} fn call_from_c(){
    println!("just called a rust function from c");
}

//在 extern "C" 块中，列出了我们希望能够调用的另一个语言中的外部方法的签名和名称。
//"C"部分定义了外部方法所使用的应用二进制接口（application binary interface，ABI）
//ABI 定义了如何在汇编语言层面调用此方法。"C" ABI 是最常见的，并遵循 C 编程语言的 ABI。

//rust中全局变量被称为静态变量
//这是一个不可变静态变量
//访问不可变静态变量是安全的
static HELLO_WORLD:&str = "hello,world";
//可变静态变量
//访问和修改可变静态变量是不安全的
//如果多个线程同时访问可变静态变量，则可能导致数据竞争
static mut COUNTER:u32 = 0;

//通过unsafe关键字，可以开启一个不安全的代码块
//在不安全的代码块中，可以做的操作有：
//这五个操作不会被rust进行安全检查，其他的则平时一样
// 解引用裸指针
// 调用不安全的方法或方法
// 访问或修改可变静态变量
// 实现不安全 trait
// 访问 union 的字段
pub fn enter(){
    raw_pointer();
    tools::print_line();
    unsafe{
        unsafe_function();
    }
    tools::print_line();
    call_extern();
    tools::print_line();
    access_static_variables();
    add_to_count(1);
    tools::print_line();

}

fn raw_pointer(){
    //裸指针有两种类型：*const T和*mut T，这里的星号并非解引用，而是语法的一部分
    //裸指针允许忽略借用规则，同时拥有可变和不可变引用，并且允许存在多个指向相同位置的指针
    //不保证指向有效内存
    //允许为空
    //无法自动清理

    //这可以换取性能或使用硬件接口，代价是安全性
    //这里没有使用unsafe关键字
    //可以在安全代码块中创建裸指针，但不能在不安全块之外解引用裸指针
    let mut num = 5;
    //as关键字将可变和不可变引用转为裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1:{:?}",r1);
    println!("r2:{:?}",r2);

    //访问一个随机的内存地址
    let random_address = 0x19648ffa2cusize;
    let r = random_address as *const i32;

    println!("r:{:?}",r);
    //在unsafe块中对其解引用
    unsafe{
        //crash
        //因为访问了无效的值
        // println!("unsafe r:{}",*r);
        println!("unsafe r1:{}",*r1);
        println!("unsafe r2:{}",*r2);
    }

    //使用裸指针的一些主要场景是：调用C代码接口或构建借用检查器无法理解的安全抽象或者其他的
}

//使用unsafe开头声明一个不安全方法，这样可以在不安全代码块中调用它
//不安全方法本身就是不安全的，所以在方法体中进行不安全的操作时无需增加unsafe代码块
unsafe fn unsafe_function(){
    // println!("this is a unsafe function");
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    // let (a,b) = r.split_at_mut(3);
    let (a,b) = split_at_mut(&mut v, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("a:{:?}",a);
    println!("b:{:?}",b);
}

//无需将split_at_mut标记位unsafe，并可以在rust中安全调用它
//这是创建了一个不安全代码的安全抽象
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    //as_mut_ptr返回一个指向slice的裸指针
    //因为slice是i32类型，所以这个指针是一个*mut i32，存储在变量ptr中
    let ptr:*mut i32 = values.as_mut_ptr();
    assert!(mid <= len);
    //rust无法理解返回的借用是values的两部分
    //他会人为这是分开的两部分，所以产生了生命周期的编译问题
    //return (&mut values[..mid], &mut values[mid..])

    unsafe {
        return (
            //from_raw_parts_mut方法需要一个裸指针和一个长度作为参数，并返回一个slice
            slice::from_raw_parts_mut(ptr, mid),
            //使用ptr.add()来让指针偏移到mid的位置，获取剩余的slice
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
        //from_raw_parts_mut不安全，因为它需要获取一个裸指针，要保证裸指针是安全的
        //裸指针上的add也是不安全的，因为他要保证偏移后的指针位置也是有效的
    }
}

fn call_extern(){
    unsafe{
        println!("abs of -1 is:{}",abs(-1));
    }
}

fn access_static_variables(){
    println!("static variables is :{}",HELLO_WORLD);
}

fn add_to_count(inc:u32){
    unsafe{
        COUNTER += inc;
        println!("COUNTER after added : {}",COUNTER);
    }
}

unsafe trait Test_Trait{

}

unsafe impl Test_Trait for i32{

}

fn unsafe_trait(){

}