use yew::prelude::*;
use yew_router::prelude::*;
use gloo_events::EventListener;
use web_sys::{window, /* console */};

mod navbar;
mod banner;
mod projects;
mod contact;

mod videos;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Videos => html! {<videos::Videos />},
        Route::NotFound => html! {<Home />},
    }
}

#[function_component(Home)]
fn home() -> Html {

    let scr_y = use_state(||window().unwrap().scroll_y());

    use_effect({
        let scr_y = scr_y.clone();
        move || {
            let mut scroll_listener = None;

            if let Some(element) = window() {
                let onscroll = Callback::from(move |_: Event| {
                    scr_y.set(window().unwrap().scroll_y())
                });

                let listener = EventListener::new(
                    &element,
                    "scroll",
                    move |e| onscroll.emit(e.clone())
                );
                scroll_listener = Some(listener);
            }
            move || drop(scroll_listener)
        }
    });

    html! {
        <div class="main">
            <navbar::Navbar scr_y={*scr_y.as_ref().unwrap()}/>
            <banner::Banner />
            <projects::Projects />
            <contact::Contact />
        </div>
    }
}

#[function_component]
fn App() -> Html {
    
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}