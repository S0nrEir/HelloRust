use std::{io, string};
use std::collections::{HashMap, HashSet};

//关于更多的Rust集合，可以看https://doc.rust-lang.org/std/collections/index.html

enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn enter(){
    ch_08_01_enter();
    ch_08_02_enter();
    ch_08_03_enter();
}

fn ch_08_01_enter(){
    //vector允许在单独的数据结构中储存多个值，注意***他们在内存中的地址是连续的***
    //创建一个vector
    let mut v:Vec<i32> = Vec::new();
    let v_temp = vec![1,2,3];//直接使用vec!宏来创建一个类型为Vec<i32>并且保存了元素的集合,Rust会自动推断其类型
    
    //向集合中添加元素
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    //移除下标位置len - 1的元素
    v.remove(v.len() - 1);

    let last : usize = v.len() - 1;
    //读vec中的元素有两种方式，一种是通过&来进行索引或使用get()方法
    //get()会返回一个Option<&T>类型
    let third:&i32 = &v[last];
    let third_temp:Option<&i32> = v.get(last);

    match third_temp {
        Some(third_temp) => println!("the 3rd elem is {}",third_temp),
        None => println!("there is no 3rd elem!"),
    }

    //另外注意，一旦程序获取了一个有效引用，rust的借用检查器会执行所有权和借用规则来保证vector内容的这个引用
    //和任何其他引用保持有效（参阅相同作用域中同时存在可变和不可变引用的规则）。
    //例如，下面的代码是无法通过编译的：

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");

    // 为什么第一个元素的引用会关心 vector 结尾的变化？
    // 不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，
    // 在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
    // 这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

    //遍历vector
    for i in &v{
        println!("{}",i);
    }

    //也可以在遍历中改变集合内的元素，前提是集合是可变(mut)的
    let mut v_1 = vec![1,2,3,4,5];
    for i in &mut v_1{
        //要修改可变引用指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
        *i += 100;
        println!("{}",i);
    }

    //可以使用枚举来存储多种类型
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.1415926),
        SpreadSheetCell::Text(String::from("text")),
    ];

    let sheet_cell = &row[0];
    match sheet_cell {
        SpreadSheetCell::Int(t) => println!("t"),
        _=>println!("none")
    }

    //Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。
    //第二个好处是可以准确的知道这个 vector 中允许什么类型。
    //如果 Rust 允许 vector 存放任意类型，那么当对 vector 元素执行操作时一个或多个类型的值就有可能会造成错误。
    //使用枚举外加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况，正如第六章讲到的那样。
}

fn ch_08_02_enter(){
    //对于Rust中的String，其实是一组字节(bytes)的集合
    //很多 Vec 可用的操作在 String 中同样可用，事实上 String 被实现为一个带有一些额外保证、限制和功能的字节 vector 的封装。
    //其中一个同样作用于 Vec<T> 和 String 方法的例子是用来新建一个实例的 new 方法
    let str = String::new();
    let data = "initial contents";
    let mut s_9 = data.to_string();
    let mut s_8 = "initial ccontents".to_string();
    //这声明并赋值了一个空string，接着可以向其中装载数据
    //注意，Rust中的String默认是以UTF-8编码的

    //使用push_str和push进行附加
    let mut s_7 = String::from("foo");
    s_7.push_str("bar");//"foobar"
    
    //push附加char
    let mut s_6 = String::from("lo");
    s_6.push('l');//"lol"

    //使用+或format!宏进行拼接
    s_6 = String::from("hello,");
    s_7 = String::from("world!");
    //s_6.add(&s_7);
    s_8 = s_6 + &s_7;//s_6被移动了，不能继续使用，这是因为+使用了add方法fn add(self, s: &str) -> String

    //使用format!宏
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    println!("{}",format!("{tic}-{tac}-{toe}"));

    //在Rust中，无法使用索引语法来访问string中的字符，下面的代码将无法通过编译：
    // let s1 = String::from("hello");
    // let h = s1[0];
    //这是因为Rust不支持索引string

    //注意，***Rust中的String实际上是一个Vec<u8>组成的集合，其中存储的是字节而非char***

    //在Rust中这会返回hello的前四个字节，对应到string实际上是"Зд""
    // let hello = "Здравствуйте";
    // let s = &hello[0..4];

    //可以使用chars()将string变为char集合再做处理
    s_8 = String::from("Здравствуйте");
    for i in s_8.chars(){
        println!("{}",i);
    }
}

fn ch_08_03_enter(){
    //使用new创建一个空的HashMap并向其中插入数据
    //注意要先use标准库集合中的HashMap
    let mut scores : HashMap<String, u32>  = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    //访问其中的值
    //get方法返回一个Option<&V>,如果取不到相应的key，则返回None，copied方法返回一个Option<u32>而非Option<&u32>
    //并且通过调用unwrap_or()来保证取不到对应的key时将返回值设置为0
    let blue_score = scores.get(&String::from("blue")).copied().unwrap_or(0);
    output_hash_map(&scores);

    //注意***对于i32这类实现了copy trait的类型，在插入HashMap时，其值将被拷贝***
    //而对于string这类拥有所有权的类型，他的值将被移动到HashMap中，HashMap会成为他们的所有者，看看如下代码编译后会报什么错
    //这是因为，filed_name变量失去了其值的所有权，因为他们被移到了HashMap中，使用引用将不会发生这样的问题，
    //但必须保证这些引用指向的值至少在HashMap有效时也是有效的
    // let field_name = String::from("field_name");
    // let field_value = String::from("field_value");
    // let mut map:HashMap<String,String> = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{}",field_name);

    //有效代码，可以通过编译
    // let field_name = String::from("field_name");
    // let field_value = String::from("field_value");
    // let mut map:HashMap<&String,&String> = HashMap::new();
    // map.insert(&field_name, &field_value);
    // println!("{}",field_name);
    
    //直接使用insert可以覆盖已有项
    scores.insert(String::from("blue"), 100);
    let value =  scores.get(&String::from("blue")).copied();
    println!("after cover : {}",value.unwrap_or(0));

    //使用Entry可以只在没有对应key时插入键值对
    //entry方法返回一个Entry类型，它描述了“可能存在也可能不存在”的值
    scores.entry(String::from("blue")).or_insert(100);
    scores.entry(String::from("yellow")).or_insert(100);
    scores.entry(String::from("green")).or_insert(500);//只有这行才会起作用因为，前两个都已经存在相应的键值对
    output_hash_map(&scores);
    
    //根据旧值更新一个值
    let text = String::from("hello world wonderful world");
    let mut map:HashMap<String,u32> = HashMap::new();
    // let mut count:&u32;
    for word in text.split_whitespace(){
        let count = map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }
}

///打印一个HashSet的所有元素
fn output_hash_map(hash_set : &HashMap<String,u32>){
    //遍历
    //使用一个元组来作为索引项
    for (k,v) in hash_set{
        println!("k={},v={}",k,v);
    }
}

// 1.给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数

// 2.将字符串转换为 Pig Latin，
//也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。
//元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！

// 3.使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
//例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
//或者公司每个部门的所有员工按照字典序排列的列表。