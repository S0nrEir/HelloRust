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

mod front_of_house{
    //模块可以嵌套，就像hosting和serving模块
    //模块内还可以定义一些除函数外的其他东西，比如结构体、枚举、常量等
    mod hosting{
        fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}
