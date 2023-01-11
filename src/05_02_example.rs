// use std::io;

//为了让结构体能够使用debug模式输出信息，使用derive属性来指定结构体使用Debug这种trait
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
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
    println!("the area is {:?}",rect_stct);
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