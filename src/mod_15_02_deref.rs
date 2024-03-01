use std::ops::Deref;

//必须使用解引用运算符追踪引用所指向的值
// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, y);//错误，不允许比较数字和引用
//     assert_eq!(5, *y);//正确，使用解引用来获取引用指向的值
// }



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
//deref trait的含义是：当使用*对该变量解引用时，返回什么值
impl<T> Deref for MyBox<T>{
    //用于该trait的关联类型
    type Target = T;

    //deref函数借用self并返回一个内部数据的引用
    fn deref(&self) -> &Self::Target {
        //解引用时返回成员的引用
        return &self._x;
    }
}

fn hello(name:&str){
    println!("hello! {name}");
}

fn enter_(){
    //可以使用字符串 slice 作为参数调用 hello 函数，比如 hello("Rust");
    //Deref 强制转换使得用 MyBox<String> 类型值的引用调用 hello 成为可能
    // let m = MyBox::new(String::from("Rust"));//String实现的deref trait返回了&str
    // hello(&m);

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