use std::borrow::Borrow;
// use crate::List::{Cons,Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::vec;

#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil
}

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        return match self {
            List::Cons(_,item ) => Some(item),
            List::Nil=>None,
        }
    }
}

pub fn enter(){
    _fn();
    // fn_();
}

fn fn_(){
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    //Rc::clone 会增加 Rc<T> 实例的 strong_count
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}",Rc::strong_count(&b));
    println!("a rc count after changing a = {}",Rc::strong_count(&a));
}

#[derive(Debug)]
struct Node{
    value:i32,
    children:RefCell<Vec<Rc<Node>>>,
    parent:RefCell<Weak<Node>>,//对于node的父节点，使用弱引用，因为强引用会形成引用循环
}

fn _fn(){
    //避免引用循环的方法之一是：使用Weak<T>替换Rc<T>
    //调用Rc::downgrade会得到一个Weak<T>类型的指针

    //为 Weak<T> 引用的值可能已经被丢弃了，为了使用 Weak<T> 所指向的值，
    //我们必须确保其值仍然有效。为此可以调用 Weak<T> 实例的 upgrade 方法，
    //这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None。
    //因为 upgrade 返回一个 Option<Rc<T>>，Rust 会确保处理 Some 和 None 的情况，所以它不会返回非法指针。
    let leaf = Rc::new(Node{
        value:3,
        children:RefCell::new(vec![]),
        parent:RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value:5,
        children:RefCell::new(vec![Rc::clone(&leaf)]),//克隆自leaf，leaf现在有两个所有者
        parent:RefCell::new(Weak::new()),
    });

    //为leaf设置一个父节点
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // println!("{}",leaf.children.borrow().get(0));
    println!("leaf parent = {:?}",leaf.parent.borrow().upgrade());
}