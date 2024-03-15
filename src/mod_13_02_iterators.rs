use std::result;

pub fn enter(){

    let temp = TempStruct::new(vec![0x100,0xff,0xfff,0xffff,0xfffff]);
    for iter in temp.iter() {
        println!("elem:{}",iter);
    }
    return;
    let temp:Vec<i32> = Vec::new();
    let v1 = vec![1,2,3];
    //rust中可以直接使用迭代器
    for val in v1.iter(){
        println!("got:{}",val);
    }
    //所有的迭代器都实现了一个叫做Iterator的trait
    //Item 类型被用作 next 方法的返回值类型
    
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // 此处省略了方法的默认实现
// }

}

struct TempStruct{
    arr:Vec<i32>,
}

impl TempStruct {
    pub fn new(arr_:Vec<i32>)->Self{
        let result = Self{
            arr:arr_
        };
        return result;
    }

    pub fn iter(&self)->TempStructIter{
        return TempStructIter::new(self.arr.to_vec());
    }
}

struct TempStructIter{
    _arr:Vec<i32>,
    _curr:usize,
    _first:bool,
    _is_over_len:bool,
}

impl TempStructIter{

    ///gen
    fn new(arr_:Vec<i32>)->Self{
        return TempStructIter{
            _arr:arr_,
            _curr:0,
            _first:true,
            _is_over_len:false
        };
    }
    
    ///重置游标
    fn reset(&mut self){
        self._curr = 0;
    }
}

impl Iterator for TempStructIter{
    //指定返回值类型
    //type Item和Self::Item定义了trait的关联类型
    type Item = i32;

    //调用next方法的方法被称为消费适配器，因为调用他们会“消耗”迭代器
    //
    fn next(&mut self) -> Option<Self::Item> {
        if self._is_over_len{
            return None;
        }

        if self._first{
            self._first = false;
            return Option::Some(self._arr[0]);
        }

        self._curr += 1;
        if self._curr < self._arr.len(){
            return Option::Some(self._arr[self._curr]);
        }
        else {
            self._is_over_len = true;
            return Option::None;
        }
    }

}

#[derive(PartialEq,Debug)]
struct Shoe{
    _size:u32,
    _style:String,
}

///返回所有符合条件尺码的集合
fn shoes_in_size(shoes_:Vec<Shoe>,shoe_size_:u32)->Vec<Shoe>{
    //filter也是一种迭代适配器，它返回一个新的，包含返回值为true的闭包的迭代器
    //然后通过collect收集起来，并且返回给外部调用
    //调用into_iter来创建一个获取 vector 所有权的迭代器
    return shoes_.into_iter().filter(|s| s._size == shoe_size_).collect();
}

#[test]
fn filters_by_size(){

    //testing case
    let shoes = vec![
        Shoe { _size: 10, _style:String::from("sneaker")},
        Shoe { _size: 13, _style:String::from("sandal")},
        Shoe { _size: 10, _style:String::from("boot")},
    ];

    let in_my_size = shoes_in_size(shoes, 10 );
    assert_eq!
        (
            in_my_size,
            vec!
                [
                    Shoe { _size: 10, _style:String::from("sneaker")},
                    Shoe { _size: 10, _style:String::from("boot")},
                ]
        );
}