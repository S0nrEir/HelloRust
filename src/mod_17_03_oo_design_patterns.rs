use RustBook_Instances_CN::{Post, NewPost};

pub fn enter(){
    _fn();
}

fn _fn(){
    // let mut post = Post::new();
    // post.add_text("i ate a salad for lunch today");
    // assert_eq!("",post.content());

    let mut new_post = NewPost::new();
    new_post.add_text("i ate a salad for lunch today");
    let new_post = new_post.request_review();
    let new_post = new_post.approve();
    assert_eq!("i ate a salad for lunch today",new_post.content());
}