use crate::models::category::Category;

#[allow(dead_code)]
pub fn mock_category(id: usize) -> Category {
    Category {
        id: id as i32,
        category: String::from("new category"),
    }
}

#[allow(dead_code)]
pub fn mock_categories(num: usize) -> Vec<Category> {
    let mut categories = Vec::new();
    for i in 1..=num {
        categories.push(mock_category(i));
    }
    categories
}