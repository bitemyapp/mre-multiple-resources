use leptos::*;
use leptos_meta::*;
use leptos_query::{
    create_query, provide_query_client_with_options_and_persister,
    query_persister, QueryOptions, QueryResult, QueryScope,
};
use leptos_router::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FirstWaitFnQuery(pub u64);

pub fn first_wait_fn_query() -> QueryScope<FirstWaitFnQuery, Result<u64, ServerFnError>> {
    create_query(first_wait_fn, QueryOptions::default())
}

#[server(FirstWaitFn, "/api")]
async fn first_wait_fn(seconds: FirstWaitFnQuery) -> Result<u64, ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds.0)).await;
    Ok(seconds.0)
}

#[server]
async fn second_wait_fn(seconds: u64) -> Result<u64, ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
    Ok(seconds)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_query_client_with_options_and_persister(
        Default::default(),
        query_persister::IndexedDbPersister::default(),
    );

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
                    <Route path="" view=HomePage/>
                    <Route path="/correct" view=HomePageCorrect/>
                    <Route path="/workaround" view=HomePageCorrect/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let signal_one = create_rw_signal(1);
    let QueryResult {
        data: one_second, ..
    } = first_wait_fn_query().use_query(move || FirstWaitFnQuery(signal_one.get()));
    let signal_two = create_rw_signal(2);
    let QueryResult {
        data: two_second, ..
    } = first_wait_fn_query().use_query(move || FirstWaitFnQuery(signal_two.get()));
    view! {
        <Transition fallback= move || view!{"loading..."}>
            {move || {
                match (one_second.get(), two_second.get()) {
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
fn HomePageWorkaround() -> impl IntoView {
    let signal_one = create_rw_signal(1);
    let QueryResult {
        data: one_second, ..
    } = first_wait_fn_query().use_query(move || FirstWaitFnQuery(signal_one.get()));
    let signal_two = create_rw_signal(2);
    let QueryResult {
        data: two_second, ..
    } = first_wait_fn_query().use_query(move || FirstWaitFnQuery(signal_two.get()));
    view! {
        <Transition fallback= move || view!{"loading..."}>
            {move || {
                let one = one_second.get();
                if one.is_none() {
                    return view!{<p>"loading..."</p>}.into_view();
                }
                let one = one.unwrap();
                if one.is_err() {
                    return view!{<p>"error"</p>}.into_view();
                }
                let one = one.unwrap();
                let two = two_second.get();
                if two.is_none() {
                    return view!{<p>"loading..."</p>}.into_view();
                }
                let two = two.unwrap();
                if two.is_err() {
                    return view!{<p>"error"</p>}.into_view();
                }
                let two = two.unwrap();
                view!{ <OtherComponent one=one two=two/> }
            }}
        </Transition>
    }
}

#[component]
fn HomePageCorrect() -> impl IntoView {
    let signal_one = create_rw_signal(1);
    let signal_two = create_rw_signal(2);
    let one_second = create_resource(
        move || signal_one,
        move |seconds| second_wait_fn(seconds.get()),
    );
    let two_second = create_resource(
        move || signal_two,
        move |seconds| second_wait_fn(seconds.get()),
    );
    view! {
        <Transition fallback= move || view!{"loading..."}>
            {move || {
                match (one_second.get(), two_second.get()) {
                    (None, _) | (_, None) => view!{<p>"loading..."</p>}.into_view(),
                    (Some(Err(_)), _) | (_, Some(Err(_))) => view!{<p>"error"</p>}.into_view(),
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
