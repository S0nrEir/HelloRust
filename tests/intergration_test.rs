//要使用集成测试，在src的同级目录建立tests目录
//该目录下的每一个文件，Rust都会将其当作一个crate编译

//并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)]
//tests文件夹在 Cargo 中是一个特殊的文件夹，Cargo 只会在运行 cargo test 时编译这个目录中的文件

//在lib中对外声明mod，然后在tests中从根crate目录引用即可
//要注意的是对于src目录，引用根路径从crate开始，而tests和src平级，他的根从tests开始
//cargo test --test integration_test运行指定模块的集成测试
use RustPrograming_Cases::mod_intergration_test;

#[test]

fn do_test(){
    assert_eq!(2, mod_intergration_test::add(1,1));
} 