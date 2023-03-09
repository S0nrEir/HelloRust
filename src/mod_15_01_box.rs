//智能指针通常使用结构体实现。
//智能指针不同于结构体的地方在于其实现了 Deref 和 Drop trait。Deref trait 允许智能指针结构体实例表现的像引用一样，
//这样就可以编写既用于引用、又用于智能指针的代码。Drop trait 允许我们自定义当智能指针离开作用域时运行的代码。

//因为 Box<T> 是一个指针，我们总是知道它需要多少空间：
//指针的大小并不会根据其指向的数据量而改变。
//这意味着可以将 Box 放入 Cons 成员中而不是直接存放另一个 List 值。
//Box 会指向另一个位于堆上的 List 值，而不是存放在 Cons 成员中
enum List{
    Cons(i32,Box<List>),
    Nil
}

pub fn enter(){
    use crate::mod_15_01_box::List::{Cons,Nil};
    // let b = Box::new(5);//在heap上存储一个i32
    // println!("b={}",b);

    let list = Cons(0x32, Box::new(Cons(0xfa, Box::new(Cons(0x8f, Box::new(Nil))))));
}