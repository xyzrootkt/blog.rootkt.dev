use leptos::prelude::*;

#[component]
pub fn Header(on_home: Callback<()>) -> impl IntoView {
    view! {
        <header class="header">
            <h1>"rootkt.dev"</h1>
            <p>"Development, Linux, Rust & other stuff."</p>

            <nav>
                <button on:click=move |_| on_home.run(())>
                    "Home"
                </button>
            </nav>
        </header>
    }
}
