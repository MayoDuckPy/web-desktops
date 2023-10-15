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
        <Script type_="module">
        r#"
            import { connectClient, createServer, closeConnection } from '/scripts/stream.js';

            window.onload = () => {
                document.getElementById('clientConnect').onclick = connectClient;
                document.getElementById('serverConnect').onclick = createServer;
                document.getElementById('closeConnection').onclick = closeConnection;
            };
        "#
        </Script>

        <section>
            <div class="filler"></div>
            <article>
                <h1>"Welcome to Leptos!"</h1>
                <video autoplay></video>
                <div>
                    <button id="clientConnect">Start Client</button>
                    <button id="serverConnect">Start Server</button>
                    <button id="closeConnection">Reset</button>
                </div>
            </article>
        </section>
    }
}
