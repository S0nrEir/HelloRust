use std::io;

//slice引用允许你引用集合中一段连续的元素序列，而无需引用整个集合。
//
fn ch_04_03_enter(){

    let s:String = String::from("hello world");
    let &result = first_word(&s);
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
fn first_word(s:&str)->&str{
    let bytes = s.as_bytes();
    for(i,&temp_char) in bytes.iter().enumerate(){
        if temp_char == b' '{
            return &s[0..i];
        }
    }
    &s[..];
}

fn first_word(s:&String)->usize{
    //使用as_bytes()函数将string转化为字节数组
    let bytes = s.as_bytes();
    //在bytes上使用iter创建一个迭代器，iter返回集合中的每一个元素，enumerate函数将元素作为元组返回

    for (i,&temp_char) in bytes.iter().enumerate(){
        if temp_char == b' '{//字节的字面值语法
            return i;
        }
    }
    s.len();
}