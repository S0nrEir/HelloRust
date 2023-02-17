use std::fmt::format;


//rust中的trait类似于接口
//定义一个Summary trait来请求摘要
//所有实现该trait的类型都要实现具体接口
//trait中可以有多个接口
pub trait Summary {
    fn summarize(&self)->String;
}

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

//实现接口
//impl 【trait名】 for 类型名
impl Summary for NewsArticle{
    fn summarize(&self)->String {
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
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        return format!("{}:{}",self.username,self.content);
    }
}

impl Tweet{
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
    let new_articel = NewsArticle::new(
        String::from("head_line"), 
        String::from("location_"), 
        String::from("author"), 
         String::from("content_"));
}