// 为了实现消息传递并发，Rust 标准库提供了一个 信道（channel）实现。
//信道是一个通用编程概念，表示数据从一个线程发送到另一个线程。
use std::sync::mpsc;
use std::{thread, vec};
use std::time::Duration;
use std::sync::mpsc::Sender;

pub fn enter(){
    // _fn();
    // fn_();
    _fn_();
    // __fn();
}

fn _fn(){
    //cahnnel()创建一个信道，并返回一个元组，第一个是发送方，第二个是接收方
    //mpsc:multiple producer single consumer，多生产者单消费者，即：多个发送发，一个接收方
    let (tx,rx) = mpsc::channel();
    //将发送端move到新线程里，并发送一个string，这样主线程和新线程就可以互相通信了
    thread::spawn(move ||{
        let val = String::from("hi from spawed thread");
        //send会返回一个Result<T,E>
        thread::sleep(Duration::from_secs(5));
        tx.send(val);
    });

    //使用接收端recv方法来接受
    // let recived = rx.recv().unwrap();
    // println!("got: {}",recived);
    //实际情况中，如果unwrap和expect可能会引发panic，导致程序崩溃，所以使用模式匹配检查一下其值
    //recv方法会阻塞当前线程直到从信道中接收一个值
    // if let Ok(recived) = rx.recv(){
    //     println!("got:{}",recived);
    // }

    //try_recv不会阻塞，他会立刻返回一个Result（类似于从信道中被动获取和主动拿取的区别？）
    //如果是Err则表明此时没有任何信息
    // if let Ok(recived) = rx.try_recv(){
    //     println!("got:{}",recived);
    // }

    //等待直到从信道中拿到消息
    // while true {
    loop {
        // if let Ok(recived) = rx.try_recv(){
        //     println!("got:{}",recived);
        //     break;
        // }
        let res = rx.try_recv();
        if let Err(err) = res{
            continue;
        }
        println!("got:{}",res.unwrap());
        break;
    }
}

fn fn_(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let val = String::from("hi from spawned thread");
        tx.send(val).unwrap();
        //将值发送到另一个线程后，那个线程可能会在当前线程再次使用它时将其丢弃或修改
        //这里会报错：send方法会获取val的所有权并将它的所有权转移给接收者
        // println!("{}", val);
    });

    let recived = rx.recv().unwrap();
    println!("got:{}",recived);
}

//并发的例子
fn _fn_(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
        let vals = vec![
          String::from("hi"),
          String::from("from"),
          String::from("the"),
          String::from("thread"),
        ];
        for val in vals{
            tx.send(val);
            thread::sleep(Duration::from_secs(1));
        }
    });//end thread

    //将rx当作一个迭代器，当信道结束时，迭代器也将结束
    for recived in rx{
        println!("got:{}",recived);
    }
}

//通过克隆发送者来创建多个producer
fn __fn(){
    let (tx,rx) = mpsc::channel();
    //使用clone方法返回一个可以传递给新建线程的发送端句柄
    let tx_1 = tx.clone();
    let handler_1 = thread::spawn(move ||{
        let vals = vec![
          String::from("hi"),
          String::from("from"),
          String::from("the"),
          String::from("thread"),
        ];
        for val in vals{
            tx_1.send(val);
            thread::sleep(Duration::from_secs(1));
        }
    });//end of thread

    let tx_2 = tx.clone();
    let handler_2 = thread::spawn(move ||{
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals{
            tx_2.send(val);
            thread::sleep(Duration::from_secs(1));
        }
    });//end of thread
    
    for recived in rx{
        println!("got:{}",recived);
    }
    

    
}

//仅适用于 unsafe 的最后一个操作是访问 联合体 中的字段，
//union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段。
//联合体主要用于和 C 代码中的联合体交互。访问联合体的字段是不安全的
//因为 Rust 无法保证当前存储在联合体实例中数据的类型。可以查看 参考 Rust 文档 了解有关联合体的更多信息。
//https://doc.rust-lang.org/reference/items/unions.html

//通常并不需要手动实现 Send 和 Sync trait，
//因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的。
//因为它们是标记 trait，甚至都不需要实现任何方法。它们只是用来加强并发相关的不可变性的。
//手动实现这些标记 trait 涉及到编写不安全的 Rust 代码，第十九章将会讲述具体的方法；
//当前重要的是，在创建新的由不是 Send 和 Sync 的部分构成的并发类型时需要多加小心，
//以确保维持其安全保证。“The Rustonomicon” 中有更多关于这些保证以及如何维持它们的信息。

//使用 unsafe 来进行这五个操作（超能力）之一是没有问题的，
//甚至是不需要深思熟虑的，不过使得 unsafe 代码正确也实属不易，
//因为编译器不能帮助保证内存安全。当有理由使用 unsafe 代码时，是可以这么做的，
//通过使用显式的 unsafe 标注可以更容易地在错误发生时追踪问题的源头。