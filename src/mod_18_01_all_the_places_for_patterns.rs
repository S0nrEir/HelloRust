
pub fn enter(){
    _fn();
    println!("---------");
    fn_();
    println!("---------");
    _fn_();
    println!("---------");
    __fn();
    println!("---------");
}

fn _fn(){
    let favorite_color:Option<&str> = None;
    let is_tuesday = false;
    let age:Result<u8,_> = "34".parse();
    //匹配多个模式的情况，可以任意跟else/else if/else if
    if let Some(color) = favorite_color{
        println!("using your favorite color : {color} as the background");
    }else if is_tuesday{
        println!("tuesday is green day");
    }else if let Ok(age) = age{
        if age > 30{
            println!("using purple as background color");
        }else{
            println!("using orange as backgound color");
        }
    }else{
        println!("using blue as background color");
    }
}

fn fn_(){
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    //只要模式匹配就一直跑while循环
    //pop()返回末尾元素Option<T>，如果没有，则返回none
    //这里尝试匹配pop的返回元素为Some
    while let Some(top_of_stack) = stack.pop(){
        println!("{}",top_of_stack);
    }
}

fn _fn_(){
    let v = vec!['a','b','c'];

    //在for循环中，模式是for关键字直接跟随的值(index,value)
    //enumerate()返回一个迭代器，其包含一个值和在迭代器中的索引
    for (index,value) in v.iter().enumerate(){
        println!("{} is at index {}",value,index);
    }
    // let iter = v.iter().enumerate();
    // iter.next();
}

fn __fn(){

    //使用let语句时，实际上是这样的：let [patterns] = [expression]
    //即：将5绑定到x
    let a = 5;
    //也可以用_来忽略匹配的值
    let (x,y,_) = (1,2,3);
}