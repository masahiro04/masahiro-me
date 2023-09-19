// use super::{footer::Footer, header::Header};
use super::{footer, header};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let LayoutProps { children } = props;
    html! {
        <>
            <div class="flex flex-col min-h-screen w-full p-5 mx-auto sm:max-w-4xl sm:py-12">
                <div class="flex-grow">
                    <main>
                        <header::Header />
                        {children.clone()}
                    </main>
                </div>
                <hr class="border-t border-white my-12" />
                <footer::Footer />
            </div>
        </>
    }
}
