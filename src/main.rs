use std::io;
use rand::Rng;
//rust中mod的声明规则：
//在 crate 根文件中，可以声明新的模块；比如，用 mod garden; 语句声明了一个叫“garden”的模块。
//编译器将在以下位置查找模块的代码：
//内联（Inline），用大括号替换 mod garden; 语句的分号：mod garden {}
//在 src/garden.rs 文件中；
//在 src/garden/mod.rs 文件中。（这样的问题是：当模块多了以后，mod.rs文件太多了）

//告诉编译器在src/front_of_house.rs中发现并引入模块代码
mod front_of_house;
// pub mod front_of_house;

//hosting模块的路径：crate.front_of_hose.hosting，这样声明后，可以直接使用hosting::xxx();
// use crate::front_of_house::hosting;

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
mod mod_15_04_reference_count;
mod mod_15_05_ref_cell;
mod mod_15_06_reference_cycles;
mod mod_16_01_threads;
mod mod_16_02_message_passing;
mod mod_16_03_shared_state;
mod mod_17_01_what_is_oo;
mod mod_17_02_trait_objects;
mod mod_17_03_oo_design_patterns;
mod mod_18_01_all_the_places_for_patterns;
mod mod_18_02_refutability;

fn main(){
    main_enter();
    // front_of_house::hosting::add_to_waitlist();
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
    // mod_09_error_handling::enter();
    // mod_10_01_generics::enter();
    // mod_10_02_trait::enter();
    // mod_10_03_lifetime::enter();
    // mod_11_01_testing::enter();
    // mod_13_01_closure::enter();
    // mod_13_02_iterators::enter();
    // mod_15_01_box::enter();
    // mod_15_02_deref::enter();
    // mod_15_03_drop::enter();
    // mod_15_04_reference_count::enter();
    // mod_15_05_ref_cell::enter();
    // mod_15_06_reference_cycles::enter();
    // mod_16_01_threads::enter();
    // mod_16_02_message_passing::enter();
    // mod_16_03_shared_state::enter();
    // mod_17_01_what_is_oo::enter();
    // mod_17_02_trait_objects::enter();
    // mod_17_03_oo_design_patterns::enter();
    // mod_18_01_all_the_places_for_patterns::enter();
    mod_18_02_refutability::enter();
}