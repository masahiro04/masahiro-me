use super::{footer::Footer, header::Header};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
    pub children: Children,
}

#[function_component(BaseLayout)]
pub fn base_layout(props: &BaseLayoutProps) -> Html {
    html! {
        <div class="flex flex-col min-h-screen w-full p-5 mx-auto sm:max-w-4xl sm:py-12">
            <div class="flex-grow">
                <main>
                    <Header />
                    { props.children.clone() }
                </main>
            </div>
            <hr class="border-t border-white my-12" />
            <Footer />
        </div>
    }
}
