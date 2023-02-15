use std::{file, fs::File, io::{ErrorKind, Write}, result};

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
    //尝试打开一个不存在的文件，使用Result<T,E>来处理异常
    //File::open返回一个Result<T,E>类型，表示操作结果
    let greeting_file_result = File::open("hello.txt");
    let mut greeting_file = match greeting_file_result {
        // _=>(), 
        //如果成功，则将结果返回给文件句柄greeting_fiel
        Ok(file)=>file,
        //失败则发生panic
        //或者检查具体的错误类型，再做不同的处理
        //kind()函数获取具体的错误类型
        Err(err)=>match err.kind() {
            //如果文件未找到，则新建文件
            //因为新建文件也可能失败，所以还要做一次match
            ErrorKind::NotFound => match File::create("hello.txt") {
                // Ok(new_file) => write(new_file.by_ref(),b"hello rust"),
                Ok(new_file)=>new_file,
                Err(err) => panic!("problem creating the file:{:?}",err),
            },
            //其他类型的错误则panic
            other_err => panic!("problem opening the file:{:?}",other_err),
        }
    };//end match
    // re_write(&mut greeting_file, String::from("testing").as_bytes());
}

fn write(file:&mut File,content:&[u8]){
    file.write_all(content);
}
