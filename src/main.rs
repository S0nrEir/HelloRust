use std::io;
use rand::Rng;

mod custom_trait;
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
mod mod_11_01_testing;
mod mod_13_01_closure;
mod mod_13_02_iterators;
mod mod_tests;
//也可以使用这种方式一同包含Ordering和io 
// use std::{cmp::Ordering,io};
mod mod_15_01_box;
mod mod_15_02_deref;
mod mod_15_03_drop;

fn main(){
    main_enter();
}

fn main_enter() {
    println!("--------------------------------------------------------------------------");

    // mod_02_guessing_game::enter();
    // mod_03_variables_and_mutability::enter();
    // mod_04_understanding_owership::enter();
    // mod_05_struct_and_example::enmain
    // mod_06_enum_match_if_let::enter();
    // mod_07_pachage_and_crates::enter();
    // mod_08_collections::enter();
    //mod_09_error_handling::enter();
    // mod_10_01_generics::enter();
    // mod_10_02_trait::enter();
    // mod_10_03_lifetime::enter();
    // mod_11_01_testing::enter();
    // mod_13_01_closure::enter();
    // mod_13_02_iterators::enter();
    // mod_15_01_box::enter();
    // mod_15_02_deref::enter();
    mod_15_03_drop::enter();
}