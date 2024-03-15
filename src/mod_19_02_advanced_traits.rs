use std::ops;
use RustBook_Instances_CN::tools::print_line;
use std::fmt;

pub fn enter(){
    default_generic_param_and_operator_overloading();
    print_line();
    fully_qualified_syntax();
    print_line();
    super_trait();
    print_line();
    new_type();
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

    //这里又有一个问题，当使用一个trait的非方法函数时，该怎么办呢
    //Dog本身带一个名叫baby_name的非函数方法，并且实现了animal的同名方法
    println!("a baby dog is called a {}",Dog::baby_name());
    //可以通过使用完全限定语法来解决这个问题
    //意为：调用Animal在Dog上的实现
    println!("a baby dog is called a {}",<Dog as Animal>::baby_name());
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

fn super_trait(){

}

//有时自己的实现的trait，可能用到了别的trait的功能
//比如OutlinePrint trait需要用到Display trait的to_string
//可以像如下方式一样声明,有点像别的语言里的继承
trait OutlinePrint: fmt::Display{
    fn outline_print(&self){
        let output = self.to_string();
        let len = output.len();
        //repeat方法会返回一个包含指定字符串重复指定次数的新字符串
        //这是str类型的方法
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
// 但如果要使用OutlinePrint trait，必须实现Display trait
// 这是因为OutlinePrint trait使用了Display trait
// 这样会报错
// impl OutlinePrint for Point{
// }
//Point实现了Display trait就可以了
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn new_type(){
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

//如果想要在 Vec<T> 上实现 Display，而孤儿规则阻止我们直接这么做，
//因为 Display trait 和 Vec<T> 都定义于我们的 crate 之外。
//可以创建一个包含 Vec<T> 实例的 Wrapper 结构体
//接着可以如列表 19-23 那样在 Wrapper 上实现 Display 并使用 Vec<T> 的值
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
//Display 的实现使用 self.0 来访问其内部的 Vec<T>，
//因为 Wrapper 是元组结构体而 Vec<T> 是结构体总位于索引 0 的项。
//接着就可以使用 Wrapper 中 Display 的功能了。

//此方法的缺点是，因为 Wrapper 是一个新类型，
//它没有定义于其值之上的方法；必须直接在 Wrapper 上实现 Vec<T> 的所有方法，
//这样就可以代理到self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper。
//如果希望新类型拥有其内部类型的每一个方法，为封装类型实现 Deref trait并返回其内部类型是一种解决方案。
//如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。

