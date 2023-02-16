use std::io;

pub fn enter(){
    ch_05_01_enter();
    ch_05_02_enter();
}

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
        email : String::from("123@xxx.com"),
        user_name:String::from("UserName"),
        sign_in_count:1,
    };

    //使用域运算符"."来访问结构体中的某个成员
    //注意要改变结构体中成员的值，要求它必须是可变的,Rust不允许仅将某个字段标记位可变
    user.active = true;
    user.email = String::from("456@xxx.com");

    //使用实例创建函数生成并且获取一个结构体实例
    let mut user_2 = gen_user(String::from("789@xxx.com"),String::from("user_name"));

    //利用别的struct创建一个新的struct
    let user_3 = User{
        active:user_2.active,
        email:user.email,
        user_name:user_2.email,
        sign_in_count:user.sign_in_count,
    };

    //但也可以使用“结构体更新语法”来实现上述目的，这样可以使得代码更简洁
    //..语法制定了剩余未显式指定的字段均使用user_3的字段值
    let user_4:User = User{
        active:false,
        ..user_3
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
    let black:Color = Color(0,0,0);
    let point:Point = Point(0,0,0);

    //要访问元组结构体的成员，和普通元组一样
    println!("black.color = {},{},{}",black.0,black.1,black.2);
    //
}

//生成一个User实例的函数
fn gen_user(email:String , user_name:String)->User{
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

// use std::io;

//为了让结构体能够使用debug模式输出信息，使用derive属性来指定结构体使用Debug这种trait
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

//在Rectangle结构体上定义area方法
//使用impl关键词 可以使impl块中的所有内容都与Rectangle类型关联
//这样的话就可以调用struct的area方法了
//在impl块中，self是impl块的别名，方法的第一个参数必须有一个名为self的Self类型参数
//所有在impl块中定义的函数都叫做关联函数，因为他们与impl后面的类型相关联
impl Rectangle{

    //计算自身面积
    fn area(&self)->u32{
        return self.width * self.height;
    }

    //计算自身是否能容纳另一个Rectangle
    fn can_hold(&self,other:&Rectangle)->bool{
        // return self.area() >= other.area();
        return self.width >= other.width && self.height >= other.height;
    }

    fn can_hold_area(&self,other:&Rectangle)->bool{
        return self.area() >= other.area();
    }
    
    //也可以定义第一个参数不为self的函数
    //关键字Self指代impl后跟的类型
    //使用::语法来调用这个关联函数，这表示，这个函数位于结构体的命名空间中，::用于关联函数和模块创建的命名空间
    fn square(size:u32)->Self{
        return Self { 
                width: size,
                height: size 
            };
    }
}

//每个结构体都允许拥有多个impl块
impl Rectangle{
    fn print_self(&self){
        println!("width:{},height:{}",self.width,self.height);
    }
}

fn ch_05_02_enter(){
    rectangles();
}

fn rectangles(){
    let width:u32 = 50;
    let height:u32 = 50;
    // println!("the area is {}",area(width,height));
    let rect : (u32,u32)= (30,30);
    // println!("the area is {}",area_dimension(rect));
    let rect_stct:Rectangle = Rectangle{
        width:30,
        height:30
    };
    //在不实现trait的情况下，以下代码是无法通过编译的
    //这是因为，{}默认告诉println!使用被称为Display的格式，大部分的基本类型都实现了Display trait
    //但对于自定义的结构体，需要自己去实现
    // println!("the area is {}",area_struct(&rect_stct));
    //{:?}代表Debug输出格式，它会将结构体内的值全都打印出来，这也是一种trait
    //另外也可以使用{:#?}这种格式，它的打印信息将更加详细
    //println!("the area is {:?}",rect_stct);

    // println!("the area is {}",rect_stct.area());

    let rect_test_1:Rectangle = Rectangle {
        width:30,
        height:30
    };

    let rect_test_2:Rectangle = Rectangle {
        width:10,
        height:40
    };

    let rect_test_3:Rectangle = Rectangle {
        width:60,
        height:54
    };

    // println!("can rect1 hold rect2? {}",rect_test_1.can_hold(&rect_test_2));
    // println!("can rect1 hold rect3? {}",rect_test_1.can_hold_area(&rect_test_3));

    let rect_squre:Rectangle = Rectangle::square(30);
    print!("rect_square is {}",rect_squre.area());

}

fn area_struct(rect:&Rectangle)->u32{
    rect.width * rect.height
}

//面积
fn area(width:u32,height:u32)->u32{
    width * height
}

fn area_dimension(dimension:(u32,u32))->u32{
    dimension.0*dimension.1
}