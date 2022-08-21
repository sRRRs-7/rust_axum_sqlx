use crate::models::user::User;

#[allow(dead_code)]
pub fn user_mock(id: usize) -> User {
    User {
        id: id as i32,
        name: String::from("daniel"),
        msg: Some(String::from("I love coffee")),
        age: Some(26),
        image: Some(String::from("image")),
    }
}

#[allow(dead_code)]
pub fn users_mock(num: usize) -> Vec<User> {
    let mut users = Vec::new();
    for i in 1..=num {
        users.push(user_mock(i));
    }
    users
}