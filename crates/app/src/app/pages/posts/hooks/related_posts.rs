use domain::entities::post::Post;
use std::cell::RefCell;
use std::rc::Rc;
use yew::{
    hook,
    platform::spawn_local,
    suspense::{Suspension, SuspensionResult},
    use_state,
};

use crate::use_cases::fetch_posts_by_category_ids_usecase;

pub struct RelatedPostsState {
    susp: Suspension,
    value: Rc<RefCell<Option<Vec<Post>>>>,
}

impl RelatedPostsState {
    fn new(category_ids: String) -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Vec<Post>>>> = Rc::default();
        {
            let value = value.clone();
            spawn_local(async move {
                {
                    let posts = match fetch_posts_by_category_ids_usecase(&category_ids).await {
                        Ok(posts) => posts,
                        Err(_) => vec![],
                    };
                    {
                        let mut value = value.borrow_mut();
                        *value = Some(posts);
                    }
                }
                handle.resume();
            });
        }
        Self { susp, value }
    }
}

#[hook]
pub fn use_related_posts(category_ids: String) -> SuspensionResult<Vec<Post>> {
    let posts_state = use_state(|| RelatedPostsState::new(category_ids));
    let result = match *posts_state.value.borrow() {
        Some(ref user) => Ok(user.clone()),
        None => Err(posts_state.susp.clone()),
    };
    result
}
