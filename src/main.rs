mod mod_02_guessing_game;//第二章猜数游戏
mod mod_03_variables_and_mutability;//第三章vasriables_and_mutability



//也可以使用这种方式一同包含Ordering和io
// use std::{cmp::Ordering,io};

fn main(){
    main_enter();
}

fn main_enter() {
    mod_02_guessing_game::enter();
    mod_03_variables_and_mutability::enter();
}