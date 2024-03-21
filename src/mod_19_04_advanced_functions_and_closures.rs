use std::fmt::Display;

use RustBook_Instances_CN::tools;

pub fn enter(){
    function_pointer();
    tools::print_line();
    return_closure();
}

fn add_one(x:i32)->i32{
    return x+1;
}

//在参数里使用fn关键字，指定函数签名，表示这是一个函数指针
//不同于闭包，fn 是一个类型而不是一个 trait，
//所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。
//函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以可以在需要闭包的地方使用函数指针。
fn do_twice(f:fn(i32)->i32,arg:i32)->i32{
    f(arg)+f(arg)
}

#[derive(Debug)]
enum Status{
    Value(u32),
    Stop,
}

// impl Display for Status{
//     fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
//         match self{
//             Status::Value(i)=>write!(f,"Value({})",i),
//             Status::Stop=>write!(f,"Stop"),
//         }
//     }
// }

///函数指针
fn function_pointer(){
    let anwser = do_twice(add_one,5);
    println!("The anwser is: {}",anwser);

    let list_of_numbers = vec![1,2,3];
    // 将一个数字vector转换为字符串vector，可以使用闭包
    let mut list_of_strings:Vec<String> = list_of_numbers.iter().map(|i|{i.to_string()}).collect();
    // println!("list_of_string:{:?}",list_of_strings);
    //也可以使用函数指针来替代闭包    
    // let list_of_numbers = vec![1, 2, 3];
    //这里使用了完全限定语法，指定调用ToString::to_string
    list_of_strings = list_of_numbers.iter().map(ToString::to_string).collect();

    //创建从0到19的Status::Value的vector，这是通过(0u32..20)实现的
    //将集合的每一个值应用于Status::Value构造函数（这是枚举的构造函数）
    let mut list_of_statuses:Vec<Status> = (0u32..20).map(Status::Value).collect();
    list_of_statuses.push(Status::Stop);
    println!("list_of_statuses:");
    for s in list_of_statuses{
        print!("{:?},",s);
    }
    println!()
}

//编译不过
//Rust不知道需要多少空间来存储这个闭包
// fn returns_closure()->dyn Fn(i32)->i32{

// }

//如果想要返回一个闭包的话，可以这样
//用box包起来
fn returns_closure()->Box<dyn Fn(i32)->i32>{
    return Box::new(|x|{
        x + 1}
    );
}

fn return_closure(){
    let closure = returns_closure();
    let var = closure(1);
    println!("call returned closure:{}",var);
}