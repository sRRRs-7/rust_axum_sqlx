use crate::models::post::Post;

#[allow(dead_code)]
pub fn mock_post(id: usize) -> Post {
    Post {
        id: id as i32,
        user_id: 1,
        category_id: 1,
        title: String::from("I am Rust developer"),
        content: String::from("ownership & borrow"),
    }
}


#[allow(dead_code)]
pub fn mock_posts(num: usize) -> Vec<Post> {
    let mut posts = Vec::new();
    for i in 0..=num {
        posts.push(mock_post(i));
    }
    posts
}