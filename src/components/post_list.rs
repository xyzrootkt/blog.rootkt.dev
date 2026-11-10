use leptos::prelude::*;

use crate::posts::Post;

#[component]
pub fn PostList(posts: Vec<Post>, on_select: Callback<usize>) -> impl IntoView {
    view! {
        <section class="posts">
            <h2>"Latest posts"</h2>

            <div class="post-list">
                {posts
                    .into_iter()
                    .enumerate()
                    .map(|(index, post)| {
                        view! {
                            <button
                                class="post-card"
                                on:click=move |_| on_select.run(index)
                            >
                                <span class="post-id">
                                    {post.slug}
                                </span>

                                <h3>{post.title}</h3>

                                <span class="read-more">
                                    "Read article →"
                                </span>
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
