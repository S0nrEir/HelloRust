pub use crate::front_of_house::hosting;
use std::{io, option, os::windows};
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

//ch17.1：
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

//ch17.2
///GUI控件绘制trait
pub trait Draw{
    ///绘制接口
    fn draw(& self);
}

///屏幕类型实例
pub struct Screen{
    ///Draw组件集合
    pub components : Vec<Box<dyn Draw>>,
    //impl Trait 和 dyn Trait 在 Rust 分别被称为静态分发和动态分发.
    //即当代码涉及多态时, 需要某种机制决定实际调用类型.
    //Rust 的 Trait 可以看作某些具有通过特性类型的集合,
    //在编译或运行时必须确定 Button 还是 TextView. 静态分发, 
    //正如静态类型语言的"静态"一词说明的, 在编译期就确定了具体调用类型. 
    //Rust 编译器会通过单态化(Monomorphization) 将泛型函数展开.

    //impl trait 和 dyn trait 区别在于静态分发于动态分发, 
    //静态分发性能好, 但大量使用有可能造成二进制文件膨胀;
    //动态分发以 trait object 的概念通过虚表实现, 会带来一些运行时开销.
    //又因 trait object 与 Trait 在不引入 dyn 的情况下经常导致语义混淆,
    //所以 Rust 特地引入 dyn 关键字, 在 Rust 2018 中已经稳定.

    //回忆一下第十章 “泛型代码的性能” 部分讨论过的，
    //当对泛型使用 trait bound 时编译器所执行的单态化处理：
    //编译器为每一个被泛型类型参数代替的具体类型生成了函数和方法的非泛型实现。
    //单态化产生的代码在执行静态分发（static dispatch）。
    //静态分发发生于编译器在编译时就知晓调用了什么方法的时候
    //这与 动态分发 （dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法。
    //在动态分发的场景下，编译器会生成负责在运行时确定该调用什么方法的代码。
    //当使用 trait 对象时，Rust 必须使用动态分发。
    //编译器无法知晓所有可能用于 trait 对象代码的类型，
    //所以它也不知道应该调用哪个类型的哪个方法实现。
    //为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。
    //动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。
    //尽管在编写示例 17-5 和可以支持示例 17-9 中的代码的过程中确实获得了额外的灵活性，但仍然需要权衡取舍。
}

impl Screen{
    ///模拟屏幕绘制
    pub fn run(&self){
        // for i in 0..self.components.len(){
        //     self.components[i].draw();
        // }
        for component in self.components.iter(){
            component.draw();
        }
    }

    ///创建一个button
    pub fn create_button(width:u32,height:u32,label:String)->Button{
        return Button{
            _width:width,
            _height:height,
            _label:label,
        }
    }

    ///创建一个selec box
    pub fn create_select_box(width:u32,height:u32,options:Vec<String>)->SelectBox{
        return SelectBox {
             _width: width, 
             _height: height, 
             _options: options 
        }
    }
}

///屏幕类型实例
// pub struct Screen<T:Draw>{
//     ///Draw组件集合
//     pub components : Vec<T>,
// }

// impl<T> Screen<T> where T:Draw{
//     ///模拟屏幕绘制
//     pub fn run(&self){
//         // for i in 0..self.components.len(){
//         //     self.components[i].draw();
//         // }
//         for component in self.components.iter(){
//             component.draw();
//         }
//     }

//     ///创建一个button
//     pub fn create_button(width:u32,height:u32,label:String)->Button{
//         return Button{
//             _width:width,
//             _height:height,
//             _label:label,
//         }
//     }

//     ///创建一个selec box
//     pub fn create_select_box(width:u32,height:u32,options:Vec<String>)->SelectBox{
//         return SelectBox {
//              _width: width, 
//              _height: height, 
//              _options: options 
//         }
//     }
// }


pub struct Button{
    pub _width:u32,
    pub _height:u32,
    pub _label:String,
}

impl Draw for Button{
    fn draw(& self) {
        println!("button draw...");
    }
}

//另一个使用gui的crate中在SelectBox上实现Draw trait
// use gui.Draw;
pub struct SelectBox {
    _width: u32,
    _height: u32,
    _options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("selec box drawing...");
    }
}