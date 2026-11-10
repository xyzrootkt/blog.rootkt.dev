use leptos::prelude::*;

use crate::components::{Footer, Header, PostList, PostView};
use crate::posts::load_posts;

#[component]
pub fn App() -> impl IntoView {
    let posts = StoredValue::new(load_posts());
    let (active, set_active) = signal(None::<usize>);

    let go_home = Callback::new(move |_| {
        set_active.set(None);
    });

    let select_post = Callback::new(move |index: usize| {
        set_active.set(Some(index));
    });

    view! {
        <div class="container">
            <Header on_home=go_home />

            <main>
                {move || {
                    match active.get() {
                        None => {
                            view! {
                                <PostList
                                    posts=posts.get_value()
                                    on_select=select_post
                                />
                            }
                                .into_any()
                        }

                        Some(index) => {
                            let post = posts.with_value(|items| items.get(index).cloned());

                            match post {
                                Some(post) => {
                                    view! {
                                        <PostView
                                            post=post
                                            on_back=go_home
                                        />
                                    }
                                        .into_any()
                                }

                                None => {
                                    view! {
                                        <p>"Post not found."</p>
                                    }
                                        .into_any()
                                }
                            }
                        }
                    }
                }}
            </main>

            <Footer />
        </div>
    }
}
