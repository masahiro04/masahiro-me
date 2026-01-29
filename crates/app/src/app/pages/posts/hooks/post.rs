use crate::use_cases::fetch_post_usecase;
use domain::entities::post::Post;
use std::cell::RefCell;
use std::rc::Rc;
use yew::{
    hook,
    platform::spawn_local,
    suspense::{Suspension, SuspensionResult},
    use_state,
};

pub struct PostState {
    #[allow(dead_code)]
    susp: Suspension,
    value: Rc<RefCell<Option<Post>>>,
}

impl PostState {
    fn new(slug: String) -> Self {
        let (susp, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Post>>> = Rc::default();
        {
            let value = value.clone();
            spawn_local(async move {
                {
                    let post = match fetch_post_usecase(slug).await {
                        Ok(post) => {
                            log::debug!("post OK!!!!!l,{:?}", post);
                            post
                        }
                        Err(e) => {
                            log::debug!("post ERR!!!!!l, {}", e);
                            None
                        }
                    };
                    {
                        let mut value = value.borrow_mut();
                        *value = post;
                    }
                }
                handle.resume();
            });
        }
        Self { susp, value }
    }
}

#[hook]
pub fn use_post(slug: String) -> SuspensionResult<Option<Post>> {
    let post_state = use_state(|| PostState::new(slug.clone()));
    //let post_state = use_state(|| PostState::new(slug));
    // Ok(*post_state.value.borrow())
    let result = match *post_state.value.borrow() {
        Some(ref post) => {
            log::debug!("hooks !!haittayo!!!!");
            Ok(Some(post.clone()))
        }
        None => {
            log::debug!("None!!!!!!!!!!!!!!!!");
            Ok(None)
        }
    };
    result
}
