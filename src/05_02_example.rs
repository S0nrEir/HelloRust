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
        Self { 
                width: size,
                height: size 
            }
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