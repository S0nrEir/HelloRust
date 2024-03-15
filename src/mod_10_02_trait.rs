use std::{fmt::{format, Display,Debug}, string, clone};

//rust中的trait类似于接口
//定义一个Summary trait来请求摘要
//所有实现该trait的类型都要实现具体接口
//trait中可以有多个接口
pub trait Summary 
{
    fn summarize(&self)->String;
    
    //可以为trait实现默认方法，这样每个具体类型就不用对该trait各做一次具体实现
    fn summarize_default(&self)->String
    {
        return String::from("(Read more...)");
    }

    //将一个SummaryTrait作为参数传入
    fn notify(item:&impl Summary)
    {
        println!("breaking news!,{}",item.summarize());
    }
    
    //将trait作为参数传入实际上是一种语法糖，大概形式类似于下面这样
    fn notify__<T:Summary>(item:&T)
    {
        //...
    }

    //+连接两个类型表示：方法要求泛型参数必须同时实现了Summary和Display两种trait
    fn notify_<T:Summary+Display>(item:&T)
    {
        //...
    }

    fn _notify_(item:&(impl Summary+Display))
    {
        //...
    }

    //也可以像C#那样使用where来做泛型约束
    fn _notify__<T,U>(t:&T,u:&U)->i32
    where
        T:Display+Clone,
        U:Clone+Debug,
    {
        0
    }
}

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

//实现接口
//impl 【trait名】 for 类型名
impl Summary for NewsArticle
{
    fn summarize(&self)->String 
    {
        return format!( "{},by {} {}",self.headline,self.author,self.location);
    }
}

impl NewsArticle{
    fn new(headline_:String, location_:String, author_:String, content_:String)->Self{
            return Self { 
                    headline: headline_, 
                    location: location_,
                    author: author_,
                    content: content_ 
                };
    }

    fn to_string(&self){
        println!("news_articel!");
    }

    //返回一个SummaryTrait
    fn return_summarizable()->impl Summary{

        let tweet = Tweet::new(String::from("hose_ebooks"),
        String::from("of course,as you probably already know,people"),
        false,
        false);

        return tweet;
    }
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        //也可以这样调用已经默认实现的trait
        return format!("tweet from summarize()->{}",self.summarize_default());
    }
}

impl Tweet{

    //将一个SummaryTrait作为参数传入
    // fn notify(item:&impl Summary){
    //     println!("breaking news!{}",item.summarize());
    // }

    fn new(username_:String,content_:String,reply_:bool,retweet_:bool)->Self{
        return Self{
            username:username_,
            content:content_,
            reply:reply_,
            retweet:retweet_
        };
    }
}

pub fn enter(){

    //其他依赖本crate的crate也可以将Summary引入作用域以实现相应的Summarize方法
    //但这里有一个限制，就是当要实现trait的类型位于crate的本地作用域时，才能为其实现trait
    //比如可以为Tweet实现标准库中的Display Trait,也可以在该模块的作用域中为Vec<T>实现SummaryTrait，这是因为SummaryTrait位于本地作用域
    //但不能为外部类型实现外部Trait，比如为Vec<T>实现DisplayTrait
    //这确保了别人编写的代码不会破坏你的代码，反之亦然
    let tweet = Tweet::new(String::from("hose_ebooks"),
            String::from("of course,as you probably already know,people"),
            false,
            false);
    // println!("1 new tweet:{}",tweet.summarize());
    println!("{}",tweet.summarize());
}

struct Pair<T>{
    x:T,
    y:T
}

//普通泛型
impl<T> Pair<T>{
    fn new (x_:T,y_:T)->Self{
        Self {x:x_,y:y_}
    }
}

//加上了trait bound后，对于Pairt<T>类型，他的泛型T必须实现Display和PartialOrder两种trait，才会生成cmp_display方法
impl<T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }  
}