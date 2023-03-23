pub fn enter(){
    // {
    //     let c = CustomSmartPointer{ data:String::from("my stuff")};
    //     let d = CustomSmartPointer{ data:String::from("other stuff")};
    //     println!("CustomSmartPointer Created");
    // }

    let a = CustomSmartPointer{ data:String::from("my stuff")};
    //显式调用std::mem::drop可以提前释放一个pointer指向的内存，
    //不用担心因为失误而使用了一个已经被提前释放的变量，因为Rust会在编译的时候帮你检查出问题并且报错
    std::mem::drop(a);
    // a.to_string();
    println!("CustomSmartPointer Created");
}


struct CustomSmartPointer{
    data:String,
}


impl Drop for CustomSmartPointer{

    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}",self.data);
    }
}

impl CustomSmartPointer{
    fn to_string(&self){
        println!("data is {}",self.data);
    }
}