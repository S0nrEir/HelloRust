pub use crate::front_of_house::hosting;
use std::{io, option};
mod back_of_house;
mod front_of_house;
pub mod mod_intergration_test;
use back_of_house::Appetizer;

//声明一个模块，以mod关键字开始，后跟模块名
//这里声明一个名为【前台】的模块

//main.rs和lib.rs是crate的根，
//这是因为这两个文件的内容分别在crate模块结构的根组成了一个名为crate的模块，这被称为模块树
//注意，整个模块都根植于名为crate的模块下

//这里的模块树形结构为：
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment


//定义一个新函数：eat_at_restaurant
pub fn eat_at_restaurant(){
    println!("eat at restaurant!");
    //使用两种路径的方式分别调用hosting::add_to_waitlist
    //crate根定义了一个名为front_of_house的模块，front_of_house模块不是pub的，但是因为eat_at_restaurant和
    //front_of_house定义在同一模块（兄弟）所以可以在eat_at_restaurant中引用front_of_house
    //可以访问hosting的父模块(front_of_house)，hosting模块也被标记为pub，所以也可以访问hosting
    //add_to_waitlist函数同理
    //绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    // front_of_house::hosting::add_to_waitlist();

    //***
    //在rust中，默认的所有项（函数，方法，结构体，枚举等）对于父模块都是私有的
    //父模块不能使用子模块中的私有项，但是子模块可以使用父模块中的项
    //可以使用pub关键字来创建公共项，使子模块的部分项可以暴露给父模块
    //因此使用pub关键字标记hosting模块和add_to_waitlist函数
    //（单独标记hosting模块是不行，因为add_to_waitlist函数仍然是私有的）
    //***

    //在使用use创建段路径后，可以直接调用add_to_waitlist
    hosting::add_to_waitlist();

    //在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //改变主意更换面包类型
    meal.toast = String::from("wheat");
    println!("meal is :{}",meal.to_string());
    println!("-----------------------");
    println!("i'd like {} toast please",meal.toast);

    //使用枚举
    let soup:Appetizer = Appetizer::Soup;
    let salad:Appetizer = Appetizer::Salad;
    
}

fn deliver_order(){
    println!("deliver order!");
}

//十七章oop相关：
pub struct AverageCollection{
    _list:Vec<i32>,
    _average:f64,
}

impl AverageCollection{
    ///添加值
    pub fn add(&mut self,value:i32)->bool{
        self._list.push(value);
        self.update_average();
        return true;
    }

    ///获取平均值
    pub fn get_average(&self)->f64{
        return self._average;
    }

    ///移除队尾最后一个
    pub fn pop_remove(&mut self)->Option<i32>{
        let result = self._list.pop();
        match result {
            Some(value) => {
                self.update_average();
                return Some(value);
            }
            _ => return None,
        }
    }

    ///移除指定位置
    pub fn remove_at(&mut self,index:usize)->bool{
        if index >= self._list.len() {
            return false;
        }
        
        self._list.remove(index);
        return true;
    }

    pub fn new()->AverageCollection{
        return AverageCollection { 
            _list:vec![],
            _average:0.0,
        }
    }

    ///更新
    fn update_average(&mut self){
        let total:i32 = self._list.iter().sum();
        self._average = total as f64 / self._list.len() as f64;
        // let len = self._list.len();
        // let mut total : i32 = 0;
        // for i in 0..len{
        //     if let Some(number) = self._list.get(i){
        //         total += number;
        //     }
        // }
        // self._average = total as f64 / len as f64;
    }
}