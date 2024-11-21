use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[server(WaitFn)]
async fn wait_fn(milliseconds: u64) -> Result<u64, ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_millis(milliseconds)).await;
    Ok(milliseconds)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mre-multiple-resources.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePageRepro/>
                    <Route path="/correct" view=HomePageCorrect/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePageRepro() -> impl IntoView {
    let fast = create_resource(|| (), |_| wait_fn(50));
    let slow = create_resource(|| (), |_| wait_fn(1000));

    view! {
        <Transition fallback= move || view!{"loading..."}>
            {move || {
                let loading_view = view!{<p>"loading..."</p>}.into_view();
                let error_view = view!{<p>"Error!"</p>}.into_view();
                let fast = fast.get();
                if fast.is_none() {
                    return loading_view;
                }
                let fast = fast.unwrap();
                if fast.is_err() {
                    return error_view;
                }
                let fast = fast.unwrap();
                let slow = slow.get();
                if slow.is_none() {
                    return loading_view;
                }
                let slow = slow.unwrap();
                if slow.is_err() {
                    return error_view;
                }
                let slow = slow.unwrap();
                tracing::info!("rendering one and two");
                view!{ <OtherComponent one=fast two=slow/> }
            }}
        </Transition>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePageCorrect() -> impl IntoView {
    let fast = create_resource(|| (), |_| wait_fn(50));
    let slow = create_resource(|| (), |_| wait_fn(1000));

    view! {
        <Transition fallback= move || view!{"loading..."}>
            {move || {
                match (fast.get(), slow.get()) {
                    (None, _) | (_, None) => view!{<p>"loading..."</p>}.into_view(),
                    (Some(Err(_)), _) | (_, Some(Err(_)))  => view!{<p>"error"</p>}.into_view(),
                    (Some(Ok(one)), Some(Ok(two))) => {
                        tracing::info!("rendering one and two");
                        view!{ <OtherComponent one=one two=two/> }
                    }
                }
            }}
        </Transition>
    }
}

#[component]
pub fn OtherComponent(one: u64, two: u64) -> impl IntoView {
    view! {
        <p>{one}{two}</p>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
