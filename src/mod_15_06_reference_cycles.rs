//创建引用循环并不容易，但也不是不可能。
//如果你有包含 Rc<T> 的 RefCell<T> 值或类似的嵌套结合了内部可变性和引用计数的类型
//请务必小心确保你没有形成一个引用循环；
//你无法指望 Rust 帮你捕获它们。
//创建引用循环是一个程序上的逻辑 bug，你应该使用自动化测试、代码评审和其他软件开发最佳实践来使其最小化。
//另一个解决方案是重新组织数据结构，使得一部分引用拥有所有权而另一部分没有。
//换句话说，循环将由一些拥有所有权的关系和一些无所有权的关系组成，只有所有权关系才能影响值是否可以被丢弃。

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
    // _fn();
    //  fn_();
    _fn_();
}

fn _fn_(){
    let leaf = Rc::new(Node{
        value:3,
        parent:RefCell::new(Weak::new()),
        children:RefCell::new(vec![]),
    });
    println!("1.leaf strong_count = {},weak count = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node{
            value:5,
            parent:RefCell::new(Weak::new()),
            children:RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        //leaf.parent持有一个branch的弱引用
        println!("------in scope------");
        println!("2.branch strong_count = {},weak count = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));
        println!("3.leaf strong_count = {},leaf weak_count = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));
        println!("------out scope------");
    }//离开作用域，branch离开作用域，强引用-1，被回收，弱引用虽然仍为1，但与是否被丢弃无关，所以没有内存泄漏
    println!("leaf parent = {:?}",leaf.parent.borrow().upgrade());
    println!("leaf strong_count = {},weak_count = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

}

fn fn_(){
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));//rc count = 1
    println!("a next item = {:?}", a.tail());//next item = Some(RefCell{Value:Nil})

    //b.tail指向a，a的引用计数+1
    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));//rc count = 2
    //Rc::clone 会增加 Rc<T> 实例的 strong_count
    println!("b initial rc count = {}", Rc::strong_count(&b));//b rc count = 1
    println!("b next item = {:?}", b.tail());//b next item =Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }

    //如果取消prinln！，则程序则会反复互相引用直到爆栈
    // println!("b rc count after changing a = {}",Rc::strong_count(&b));
    // println!("a rc count after changing a = {}",Rc::strong_count(&a));
}

#[derive(Debug)]
struct Node{
    value:i32,
    children:RefCell<Vec<Rc<Node>>>,//
    //对于node的父节点，使用弱引用，因为强引用会形成引用循环
    //父节点应该拥有其子节点：如果父节点被丢弃了，其子节点也应该被丢弃。
    //然而子节点不应该拥有其父节点：如果丢弃子节点，其父节点应该依然存在。这正是弱引用的例子
    parent:RefCell<Weak<Node>>,//可变弱引用
}

fn _fn(){
    //避免引用循环的方法之一是：使用Weak<T>替换Rc<T>
    //调用Rc::downgrade会得到一个Weak<T>类型的指针
    //weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

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