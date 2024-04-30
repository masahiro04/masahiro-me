use domain::repositories::post_repository::{PostRepositoryInterface, WithPostRepository};
use std::io::Result;
use use_case::{
    fetch_advisory_projects_usecase::FetchAdvisoryProjectsUsecase,
    fetch_past_work_projects_usecase::FetchPastWorkProjectsUsecase,
    fetch_post_usecase::FetchPostUsecase, fetch_related_posts_usecase::FetchRelatedPostsUsecase,
    fetch_work_projects_usecase::FetchWorkProjectsUsecase,
};
use {
    domain::entities::{post::Post, project::Project},
    infrastructure::repositories::{
        post_repository::PostRepository, project_repository::ProjectRepository,
    },
};

fn client() -> reqwest::Client {
    reqwest::Client::new()
}

// usecaseの中身をダミーに変更したところ、413kになった
// post系のrepoを1つだけ有効化したら785kになったので
// 300k以上一気に増えることになる
pub async fn fetch_posts_usecase(per_page: i32, offset: i32) -> anyhow::Result<Vec<Post>> {
    let client = client();
    let api_url = "https://api.masahiro.me/api".to_string();
    // let api_url = "https://masahiro-me-api-p2h6pos6wq-an.a.run.app/api/v1".to_string();
    struct FetchPostsUsecaseImpl {
        repository: PostRepository,
    }
    impl WithPostRepository for FetchPostsUsecaseImpl {
        type PostRepository = PostRepository;
        fn post_repository(&self) -> &Self::PostRepository {
            &self.repository
        }
    }
    let repository = PostRepository::new(api_url, client);
    let usecase = FetchPostsUsecaseImpl { repository };
    usecase.post_repository().find_posts(per_page, offset).await
}
pub async fn fetch_related_posts_usecase(category_ids: &str) -> Result<Vec<Post>> {
    let client = client();
    // let api_url = "https://masahiro-me-api-p2h6pos6wq-an.a.run.app/api/v1".to_string();
    let api_url = "https://api.masahiro.me/api".to_string();
    let repo = PostRepository::new(api_url, client);
    let usecase = FetchRelatedPostsUsecase::new(repo);
    usecase.execute(category_ids).await
    //     Ok(vec![])
}
pub async fn fetch_post_usecase(slug: String) -> Result<Option<Post>> {
    let client = client();
    // let api_url = "https://masahiro-me-api-p2h6pos6wq-an.a.run.app/api/v1".to_string();
    let api_url = "https://api.masahiro.me/api".to_string();
    let repo = PostRepository::new(api_url, client);
    let usecase = FetchPostUsecase::new(repo);
    usecase.execute(slug).await
    // Ok(None)
}
pub fn fetch_work_projects_usecase() -> Vec<Project> {
    let repo = ProjectRepository::new();
    let usecase = FetchWorkProjectsUsecase::new(repo);
    usecase.execute()
    // vec![]
}
pub fn fetch_past_work_projects_usecase() -> Vec<Project> {
    let repo = ProjectRepository::new();
    let usecase = FetchPastWorkProjectsUsecase::new(repo);
    usecase.execute()
    // vec![]
}
pub fn fetch_advisory_projects_usecase() -> Vec<Project> {
    let repo = ProjectRepository::new();
    let usecase = FetchAdvisoryProjectsUsecase::new(repo);
    usecase.execute()
    // vec![]
}
