
pub fn enter(){
    _fn();
}

fn _fn(){
    //能匹配任何传递可能值的模式被称为不可反驳（irrefutable）
    //比如let x = 5；因为x可以匹配任何值
    //对某些可能的值进行匹配会失败的模式被称为可反驳的（refutable)
    //比如if let Some(x) = xxx
    //；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。

    //这样的代码无法运行
    // let Some(x) = Some(5);
    // let Some(val) = return_option();

    //可以修改成如下所示
    //不过将不可反驳模式用于if let是没有意义的
    if let x = 5{
        //...
    }

    // if let Some(temp) = xxx{
    //     //...
    // }

    // let val = return_option();
    // println!("{:?}", val);
}

fn return_option()->Option<i32> {
    return Some(5);
}