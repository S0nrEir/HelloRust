pub trait Messenger {
    fn send(&self, msg:&str);
}

pub struct LimitTracker<'a,T:Messenger>{
    messenger : &'a T,
    value:usize,
    max:usize
}

impl<'a,T> LimitTracker<'a,T> where T:Messenger, {
    pub fn new (messenger:&'a T,max:usize) -> LimitTracker<'a,T>{
        return LimitTracker { 
            messenger: messenger, 
            value: 0, 
            max
        };
    }

    pub fn set_value(&mut self,value:usize){
        let mut msg = "";
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0{
            msg = "Error:you are over your quota";
        }
        else if percentage_of_max >= 0.9{
            msg = "urgent warning:you've used up over 90% of you quota";
        }
        else if percentage_of_max >= 0.75{
            msg = "warning:you've used up over 75% of your quota";
        }

        self.messenger.send(msg);
    }
}

//RefCell<T>代表其数据的唯一所有权
//并且使用unsafe来规避Rust通常的借用规则，这意味着需要对他们手动管理
//RefCell的借用规则作用于运行时，
//RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
pub fn enter(){
    fn_();
}

fn fn_(){
    //当有一个不可变值时，不能可变地借用他
    //如下代码无法编译通过
    // let x = 5;
    // let y = &mut x;
}

#[cfg(test)]
mod tests{
    use std::{cell::RefCell, vec};
    use super::*;

    struct MockMessenger{
        sent_messages:RefCell<Vec<String>>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            return MockMessenger { 
                sent_messages: RefCell::new(vec![]),
            };
        }

        //编译报错，在相同域内创建两个可变借用
        //在编译时不会有问题，但在运行时会发生panic
        fn test(&self){
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from("123"));
            two_borrow.push(String::from("123"));
        }
    }

    impl Messenger for MockMessenger {
        //此处报错，借用检查器不允许这样做
        //因为send函数获取了self的不可变引用，也不能使用&mut self代替，因为这不符合Messenger trait的签名
        //无法使用&mut self，也要符合trait签名的实现方法之一就是：使用RefCell
        // fn send(&self, message: &str) {
        //     self.sent_messages.push(String::from(message));
        // }

        fn send(&self, message:&str){
            //使用borrow_mut函数获取RefCell值中的可变引用
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message(){
        let mock_messenger = MockMessenger::new();
        let mut limit_tricker = LimitTracker::new(&mock_messenger,100);
        limit_tricker.set_value(80);
        //使用borrow获取不可变借用
        assert_eq!(mock_messenger.sent_messages.borrow().len(),1);
    }
    //borrow()和borrow_mut()都属于RefCell<T>安全API的一部分，borrow 方法返回 Ref<T> 类型的智能指针，
    //borrow_mut 方法返回 RefMut<T> 类型的智能指针。这两个类型都实现了 Deref，所以可以当作常规引用对待。
}