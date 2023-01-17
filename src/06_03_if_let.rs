
fn ch_06_03_enter() {
    let config_max : Option<u8> = Option::Some(3u8);
    // match config_max {
    //     //匹配max，max被绑定为Some中的值
    //     Some(max ) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    //下面的代码等价于上面
    //if let语法通过等号分隔模式和表达式，这里的表达式对应match的第一个分支
    //模式不匹配时if中的代码将不会执行
    //这样可以减少模式匹配的代码量，只处理你想要进行模式匹配的代码
    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max);
    }else{
        //可以在if let中带一个else，这表示模式匹配中_部分的代码，即所有指定模式不匹配时的默认分支
        println!("else!");
    }   
}
 