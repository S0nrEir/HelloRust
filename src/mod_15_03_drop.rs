pub fn enter(){
    // {
    //     let c = CustomSmartPointer{ data:String::from("my stuff")};
    //     let d = CustomSmartPointer{ data:String::from("other stuff")};
    //     println!("CustomSmartPointer Created");
    // }

    let a = CustomSmartPointer{ data:String::from("my stuff")};
    let b = CustomSmartPointer{ data:String::from("other stuff")};
    //error：不允许显式调用析构方法，因为rust会在代码段末尾自动调用b.drop()
    //b.drop();

    //如果需要显示地调用drop，可以使用std::mem::drop，这可以提前释放一个pointer指向的内存，
    //不用担心因为失误而使用了一个已经被提前释放的变量，因为Rust会在编译的时候帮你检查出问题并且报错
    //std::mem::drop(a);
    // a.to_string();
    println!("CustomSmartPointer Created");
}


struct CustomSmartPointer{
    data:String,
}


impl Drop for CustomSmartPointer{

    //drop指定了值要离开作用域时要做些什么
    //比如Box类型，当Box<T>被丢弃时会释放box指向的堆内存空间
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'",self.data);
    }
}

impl CustomSmartPointer{
    fn to_string(&self){
        println!("data is {}",self.data);
    }
}