use std::ops::Deref;

struct MyBox<T>{
    _x:T,
}

impl<T> MyBox<T>{
    pub fn new(x:T)->MyBox<T>{
        return MyBox{
            _x:x,
        };
    }
}

//为MyBox实现Deref trait才可以正确的为该类型解引用
impl<T> Deref for MyBox<T>{
    //用于该trait的关联类型
    type Target = T;

    //deref函数借用self并返回一个内部数据的引用
    fn deref(&self) -> &Self::Target {
        //解引用时返回成员的引用
        return &self._x;
    }
}

fn enter_(){
    let x = 0x5;
    let y = MyBox::new(x);
    println!("{}",*y);//这里对y解引用，实际上rust做了这件事：*(y.deref())
    let str = MyBox::new(String::from("testing"));
    println!("{}",*str);
}

pub fn enter(){
    enter_();
    // let x = 5;
    // let y = &x;
    // println!("{}",*y);

    //与上例的区别在于，y是一个指向x值拷贝的Box<T>实例，而非指向x值的引用
    // let x = 5;
    // let y = Box::new(x);
    // println!("{}",*y);

    
}