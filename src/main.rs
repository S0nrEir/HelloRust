use std::io;
use rand::Rng;

mod mod_02_guessing_game;//ch2
mod mod_03_variables_and_mutability;//vasriables_and_mutability
mod mod_04_understanding_owership;
mod mod_05_struct_and_example;
mod mod_06_enum_match_if_let;
mod mod_07_pachage_and_crates;
mod mod_08_collections;
mod mod_09_error_handling;
mod mod_10_01_generics;
mod mod_10_02_trait;
mod mod_10_03_lifetime;

//也可以使用这种方式一同包含Ordering和io 
// use std::{cmp::Ordering,io};

fn main(){
    main_enter();
}

fn main_enter() {
    // mod_02_guessing_game::enter();
    // mod_03_variables_and_mutability::enter();
    // mod_04_understanding_owership::enter();
    // mod_05_struct_and_example::enter();
    // mod_06_enum_match_if_let::enter();
    // mod_07_pachage_and_crates::enter();
    // mod_08_collections::enter();
    //mod_09_error_handling::enter();
    // mod_10_01_generics::enter();
    // mod_10_02_trait::enter();
    mod_10_03_lifetime::enter();
}