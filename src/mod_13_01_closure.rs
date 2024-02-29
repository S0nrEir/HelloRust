use std::io;
use std::vec;
use std::thread;
use core::time::Duration;

pub fn enter(){
    sort_by_key();
    return;

    //新线程可能在主线程剩余部分执行完前执行完，或者也可能主线程先执行完。
    //如果主线程维护了 list 的所有权但却在新线程之前结束并且丢弃了 list，则在线程中的不可变引用将失效。
    //因此，编译器要求 list 被移动到在新线程中运行的闭包中，这样引用就是有效的
    //如果希望强制闭包获取它用到的环境中值的所有权，可以在参数列表前使用 move 关键字，就像下面这样
    let list = vec![1,2,3];
    println!("before defining closure:{:?}",list);
    thread::spawn(move || {println!("from thread:{:?}",list);})
        .join()
        .unwrap();

    return;

    // let store = Inventory{
    //     shirts:vec![
    //         ShirtColor::Blue,
    //         ShirtColor::Red,
    //         ShirtColor::Blue,
    //     ]
    // };
    
    let store = Inventory::new(&vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
    ]);

    //将一个闭包函数放到一个变量里
    let expensive_closure = |num:u32|->u32{//也可以注明参数类型
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        return num;
    };
    //更多的闭包例子：
    let add_one_v2 = |x:u32|->u32{ x+1 };
    //在正确的上下文中，下面两个闭包函数的参数和返回值类型将被rust推断出来，所以不用注明
    // let add_one_v3 = |x| { return x+1 };
    // let add_one_v4 = |x| x+1;

    let user_pref_1 = Some(ShirtColor::Red);
    let give_away_1 = store.give_away(user_pref_1);
    println!("the user with preference {:?} gets {:?}",user_pref_1,give_away_1);

    let user_pref_2 = None;
    let give_away_2 = store.give_away(user_pref_2);
    println!("the user with preference {:?} gets {:?}",user_pref_2,give_away_2);
}

#[derive(Debug,PartialEq,Clone,Copy)]
enum ShirtColor{
    Red,
    Blue,
}

struct Inventory{
    shirts : Vec<ShirtColor>,
}

impl Inventory{

    pub fn new(colors_:&Vec<ShirtColor>)->Self{
        let mut store = Inventory{
            shirts : vec![],
        };
        
        for color in colors_{
            store.shirts.push(*color);
        }

        return store;
    }

    pub fn give_away(&self,user_preference:Option<ShirtColor>)->ShirtColor{
        //Option的unwrap_or_else函数，如果Option的成员是Some则返回Some<T>
        //如果是None，则调用匿名函数并返回匿名函数中的值
        //匿名函数的签名是()->T，这是由Option决定的
        return user_preference.unwrap_or_else(|| self.most_stocked());
        //Rust中的闭包形式是：|参数1，参数2，参数3，...|{函数体}
        //对于单行函数，也可以不加花括号，比如|参数1，参数2，...| 函数体
    }

    fn most_stocked_()->ShirtColor{
        return ShirtColor::Blue;
    }


    ///哪个多给哪个    
    fn most_stocked(&self)->ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts{
            match color{
                ShirtColor::Red=> num_red += 1 ,
                ShirtColor::Blue=> num_blue += 1,
            }//end match
        }//end for

        if num_red > num_blue{
            return ShirtColor::Red;
        }else{
            return ShirtColor::Blue;
        }
    }
}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn sort_by_key(){
    let mut list = [
        Rectangle{width:10,height:1},
        Rectangle{width:3,height:5},
        Rectangle{width:7,height:12},
    ];

    //闭包捕获：捕获闭包上下文的值到闭包中,
    
    //闭包排序
    //按照widht从小到大排序
    //sort_by_key需要接受一个FnMut闭包的原因是他会多次调用该闭包
    //每个slice中的元素调用一次
    list.sort_by_key(|r|{return r.width;});

    //该代码尝试在闭包的环境中向 sort_operations vector 放入 value— 一个 String 来实现计数。
    //闭包捕获了 value 然后通过转移 value 的所有权的方式将其移出闭包给到 sort_operations vector。
    //这个闭包可以被调用一次，尝试再次调用它将报错。因为这时 value 已经不在闭包的环境中，无法被再次放到 sort_operations 中
    //因而，这个闭包只实现了 FnOnce。由于要求闭包必须实现 FnMut，因此尝试编译这个代码将得到报错：value 不能被移出闭包：
    
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
//--------------------------------------------------------------------------------------------------
    //要修复这里，我们需要改变闭包体让它不将值移出环境。
    //在环境中保持一个计数器并在闭包体中增加它的值是计算 sort_by_key 被调用次数的一个更简单直接的方法。
    //示例 13-9 中的闭包可以在 sort_by_key 中使用，因为它只捕获了 num_sort_operations 计数器的可变引用，
    //这就可以被调用多次。

    // let mut num_sort_operations = 0;
    // list.sort_by_key(|r| {
    //     num_sort_operations += 1;
    //     r.width
    // });
    //三种闭包类型（实际上是trait）：
    //FnOnce:调用一次，将捕获值移出闭包
    //FnMut:调用多次，不将捕获值移出闭包，修改捕获值
    //Fn:不将捕获值移出闭包，不修改捕获值，也包括不从环境中捕获变量的闭包

    println!("{:?}",list);
}