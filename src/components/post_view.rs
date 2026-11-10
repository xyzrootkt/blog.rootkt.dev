use leptos::prelude::*;

use crate::posts::Post;

#[component]
pub fn PostView(post: Post, on_back: Callback<()>) -> impl IntoView {
    view! {
        <article class="article">
            <button
                class="back"
                on:click=move |_| on_back.run(())
            >
                "← Back to posts"
            </button>

            <div class="article-header">
                <span>{post.slug}</span>
                <h1>{post.title}</h1>
            </div>

            <div
                class="markdown"
                inner_html=post.html
            ></div>
        </article>
    }
}
