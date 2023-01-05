use std::io;

//rust定义结构体，和别的语言类似，使用struct关键字即可。
//比如下面这样的

struct User{
    active:bool,
    user_name:String,
    email:String,
    sign_in_count:u64,
}

fn ch_05_01_enter(){
    //通过这样的方式来初始化并且声明一个struct
    //类似于C#的模式匹配语法
    let mut user = User{
        active : false,
        email : string::from("123@xxx.com"),
        user_name:String::from("UserName"),
        sign_in_count:1,
    };

    //使用域运算符"."来访问结构体中的某个成员
    //注意要改变结构体中成员的值，要求它必须是可变的,Rust不允许仅将某个字段标记位可变
    user.active = true;
    user.email = String::from("456@xxx.com");

    //使用实例创建函数生成并且获取一个结构体实例
    let mut user_2 = gen_user("789@xxx.com","user_name");

    //利用别的struct创建一个新的struct
    let user_3 = User{
        bool:user_2.active,
        email:user.email,
        user_name:user_2.email,
        sign_in_count:user.sign_in_count,
    };

    //但也可以使用“结构体更新语法”来实现上述目的，这样可以使得代码更简洁
    //..语法制定了剩余未显式指定的字段均使用user_3的字段值
    let user_4 = User{
        active:false,
        ..user_3,
    };

    tuple_struct();
}

//另外也可以使用元组结构体，像下面这样
struct Color (i32,i32,i32);
struct Point(i32,i32,i32);

//这是没有任何字段的结构体，被称为类单元结构体，常被用于trait
struct AlwaysEqual;

//元组结构体没有字段名，只有类型
fn tuple_struct(){
    let black = Color(0,0,0);
    let point = Point(0,0,0);

    //要访问元组结构体的成员，和普通元组一样
    println!("black.color = {black.1},{black.2},{black.3}");

    //
}

//生成一个User实例的函数
fn gen_user(email:String,user_name:string)->User{
    User{
        active:true,
        email:email,
        user_name:user_name,
        sign_in_count:0,
    }
    // let mut user = User{
    //     active:true,
    //     email:email,
    //     user_name:user_name,
    //     sign_in_count:0,
    // };
    // return user;
}