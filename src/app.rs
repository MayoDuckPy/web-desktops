use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/web-desktops.css"/>

        // sets the document title
        <Title text="Web Desktops"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Script type_="module">
        r#"
            import { startCapture, stopCapture } from '/scripts/stream.js';

            window.onload = () => {
                document.getElementById('startButton').onclick = startCapture;
                document.getElementById('stopButton').onclick = stopCapture;
            }
        "#
        </Script>

        <section>
            <div class="filler"></div>
            <article>
                <h1>"Welcome to Leptos!"</h1>
                <video></video>
                <div>
                    <button id="startButton">Start</button>
                    <button id="stopButton">Stop</button>
                </div>
            </article>
        </section>
    }
}
