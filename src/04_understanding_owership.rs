use std::io;

//Rust中没有GC，也无需开发者自行管理内存，而是使用“所有权”系统管理内存
//stack中的所有数据都必须占用已知且固定的大小，而heap上的数据则是大小未知或可能变化的
//==================================================================================
//一些基本概念：
//Rust中的每一个值都有一个所有者（owner）
//值在任何时刻有且只有一个owner
//当所有者离开作用域时，该值将被丢弃
//==================================================================================
//Rust的策略是：内存在拥有它的变量离开作用域后将被自动释放
//在离开作用域时，Rust将自动调用一个叫drop的特殊函数来释放其所持有的堆内存
//在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）
//如果你使用过 RAII 模式的话应该对 Rust 的 drop 函数并不陌生。
//==================================================================================
//
fn ch_04_enter(){
    //因为Rust的特性，所以像下边这样在do_something函数后，想要再次使用变量s1将会导致一个编译器错误
    //这是因为s1的值移动到函数内部后，在离开函数时，将会被释放，因此在调用do_something函数后使用s1变量在Rust中是不被允许的
    // let s1 = String::from("hello");
    // println!("s1 is :{s1}");

    
    //为了解决这个问题，可以使用引用，像下面这样
    //从Rust的角度来讲，从Rust的角度来讲，这并不会转移s1的所有权到函数内部，但却可以访问s1所持有的内存地址和其数据
    //注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    let s1 = String::from("reference");
    do_something_ref(&s1);
    println!("s1 is {s1}");
    
    //==================================================================================
    //关于引用的总结
    //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //引用必须总是有效的。
    //==================================================================================
    //slice引用：
    
}

fn do_something(str:String){
    println!("str is :{str}");
}

//函数的参数传入一个string的引用
//如果想要改变引用的值，则需要加上mut关键字，在变量声明和函数签名处都是如此，这被称为可变引用。
fn do_something_ref(str:&mut String){
    println!("str is {str}");
}