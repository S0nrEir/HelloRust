use std::{file, fs::File, io::{ErrorKind, Write, Read,self}};
use std::io::Error;

pub fn enter(){
    // ch_09_01_enter();
    ch_09_02_enter();
}

fn ch_09_01_enter(){
    return;
    //在rust中可以使用panic!来处理不可恢复错误（可能导致程序崩溃的）
    //当发生panic时，Rust会回溯stack并且逐个清理每一个函数的数据，这个过程会有很多工作
    //另外一种方式是直接终止（abort），这会不清理就退出程序。
    //此时程序使用的内存就需要操作系统来清理
    //可通过在cargo.toml的[profile]部分指定panic='abort'，指定程序发生panic时由展开(unwinding)切换为终止(abort)

    //主动发起panic
    // panic!("程序发生crash，直接退出");

    //可以通过设置调用RUST_BACKTRACE来设置环境变量得到发生该panic的完整调用栈
    //命令行设置方法set RUST_BACKTRACE=1
    //PowerSheel:$end:$RUST_BACKTRACE=1或$end:$RUST_BACKTRACE="full"
    let v = vec![1,2,3];
    v[99];
}

//使用Result<T,E>来处理可恢复错误\
fn ch_09_02_enter(){
    // read_username_from_file_2();
    // return;
    // file_unwrap_expect();
    //尝试打开一个不存在的文件，使用Result<T,E>来处理异常
    //File::open返回一个Result<T,E>类型，表示操作结果
    let greeting_file_result = File::open("hello.txt");
    let mut greeting_file = match greeting_file_result 
    {
        // _=>(), 
        //如果成功，则将结果返回给文件句柄greeting_fiel
        Ok(file)=>file,
        //失败则发生panic
        //或者检查具体的错误类型，再做不同的处理
        //kind()函数获取具体的错误类型
        Err(err)=>match err.kind() 
        {
            //如果文件未找到，则新建文件
            //因为新建文件也可能失败，所以还要做一次match
            ErrorKind::NotFound => 
            {
                let result = File::create("hello.txt");
                match result 
                {
                    // Ok(new_file) => write(new_file.by_ref(),b"hello rust"),
                    Ok(new_file)=>new_file,
                    Err(err) => panic!("problem creating the file:{:?}",err),
                }
            },
            //其他类型的错误则panic
            other_err => panic!("problem opening the file:{:?}",other_err),
        }
    };//end match
    // re_write(&mut greeting_file, String::from("testing").as_bytes());
}

//使用?来对progapating进行简写，这和read_username_from_file函数起到的作用是一样的
//Result值之后的?表示：如果结果是ok，这个表达式将返回ok中的值，程序继续执行
//如果是Err，Err将作为函数的返回值，就像使用了return关键字
//match和?的区别在于。?的错误值被传递给了from函数，它用于将错误的类型转换为当前函数返回值指定的类型，这是一个标准库的trait
//注意，?运算符只能被用于返回值与?作用的值像兼容的函数。
//换句话说，函数返回值为Result<T,E>的函数才能使用?表达式
//同样?表达式也可用于Option<T>
fn read_username_from_file_2()->Result<String, io::Error>{
    //也可以像这样直接使用链式调用
    // let mut username = String::new();
    // let mut username_file = File::open("hello.txt")?.read_to_string(&mut username)?;
    // return Ok(username);
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    return Ok(username);
}

//错误传播
//当调用一个可能会失败的函数时，除了在这个函数处理错误外，还可以选择外部调用方知道这个错误并如何处理，
//将错误的处理方式交给外部调用
//从文件读用户名，返回一个Result<String,io::Error>类型
//如果这个函数没有任何错误成功返回，调用方会收到一个包含String的Ok值
fn read_username_from_file()->Result<String, io::Error>{
    let username_file_result = File::open("hello.txt");
    //打开以后检查结果，如果失败返回Err成员
    let mut username_file = match username_file_result {
        Ok(file)=>file,
        Err(err)=>return Err(err),
    };

    let mut username = String::new();
    //把文件内容读到string里，成功写入设置ok成员，失败设置Err成员
    match username_file.read_to_string(&mut username){
        Ok(_)=>return Ok(username),
        Err(err)=>return Err(err),
    }
}

fn file_unwrap(){
    // let mut greeting_file_result = File::open("hello.txt");
    // //unwrap()函数，如果Result值成员是OK，则返回OK中的值，如果Result成员是Err，则unwrap函数会自动调用panic，比如下面这样
    // greeting_file_result = File::open("hello.txt").unwrap();
}

fn file_unwrap_expect(){
    // let mut greeting_file_result = File::open("hello.txt");
    // //自定义错误信息
    // greeting_file_result = File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn write(file:&mut File,content:&[u8]){
    file.write_all(content);
}
