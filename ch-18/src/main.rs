use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();

    // let post = post.reject();
    // below should not work because approval doesn't work on DraftPosts
    // let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
