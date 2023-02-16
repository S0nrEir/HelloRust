use std::io;

pub fn enter(){
    let result_of_fib = fibonacci(10);
    println!("res is :{result_of_fib}");
    return;
    
    ch_03_01();
    ch_03_02();
    ch_03_03();
    ch_03_04();
    ch_03_05();
}


//斐波那契数列
//1，1，2，3，5，8，13，21，34，55，89...
//从第三项开始，每一项都等于前两项之和
fn fibonacci(level:i32)->i32{
    // println!("num is:{level}");
    // if level < 3{
    //     1
    // }else {
    //     fibonacci(level - 1) + fibonacci(level - 2)
    // }

    // if level < 1{
    //     return 0;
    // }

    if level <= 2{
        // 1
        return 1;
    }
    fibonacci(level - 1) + fibonacci(level - 2)
}

fn ch_03_05(){
    //Rust中的if不需要加括号
    let mut number:i32 = 5;
    //但if后的花括号是必须要加的 
    if number < 3{
        println!("number less than 3");
    }else{
        println!("number equal or greater than 3");
    }

    //也可以在let语句的右侧使用它
    //类似于三元运算符 var x = contition ? 1 : 0;
    let contition:bool = true;
    let x = if contition {1} else {0};

    //Rust的循环分为三种
    //loop表示一个死循环直到你主动跳出循环，比如break
    //它可以重复做一些操作和检查，比如检查线程是否完成了任务
    let mut counter:i32 = 0;
    let count_lmt:i32 = 100;
    let mut result:i32 = 0;
    //下面两段代码是等价的
    // let result = loop{
    //     if counter >= count_lmt{
    //         //在Rust中，break和返回值都可以中断循环
    //         //使用return可以将loop的结果返回给result
    //         return counter;
    //     }
    //     counter += 1;
    // };

    loop{
        if counter >= count_lmt{
            result = counter;
            break;
        }
        counter += 1;
    }

    //使用break可以中断指定标签的循环
    // counter = 0;
    //这是一个标签为"counting_up"的循环
    //这段代码
    'counting_up:loop
    {
        let mut remaining = 10;
        loop
        {
            if remaining == 9{
                break;
            }
            if counter == 2{
                //中断标签为"counting_up"的循环，也就是最外层循环
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }

    //还有一种循环是while
    //和大多数语言一样，true继续执行，否则退出，通过break,continue,if等关键字控制流程
    number = 10;
    while number >=0
    {
        println!("number is:{number}");
        number-=1;
    }

    //这是一个遍历数组的例子
    let arr:[i32;5] = [10,20,30,40,50];
    let mut index = 0;
    let len = arr.len();
    while index < len
    {
        println!("arr[index]");
        index+=1;
    }

    //另外一个例子是for循环
    //这有点类似于foreach循环
    for element in arr{
        println!("the current value of arr is {element}");
    }

    //也可以使用范围表达式来指定要遍历的集合范围
    for num in (1..4).rev(){
        println!("num is {num}");
    }
}

fn ch_03_04(){
    //注释，没啥好说的
}

fn ch_03_03(){
    //Rust中所有的函数和变量遵循snake_case规则，即全部小写，以下划线分隔
    //fn关键字声明新函数
    let result = add_two_number(1,1);
    println!("reuslt of function add(1,1) is {result}");
}

//带有一个i32类型参数的函数
fn another_function(x:i32){
    println!("another_function,x->{x}");   
}

//***Rust比较特殊的一点是，函数的返回值等同于函数体最后一个表达式的值
//带有一个u32参数，返回值类型为i32的函数
fn yet_another_function(val:i32)->i32{
    //下面两种方式都可以作为返回值
    //val;//将参数val作为返回值
    5//整型5作为返回值
}

//使用return关键字作为返回值的函数
fn yet_another_function_return_with_bool(val:i32)->bool{
    return false;
}

fn add_two_number(x:i32, y:i32)->i32{
    return x + y;
}

fn ch_03_02(){
    //Rust的数据类型分为两种子集：标量和复合
    //Rust和C++一样，是静态类型语言，在编译时必须知道变量的类型
    //通常情况下，根据值和使用方式，编译器可以推断出想要使用的类型，但在某些情况下必须注明，比如多种类型均有可能时（使用parse函数将String转换为数字时）
    let number :u32 = "42".parse().expect("cast faild");
    //标量类型代表一个单独的值，整型、浮点、布尔、字符
    let f : bool = false;//一个bool类型
    //***注意，Rust的Char类型大小为四字节，这可以让他比ASCII表示更多的内容，包括汉字、日文、韩文、甚至是Emoji等

    //复合类型：元组和数组
    //元组长度固定，一旦声明不可改变
    let tup : (i32,f32,String) = (400, 10.0 , String::from("tuple"));//声明一个元组
    //为了获取元组中的值，可以使用模式匹配来解构，像这样
    // let (x,y,z) = tup;
    // println!("x = {x},y = {y},z = {z}");
    //也可以直接使用.来访问元组的成员
    let test : i32 = tup.0;
    let test : f32 = tup.1;
    let test : &String = &tup.2;
    println!("tup.0 = {},tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);
    
    //Rust中，数组长度是固定的
    let arr = [1,2,3,4,5];
    let another_arr:[i32;5]=[1,2,3,4,5];//主动声明了类型和大小的数组
    //访问数组元素
    let first = arr[0];
    let second = arr[1];
    //不同于C#，Rust中数组存储在Stack上
}

fn ch_03_01(){
    //声明常量使用const关键字，而非不带mut的let，并且必须注明值类型
    //const可以在任何地方被声明，包括全局作用域
    //Rust对常量的命名建议规范是全大写，下划线分隔
    const MY_CONST:i32 = 10;

    //Rust允许“隐藏”变量，此时任何使用该变量的地方都会视为使用隐藏后的变量，直到他的作用域结束
    //隐藏和mut是有区别的，比如没有使用let关键字时会导致编译错误
    //另外，当再次使用let时，实际上是创建了一个新变量
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
    }
    //这里会导致编译报错，不能改变变量类型(mut)
    // let mut spaces = "     ";
    // spaces = spaces.len();

}