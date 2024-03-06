use std::thread;
use std::time::Duration;

pub fn enter(){
    // fn_();
    _fn();
}

fn fn_(){
    // thread::spawn(thread_func);
    let handler = thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the spawned thread",i);
            //sleep强制线程停止一段时间
            thread::sleep(Duration::from_secs_f32(0.5));
        }
    });

    //主线程结束时，新开线程也会结束，不管其是否执行完毕
    for i in 0..5{
        println!("hi number {} from the main thread",i);
        thread::sleep(Duration::from_secs_f32(0.5));
    }
    
    //spawn返回一个JoinHandle，调用其join函数来保证自己所属的线程在主线程退出前跑完
    //handler会阻塞当前线程以保证它所属的线程执行完毕
    //换句话说，它阻塞了主线程
    handler.join().unwrap();
}

fn thread_func(){
    for i in 1..10{
        println!("hi number {} from the spawned thread",i);
        thread::sleep(Duration::from_millis(10));
    }
}

fn _fn(){
    let v = vec![1,2,3];
    //编译报错，闭包使用了v，所以闭包会捕获v使其成为闭包的一部分（转移所有权 ）
    //Rust 会 推断 如何捕获 v，因为 println! 只需要 v 的引用，闭包尝试借用 v。
    //然而这有一个问题：Rust 不知道这个新建线程会执行多久，所以无法知晓对 v 的引用是否一直有效。
    // let handler = thread::spawn(||{
    //     println!("heres a vector {:?}",v);
    // });

    //下面的情况可能使得v的不再有效
    // let handler = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    // drop(v);

    //使用move关键字转移v的所有权至闭包内
    let handler = thread::spawn( move || {
        println!("here's a vector: {:?}", v);
        drop(v);
    });

    handler.join().unwrap();
}