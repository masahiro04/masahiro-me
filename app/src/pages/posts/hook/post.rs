use crate::{domain::entities::post::Post, usecase::exe::fetch_post_usecase};
use std::cell::RefCell;
use std::rc::Rc;
use yew::{
    hook,
    platform::spawn_local,
    suspense::{Suspension, SuspensionResult},
    use_state,
};

const PER_PAGE: i32 = 10;

pub struct PostState {
    susp: Suspension,
    value: Rc<RefCell<Option<Post>>>,
}

impl PostState {
    fn new(slug: String) -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Post>>> = Rc::default();

        {
            let value = value.clone();
            // we use tokio spawn local here.
            spawn_local(async move {
                // let user = fetch_user().await;
                {
                    let mut value = value.borrow_mut();

                    let post = match fetch_post_usecase(slug).await {
                        Ok(post) => post,
                        Err(e) => None,
                    };
                    *value = post;
                }

                handle.resume();
            });
        }

        Self { susp, value }
    }
}

#[hook]
pub fn use_post(slug: String) -> SuspensionResult<Post> {
    let post_state = use_state(|| PostState::new(slug));

    let result = match *post_state.value.borrow() {
        Some(ref user) => Ok(user.clone()),
        None => Err(post_state.susp.clone()),
    };

    result
}
