use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("This should be illegal");
    post.approve();
    assert_eq!("", post.content());

    // post.approve();
    // assert_eq!("I ate a salad for lunch today", post.content());

    post.reject();
    post.reset_text();
    // assert_eq!("", post.content());
    post.add_text("I ate nine burgers for dinner today");

    post.request_review();
    post.approve();
    post.approve();
    assert_eq!("I ate nine burgers for dinner today", post.content());
    // assert_eq!("", post.content());
}
