//模块可以嵌套，就像hosting和serving模块
//模块内还可以定义一些除函数外的其他东西，比如结构体、枚举、常量等
pub fn add_to_waitlist(){
    println!("add to waitlist!");
}

fn seat_at_table(){
    println!("seat at table!");
}