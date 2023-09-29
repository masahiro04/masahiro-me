use domain::repositories::{
    post_repository::{PostRepositoryInterface, WithPostRepository},
    project_repository::{ProjectRepositoryInterface, WithProjectRepository},
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
    usecase.post_repository().find_all(per_page, offset).await
}
pub async fn fetch_posts_by_category_ids_usecase(category_ids: &str) -> anyhow::Result<Vec<Post>> {
    let client = client();
    let api_url = "https://api.masahiro.me/api".to_string();
    struct FetchPostsByCategoryIdsUsecaseImpl {
        repository: PostRepository,
    }
    impl WithPostRepository for FetchPostsByCategoryIdsUsecaseImpl {
        type PostRepository = PostRepository;
        fn post_repository(&self) -> &Self::PostRepository {
            &self.repository
        }
    }
    let repository = PostRepository::new(api_url, client);
    let usecase = FetchPostsByCategoryIdsUsecaseImpl { repository };
    usecase
        .post_repository()
        .find_by_category_ids(category_ids)
        .await
}
pub async fn fetch_post_usecase(slug: String) -> anyhow::Result<Option<Post>> {
    let client = client();
    let api_url = "https://api.masahiro.me/api".to_string();
    struct FetchPostUsecaseImpl {
        repository: PostRepository,
    }
    impl WithPostRepository for FetchPostUsecaseImpl {
        type PostRepository = PostRepository;
        fn post_repository(&self) -> &Self::PostRepository {
            &self.repository
        }
    }
    let repository = PostRepository::new(api_url, client);
    let usecase = FetchPostUsecaseImpl { repository };
    usecase.post_repository().find_one(slug).await
}

pub fn fetch_work_projects_usecase() -> Vec<Project> {
    struct FetchWorkProjectsUsecaseImpl {
        repository: ProjectRepository,
    }
    impl WithProjectRepository for FetchWorkProjectsUsecaseImpl {
        type ProjectRepository = ProjectRepository;
        fn project_repository(&self) -> &Self::ProjectRepository {
            &self.repository
        }
    }
    let repository = ProjectRepository::new();
    let usecase = FetchWorkProjectsUsecaseImpl { repository };
    usecase.project_repository().find_works()
}
pub fn fetch_past_work_projects_usecase() -> Vec<Project> {
    struct FetchPastWorkProjectsUsecaseImpl {
        repository: ProjectRepository,
    }
    impl WithProjectRepository for FetchPastWorkProjectsUsecaseImpl {
        type ProjectRepository = ProjectRepository;
        fn project_repository(&self) -> &Self::ProjectRepository {
            &self.repository
        }
    }
    let repository = ProjectRepository::new();
    let usecase = FetchPastWorkProjectsUsecaseImpl { repository };
    usecase.project_repository().find_past_works()
}
pub fn fetch_advisory_projects_usecase() -> Vec<Project> {
    struct FetchAdvisoryProjectsUsecaseImpl {
        repository: ProjectRepository,
    }
    impl WithProjectRepository for FetchAdvisoryProjectsUsecaseImpl {
        type ProjectRepository = ProjectRepository;
        fn project_repository(&self) -> &Self::ProjectRepository {
            &self.repository
        }
    }
    let repository = ProjectRepository::new();
    let usecase = FetchAdvisoryProjectsUsecaseImpl { repository };
    usecase.project_repository().find_advisories()
}
