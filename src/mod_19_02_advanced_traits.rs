use std::ops;
use rand::rngs::adapter;
use RustBook_Instances_CN::tools::{self, print_line};

pub fn enter(){
    default_generic_param_and_operator_overloading();
    print_line();
    fully_qualified_syntax();
    print_line();
}

//这是一个迭代器的trait
//他有一个叫做Item的关联类型
///迭代器
pub trait Iterator{
    //Item是一个占位符类型
    //trait的实现者会指定Item的具体类型
    //他看起来像是泛型，但并不是
    type Returned_U32;
    fn next(&mut self)->Option<Self::Returned_U32>;
}

struct Counter{
}

//一个实现了Iterator trait的结构体
//它的语法和泛型很像：
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }
//他们的区别在于：如果使用泛型，则要在每一个实现显示声明T的类型
//关联类型则不用，因为不能多次实现这个trait
//关联类型通常有一个描述类型用途的名字，并且在 API 文档中为关联类型编写文档是一个最佳实践。
impl Iterator for Counter{
    type Returned_U32 = u32;
    // type Item = u32;
    fn next(&mut self)->Option<Self::Returned_U32>{
        return Some(1);
    }
}

/// 默认泛型参数和运算符重载
fn default_generic_param_and_operator_overloading(){
    //Rust 并不允许创建自定义运算符或重载任意运算符，
    //不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载。
    let p1 = Point{x:1,y:0};
    let p2 = Point{x:2,y:3};
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    println!("p1 + p2 = {:?}",p1+p2);
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Point{
    x:i32,
    y:i32,
}

//实现Add trait
//默认泛型类型位于Add trait中
//尖括号中的部分叫做默认泛型参数，用于定义add方法中的rhs参数
//如果实现trait的时候没有指定泛型参数的类型，那么默认泛型参数就会被使用
// trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

impl ops::Add for Point{
    type Output = Point;
    
    fn add(self,other:Point)->Point{
        return Point{
            x:self.x+other.x,
            y:self.y+other.y,
        };
    }
}

struct Millimeters(u32);
struct  Meters(u32);

//这是一个不使用默认泛型参数的例子
impl ops::Add for Millimeters{
    type Output = Millimeters;
    fn add(self,other:Millimeters)->Millimeters{
        return Millimeters(self.0+other.0);
    }
}

///完全限定语法
fn fully_qualified_syntax(){
    //Pilot和Wizard都有fly方法
    //Human也有自己的fly实现
    //当调用的时候，rust默认会调用本类型的fly
    let human = Human;
    human.fly();
    
    //在方法名前指定trait的名称可以让rust明确要调用哪个fly
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();//等同于Human::fly(&person);
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
