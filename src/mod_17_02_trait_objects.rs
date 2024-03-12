//使用lib时，命名空间路径以项目开头
use RustBook_Instances_CN::Screen;

pub fn enter(){
    _fn();
}

fn _fn(){
    let screen = Screen{
        components:vec![
            Box::new(
                Screen::create_select_box(75, 10, vec![String::from("yes"),String::from("mayday"),String::from("no")])
            ),
            Box::new(
                Screen::create_button(50, 10, String::from("ok"))
            ),
        ]
    };

    screen.run();
}