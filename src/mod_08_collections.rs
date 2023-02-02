use std::io;

//关于更多的Rust集合，可以看https://doc.rust-lang.org/std/collections/index.html

pub fn enter(){
    ch_08_01_enter();
}


fn ch_08_01_enter(){
    //vector允许在单独的数据结构中储存多个值，注意***他们在内存中的地址是连续的***
    //创建一个vector
    let mut v:Vec<i32> = Vec::new();
    let v_temp = vec![1,2,3];//直接使用vec!宏来创建一个类型为Vec<i32>并且保存了元素的集合,Rust会自动推断其类型
    
    //向集合中添加元素
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    
    //移除下标位置len - 1的元素
    v.remove(v.len() - 1);

    let last : usize = v.len() - 1;
    //读vec中的元素有两种方式，一种是通过&来进行索引或使用get()函数
    //get()会返回一个Option<&T>类型
    let third:&i32 = &v[last];
    let third_temp:Option<&i32> = v.get(last);

    match third_temp {
        Some(third_temp) => println!("the 3rd elem is {}",third_temp),
        None => println!("there is no 3rd elem!"),
    }
}