use crate::mod_05_struct_and_example::Rectangle;

#[cfg(test)]

pub fn run(){
    println!("running test...entry");
    it_works();
}

//这表示函数为一个测试函数
//可以运行cargo test查看测试结果
#[test]
fn it_works(){
    let result = 2+2;
    //宏断言
    assert_eq!(result,4);
}

#[test]
fn exploration(){
    //assert_eq!()和assert_ne!()分别使用==和!=。断言失败时这些断言会使用调试格式打印其参数。
    //这意味被比较的值必须实现PartialEq和Debug这两个trait
    //所有基本类型和大部分标准库类型都实现了这些trait
    //但是对于自定义的struct，需要自己实现PartialEq才能断言其值是否相等或不等。需要实现Debug才能在断言失败时打印其值
    //或者可以在结构体前加derive属性，然后在其标记的类型上生成对应trait的默认实现，如同mod05的Rectangle
    //或这样:
    //#derive[PartialEq,Debug]
    assert_eq!(1+1,2);
}

#[test]
fn it_doesnt_work(){
    assert_eq!(1+1,3);
}

//当发生panic时则测试失败
//每个测试都单独运行在一个新线程，当主线程发现测试线程异常，就会将对应测试标记为失败。
#[test]
fn panic(){
    panic!("make this test fail");
}

//这里用mod05的Rectangle结构体做个例子
#[test]
fn larger_can_hold_smaller(){
    let larger = Rectangle::new(7 ,8);
    let smaller = Rectangle::new(1,5);
    // assert!(larger.can_hold(&smaller));
    assert!(smaller.can_hold(&larger),"bigger! Can't hold!");
}

#[test]
fn ignore_fn(){

}