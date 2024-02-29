use std::rc::Rc;

enum List{
    Cons(i32,Rc<List>),
    Nil,
}

pub fn enter(){

    // 编译错误，因为a的所有权已经移动给b
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // func();
    ref_count_print();


}

fn func(){
    
    //使用Rc<T>（引用计数）代替Box<T>
    //创建b时，不会获取a的所有权，而是克隆a所包含的Rc<List>，这回将引用计数从1增加至2
    //并且允许a和b共享Rc<List>中数据的所有权。
    //创建c时也会克隆a，这会将引用计数从2增加至3
    //每次调用clone方法，数据的引用计数都会增加，直到没有引用前都不会被清理
    let a = Rc::new(List::Cons((5), (Rc::new(List::Cons((10), (Rc::new(List::Nil)))))));
    //b和c共同持有a的引用
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
}


fn ref_count_print(){
    //对于一个引用的计数，可以通过Rc::Strong_count函数获取
    let a = Rc::new(List::Cons((5), (Rc::new(List::Cons((10), (Rc::new(List::Nil)))))));
    println!("count after creating a = {}",Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}",Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}",Rc::strong_count(&a));
        //当离开作用域时，Rc<T>的Drop trait会保证其引用自动减少
    }
    println!("count after c gose out of scope = {}",Rc::strong_count(&a));
}