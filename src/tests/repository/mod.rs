use crate::repository::{
    user::MockUserRepoTrait,
    category::MockCategoryRepoTrait,
    post::MockPostRepoTrait,
    Repositories
};

#[derive(Debug)]
pub struct MockRepoImpl {
    pub user: MockUserRepoTrait,
    pub category: MockCategoryRepoTrait,
    pub post: MockPostRepoTrait,
}

impl MockRepoImpl {
    pub fn new(
        mock_user: MockUserRepoTrait,
        mock_category: MockCategoryRepoTrait,
        mock_post: MockPostRepoTrait,
    ) -> Self {
        Self {
            user: mock_user,
            category: mock_category,
            post: mock_post,
        }
    }
}

impl Repositories for MockRepoImpl {
    type UserRepo = MockUserRepoTrait;
    type CategoryRepo = MockCategoryRepoTrait;
    type PostRepo = MockPostRepoTrait;

    fn user(&self) -> &Self::UserRepo {
        &self.user
    }
    fn category(&self) -> &Self::CategoryRepo {
        &self.category
    }
    fn post(&self) -> &Self::PostRepo {
        &self.post
    }
}


pub async fn create_repository_test() -> MockRepoImpl {
    MockRepoImpl::new(
        MockUserRepoTrait::new(),
        MockCategoryRepoTrait::new(),
        MockPostRepoTrait::new(),
    )
}