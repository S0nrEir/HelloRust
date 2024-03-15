use std::string;

//生命周期注解
pub fn enter(){
    // let string_1 = String::from("abcd");
    // let string_2 = "xyz";
    // let result = longest(&string_1, string_2);

    // println!("the longest result str is : {}",result);

    //另一个例子：
    {
        let string_1 = String::from("long string is long");
        {
            let string_2 = String::from("xyz");
            let result = longest(string_1.as_str(), string_2.as_str());
            println!("longest string is : {}",result);
        }
    }
}


//这里会报错
//因为Rust不知道将要返回的引用是指向x还是y的
//因为if块中返回x而else块中返回y
//定义这个方法的时候，并不知道传递给方法的具体值，所以也不知道到底是 if 还是 else 会被执行，
//也不知道传入的引用的具体生命周期
//以及生命周期在方法返回后是否有效
// fn longest(x:&str,y:&str) -> &str{

//     if x.len() > y.len(){
//         return x;
//     }else {
//         return y;
//     }

// }

//生命周期注解语法不改变任何引用的生命周期长短
//它描述了多个引用生命周期的关系
//当指定了泛型生命周期后方法能接受任何声明周期的引用
//'a即声明了一个生命周期
//声明方式类似于泛型
//longest方法定义指定了签名中所有的引用必须有相同的生命周期
//注意：***它的实际含义是 longest 方法返回的引用的生命周期与方法参数所引用的值的生命周期的较小者一致***
//记住通过在方法签名中指定生命周期参数时，并没有改变任何传入值或返回值的生命周期，
//而是指出任何不满足这个约束条件的值都将被借用检查器拒绝。
//当具体引用被传递给longest方法时，'a代表的其实分别是x和y的作用域
fn longest<'a>(x:&'a str,y:&'a str) -> &'a str{

    if x.len() > y.len(){
        return x;
    }else {
        return y;
    }

}

//如果一个方法返回的引用没有指向任何参数，那么它的返回值只能是方法体内部创建的
//但这会导致一个问题：返回值将在方法结束时离开作用域并被Rust清理，这是一个悬垂引用
//就像下面这个方法
// fn longest_<'a>(x:&str,y:&str) -> &'a str{
//     let result = String::from("really long string");
//     return result.as_str();
// }

//这样是可以的写法
//或者返回一个持有生命周期的类型
fn longest_<'a>(x:&'a str,y:&'a str) -> String{
    let result = String::from("really long string");
    return result;
}