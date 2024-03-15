use crate::mod_05_struct_and_example::Rectangle;

//Rust中的测试分为两类，分别是单元测试和集成测试
//单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。而集成测试对于你的库来说则完全是外部的。
//它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。


//单元测试与要测试的代码一同放在src目录下相同的文件中，
//在要测试的文件中创建包含测试方法的tests模块，并使用cfg(test)标注模块，就像下面这样

//测试模块的#[cfg(test)]注解告诉Rust只在执行cargo test的时候才编译和运行测试代码
#[cfg(test)]
pub fn run(){
    println!("running test...entry");
    it_works();
}

//这表示方法为一个测试方法
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
    assert!(larger.can_hold(&smaller));
    // assert!(smaller.can_hold(&larger),"bigger! Can't hold!");
}

//可以通过cargo test 【方法名】的方式来运行指定的方法测试
//cargo test 【模块名称】来运行一个模块中的所有测试
//cargo test 【work】 运行所有方法名中包含work的测试
//cargo test -- -- ignored 只运行被忽略的测试
//默认情况Rust会运行所有打了test属性标记的方法测试

//对于想要忽略的测试，在test后添加ignore属性标记即可
#[test]
#[ignore]
fn ignore_fn(){

}