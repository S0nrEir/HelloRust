//内部可变性是rust的一项特性，这允许在拥有不可变引用时也能改变数据，通常这是借用规则不允许的
pub trait Messenger {
    fn send(&self, msg:&str);
}

pub struct LimitTracker<'a,T:Messenger>{
    _messenger : &'a T,
    _value:usize,
    _max:usize
}

impl<'a,T> LimitTracker<'a,T> where T:Messenger, 
{
    pub fn new (messenger:&'a T,max:usize) -> LimitTracker<'a,T>
    {
        return LimitTracker 
        {
            _messenger: messenger, 
            _value: 0, 
            _max:max
        };
    }

    pub fn set_value(&mut self,value:usize){
        let mut msg = "";
        self._value = value;
        let percentage_of_max = self._value as f64 / self._max as f64;

        if percentage_of_max >= 1.0{
            msg = "Error:you are over your quota";
        }
        else if percentage_of_max >= 0.9{
            msg = "urgent warning:you've used up over 90% of you quota";
        }
        else if percentage_of_max >= 0.75{
            msg = "warning:you've used up over 75% of your quota";
        }

        self._messenger.send(msg);
    }
}

//RefCell<T>代表其数据的唯一所有权
//并且使用unsafe来规避Rust通常的借用规则，这意味着我们需要对它们手动管理，而不是靠rust编译器的借用规则
//RefCell的借用规则作用于运行时，
//RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
pub fn enter(){
    // fn_();
    self::tests::it_sends_an_over_75_percent_warning_message();
}

fn _fn_()
{
    
}

fn fn_(){
    //当有一个不可变值时，不能可变地借用他
    //如下代码无法编译通过
    // let x = 5;
    // let y = &mut x;
}

// Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；
// RefCell<T> 允许在运行时执行不可变或可变借用检查。
// 因为 RefCell<T> 允许在运行时执行可变借用检查，
//所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
//如果违反借用规则，Box<T>和Rc<T>会在编译期报错，而RefCell<T>则会发生panic

//#[cfg(test)]
mod tests{
    use std::{cell::RefCell, vec};
    use super::*;

    struct MockMessenger{
        _sent_messages:RefCell<Vec<String>>,
        // _sent_messages:Vec<String>,
    }

    impl MockMessenger
    {
        fn new() -> MockMessenger
        {
            return MockMessenger 
            { 
                _sent_messages: RefCell::new(vec![]),
            };
        }

        //在相同域内创建两个可变借用
        //在编译时不会有问题，但在运行时会发生panic
        fn test_mut_borrow_mut(&self)
        {
            let mut one_borrow = self._sent_messages.borrow_mut();
            let mut two_borrow = self._sent_messages.borrow_mut();

            one_borrow.push(String::from("123"));
            two_borrow.push(String::from("123"));
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) 
        {
            println!("MockMessenger.send!");
            // self.test_mut_borrow_mut();
            // return;
            //此处报错，借用检查器不允许这样做
            //因为send函数获取了self的不可变引用，也不能使用&mut self代替，因为这不符合Messenger trait的签名
            //因为_sent_messages要改变自身的值（push函数），
            //此时就需要一个在不可变的情况仍能改变自身值得方法，也就是RefCell<T>
            //无法使用&mut self，也要符合trait签名的实现方法之一就是：使用RefCell
            // self._sent_messages.push(String::from(message));
            //borrow_mut函数可以获得RefCell中值的可变引用，接着对vector的可变引用调用push
            self._sent_messages.borrow_mut().push(String::from(message));
        }
    }

    //#[test]
    pub fn it_sends_an_over_75_percent_warning_message(){
        let mock_messenger = MockMessenger::new();
        let mut limit_tricker = LimitTracker::new(&mock_messenger,100);
        limit_tricker.set_value(80);
        //使用borrow获取不可变借用
        assert_eq!(mock_messenger._sent_messages.borrow().len(),1);
    }
    //borrow()和borrow_mut()都属于RefCell<T>安全API的一部分，borrow 方法返回 Ref<T> 类型的智能指针，
    //borrow_mut 方法返回 RefMut<T> 类型的智能指针。这两个类型都实现了 Deref，所以可以当作常规引用对待。
}
//当创建不可变和可变引用时，分别使用 & 和 &mut 语法。
//对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法
//这属于 RefCell<T> 安全 API 的一部分。
//borrow 方法返回 Ref<T> 类型的智能指针，borrow()获取的是不可变借用
//borrow_mut 方法返回 RefMut<T> 类型的智能指针。borrow_mut返回的是可变借用
//这两个类型都实现了 Deref，所以可以当作常规引用对待。

//Rc<T>和RefCell<T>可以结合使用，这样就可以得到拥有多个所有者并且可变的值了
//比如这样：
// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
// }