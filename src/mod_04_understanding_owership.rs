
pub fn enter(){
    ch_04_enter();
    ch_04_03_enter();
}

//slice引用允许你引用集合中一段连续的元素序列，而无需引用整个集合。
//
fn ch_04_03_enter(){

    let s:String = String::from("hello world");
    let len = first_word(&s);
    println!("len of first word of s is {}",len);
    return;

    //字符串slice：string中一部分值的引用
    let hello = &s[0..5];
    let world = &s[6..11];
    //如果slice包含string的最后一个字节，也可以舍去尾部数字，比如下面这样
    let slice_1 = &s[0..];
    //让索引从0开始
    let slice_2 = &s[..len];
    //当然也有一些其他类型的slice
    let a = [1,2,3,4,5,6,7];
    let slice_3 = &a[1..3];
    //以上slice的类型是&[i32]
}

//***这里注意，字符串slice的类型声明写作&str
fn first_word_with_str(s:&str)->&str{
    let bytes = s.as_bytes();
    for(i,&temp_char) in bytes.iter().enumerate(){
        if temp_char == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}

fn first_word(s:&String)->usize{
    //使用as_bytes()函数将string转化为字节数组
    let bytes = s.as_bytes();
    //在bytes上使用iter创建一个迭代器，iter返回集合中的每一个元素，enumerate函数将元素作为元组返回

    for (i,&temp_char) in bytes.iter().enumerate(){
        if temp_char == b' '{//字节的字面值语法，如果temp_char为空格
            return i;
        }
    }
    return s.len(); 
}

fn ch_04_enter(){
    //变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，
    //其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

    //因为Rust的特性，所以像下边这样在do_something函数后，想要再次使用变量s1将会导致一个编译器错误
    //这是因为s1的值移动到函数内部后，在离开函数时，将会被释放，因此在调用do_something函数后使用s1变量在Rust中是不被允许的
    // let s1 = String::from("hello");
    // println!("s1 is :{s1}");

    
    //为了解决这个问题，可以使用引用，像下面这样
    //从Rust的角度来讲，这并不会转移s1的所有权到函数内部，但却可以访问s1所持有的内存地址和其数据
    //注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    //当变量离开作用域时，rust将自动调用一个叫做drop的特殊函数释放该变量持有的内存
    let mut s1 = String::from("reference");
    do_something_ref(&mut s1);
    println!("s1 is {s1}");
    
    //==================================================================================
    //关于引用的总结
    //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //引用必须总是有效的。
    //==================================================================================
}

fn do_something(str:String){
    println!("str is :{str}");
}

//函数的参数传入一个string的引用
//如果想要改变引用的值，则需要加上mut关键字，在变量声明和函数签名处都是如此，这被称为可变引用。
fn do_something_ref(str:&mut String){
    println!("str is {str}");
}