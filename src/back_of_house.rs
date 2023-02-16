// use crate::deliver_order;
use std::fmt::format;

pub fn to_string(){
    fix_incorrect_order();
    
}

fn fix_incorrect_order(){
    println!("fix incorrect order");
    cook_order();
    //使用super来调用父模块（crate根）的函数
    super::deliver_order();
}

fn cook_order(){
    println!("cook order");
}

//还可以使用pub来设计公有的结构体和枚举
//但要注意，其中的字段仍然是私有的，如果要让字段也变成公有的，仍然需要使用pub关键字来指定
pub struct Breakfast{
    //使用pub来标记结构体中的字段使其也变为公有的
    pub toast:String,
    seasonal_fruit:String,
}

//结构体的一些实现
impl Breakfast {
    pub fn to_string(&self)->String{
        return format!("toast is {}, seasonal fruit is {}",self.toast, self.seasonal_fruit);
    }
    
    //注意，因为Breakfast结构体拥有私有字段，所以要提供一个公有的关联函数来构造一个Breakfast的实例
    pub fn summer(toast: &str)->Breakfast{
        return Breakfast{
            toast:String::from(toast),
            seasonal_fruit:String::from("peaches"),
        };
    }
}

//对于枚举，如果声明为pub，则其所有成员都是pub的
pub enum Appetizer{
    Soup,
    Salad,
}