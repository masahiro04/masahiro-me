use crate::{domain::entities::post::Post, usecase::exe::fetch_posts_usecase};
use std::cell::RefCell;
use std::rc::Rc;
use yew::{
    hook,
    platform::spawn_local,
    suspense::{Suspension, SuspensionResult},
    use_state,
};

const PER_PAGE: i32 = 10;

pub struct OffsetState {
    value: i32,
}
pub struct PostsState {
    offset: OffsetState,
    susp: Suspension,
    value: Rc<RefCell<Option<Vec<Post>>>>,
}

impl PostsState {
    fn new(offset: i32) -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Vec<Post>>>> = Rc::default();

        {
            let value = value.clone();
            spawn_local(async move {
                {
                    let mut value = value.borrow_mut();
                    let posts = match fetch_posts_usecase(PER_PAGE, offset).await {
                        Ok(posts) => posts,
                        Err(_) => vec![],
                    };
                    *value = Some(posts);
                }
                handle.resume();
            });
        }

        Self {
            susp,
            value,
            offset: OffsetState { value: offset },
        }
    }
}

#[hook]
pub fn use_posts(offset: i32) -> SuspensionResult<Vec<Post>> {
    let posts_state = use_state(|| PostsState::new(offset));
    if posts_state.offset.value != offset {
        let new_state = PostsState::new(offset);
        posts_state.set(new_state);
    }

    let result = match *posts_state.value.borrow() {
        Some(ref user) => Ok(user.clone()),
        None => Err(posts_state.susp.clone()),
    };

    result
}
