use std::io;

//定义一个模块，以mod关键字开始，后跟模块名
//这里定义一个名为【前台】的模块

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
    //绝对路径

    //crate根定义了一个名为front_of_house的模块，front_of_house模块不是pub的，但是因为eat_at_restaurant和
    //front_of_house定义在同一模块（兄弟）所以可以在eat_at_restaurant中引用front_of_house
    //可以访问hosting的父模块(front_of_house)，hosting模块也被标记为pub，所以也可以访问hosting
    //add_to_waitlist函数同理
    //
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

    //在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //改变主意更换面包类型
    meal.toast = String::from("wheat");
    println!("meal is :{}",meal.to_string());
    println!("-----------------------");
    println!("i'd like {} toast please",meal.toast);

    //使用枚举
    let soup:back_of_house::Appetizer = back_of_house::Appetizer::Soup;
    let salad:back_of_house::Appetizer = back_of_house::Appetizer::Salad;
    
}

mod front_of_house{

    //模块可以嵌套，就像hosting和serving模块
    //模块内还可以定义一些除函数外的其他东西，比如结构体、枚举、常量等
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("add to waitlist!");
        }
        
        fn seat_at_table(){
            println!("seat at table!");
        }
    }

    mod serving{
        fn take_order(){
            println!("take order!");
        }

        fn serve_order(){
            println!("serve_order");
        }

        fn take_payment(){
            println!("take_payment");
        }
    }
}


fn deliver_order(){
    println!("deliver order!");
}

mod back_of_house{
    // use crate::deliver_order;

    use std::fmt::format;

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
}
