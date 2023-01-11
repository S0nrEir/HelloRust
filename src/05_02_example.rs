
fn ch_05_02_enter(){
    rectangles();
}

fn rectangles(){
    let width:u32 = 50;
    let height:u32 = 50;
    println!("the area is {}",area(width,height));

}

//面积
fn area(width:u32,height:u32)->u32{
    width * height
}