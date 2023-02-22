//在Rust中使用泛型没有性能问题，编译时Rust会检查代码中所有用到泛型的地方，然后分析上下文将泛型替换为具体类型。
//也可以使用这样的方法来为struct指定泛型
struct Point<T>{
    x:T,
    y:T,
}

//泛型函数
//注意必须在impl关键字后边跟上<T>
impl<T> Point<T>{
    fn get_x(&self)->&T{
        return &self.x;
    }
}

//为泛型为f32的Point实例定义方法，其他泛型不是f32的Point<T>实例则没有定义此方法
impl Point<f32>{
    fn distance_from_origin(&self) -> f32{
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

//枚举也可以在成员中存放泛型的成员数据，比如Option和Result
// enum Option<T>{
//     Some(T),
//     None,
// }

// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }

pub fn enter(){

    // let integer = Point{
    //      x:5,
    //      y:10 
    // };

    // let float_ = Point{
    //   x:1.0,
    //   y:4.0,
    // };

    // let number_list = vec![34,50,25,100,25];
    // let result = largest(&number_list);
    // println!("largest number is {}",result) ;

    // let char_list = vec!['y','m','a','q'];
    // let result = largest(&char_list);
    // println!("largest char is {}",result);
}

//在Rust中使用泛型
//只要在函数名后跟T即可
//这里的代码是有问题的，无法通过编译
// fn largest<T>(list:&[T]) -> T{

//     let mut largest = &list[0];
//     for item in list{
//         if item > largest{
//             largest = item;
//         }
//     }

//     largest;
// }