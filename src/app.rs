use crate::video_player::VideoPlayer;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web-desktops.css"/>

        // sets the document title
        <Title text="Web Desktops"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section>
            <h1>"Web Desktops"</h1>
            <VideoPlayer/>
            <div id="connect-buttons">
                <button id="clientConnect">Start Client</button>
                <button id="serverConnect">Start Server</button>
                <button id="closeConnection">Reset</button>
            </div>
        </section>
    }
}
