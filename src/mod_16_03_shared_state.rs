use core::num;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::{sync::Mutex};
use std::thread::{self, JoinHandle};
use ::std::sync::Arc;

pub fn enter(){
    // fn_();
    // _fn();
    _fn__();
}

fn fn_(){
    //处理并发的一种方式是消息传递
    //还有一种方式，是共享数据
    
    //互斥器mutex( mutual exclusion)在任意时刻只允许一个线程访问数据
    //要访问互斥器中的数据，线程要先获取互斥器的锁（lock）
    //锁是互斥器数据结构的一部分，它记录了谁有数据的排他访问权（A有数据的访问权，BCD就不能再有）

    //使用互斥器，需要记住：
    //1.在使用前数据前先尝试获取
    //2.处理完被互斥器保护的数据后，手动解锁数据

    //使用关联函数创建一个Mutex<T>
    let mutex = Mutex::new(5);
    {
        //lock()访问锁，以获取其中的数据
        //这会阻塞当前线程，直到拥有锁
        //如果线程A获取了锁，但A发生了panic，则别的线程调用lock会失败，在这种情况下，没有线程能够再获取锁
        // let mut num = mutex.lock().unwrap();
        // *num = 6;

        //这里要把匹配到的MutexGuard声明为可变才行
        // if let Ok(guard) = mutex.lock(){
        //lock返回ok则返回MutexGuard，如果时Error则返回Poison
        if let Ok(mut guard) = mutex.lock(){
            //MutexGuard是一个智能指针，它实现了Deref以解引用访问其内部数据
            //也实现了drop以释放锁（离开scope时）
            //
            *guard = 6;
            //println!("mutex.value = {}",num);
        }else{
            println!("faild to get mutex lock");
        }
    }
    println!("num = {:?}",mutex);
}

fn _fn(){
    // let counter:Mutex<i32> = Mutex::new(0);
    // let mut handlers:Vec<JoinHandle<()>> = vec![];
    // for _ in 0..10{
    //     //这里的问题在于，counter的所有权被放到了上一次遍历过程中的线程里了
    //     //因此这次遍历无法获取counter的所有权
    //     //也即无法将counter锁的所有权移动到多个线程中
    //     let handler = thread::spawn(move ||{
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });//end of thrad

    //     handlers.push(handler);
    // }
}

fn _fn_(){
    //这个例子的问题在于
    //因为Rc<T>的限制，他无法被安全地共享于多线程中
    //当Rc<T>管理引用计数时，它必须在每一个clone调用时+1计数 在克隆被丢弃时计数-1
    //如果改变计数的操作被其他线程打断，则会导致诡异的bug，因此Rc无法用于多线程

    // let counter:Rc<Mutex<i32>> = Rc::new(Mutex::new(0));
    // let mut handlers:Vec<JoinHandle<()>> = vec![];

    // for _ in 0..10{
    //     let counter = Rc::clone(&counter);
    //     let handler = thread::spawn(move ||{
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });//end of thrad

    //     handlers.push(handler);
    // }
}

fn _fn__(){
    //在这种情况下，需要的是一个特性类似于Rc，但能以线程安全的形式改变计数的类型
    //Arc<T>(atomic reference count)就是可以安全用于并发环境的类型
    //它是原子性的，可以在线程间安全地被共享
    //并非所有的类型都是原子性的，这是因为原子性的数据类型会有性能损耗
    //对于单线程场景，无需关心数据共享和并发问题，也就不需要原子性行的数据类型了
    //arc也是一个智能指针

    //Arc<T>的使用方式和Rc<T>一致
    let counter:Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handlers:Vec<JoinHandle<()>> = vec![];

    for _ in 0..10{
        //counter在声明时是不可变的，但可以获取其内部值得可变引用
        //这是因为Mutex<T>提供了内部可变性，就像RefCel
        //也可以使用Mutex<T>改变Arc<T>的内容

        //使用clone获取其包含的Mutex类型的引用
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handler = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });//end of thrad

        handlers.push(handler);
    }

    for handle in handlers{
        handle.join().unwrap();
    }

    println!("result : {}",*counter.lock().unwrap());
}