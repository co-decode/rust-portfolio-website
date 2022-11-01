use yew::prelude::*;
use gloo_events::EventListener;
use web_sys::{window, /* console */};

mod navbar;
mod banner;
mod projects;
mod contact;

#[function_component]
fn App() -> Html {
    let scr_y = use_state(||window().unwrap().scroll_y());
    

    use_effect(
        {
            let scr_y = scr_y.clone();

            move || {
                let mut scroll_listener = None;

                if let Some(element) = window() {
                    let onscroll = Callback::from(move |_: Event| {
                        // console::log_1(&format!("{:?} & {:?}", window().unwrap().inner_width(), *scr_y).into());
                        // let scroll_position = 
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
        }
    //     window()
    );
    html! {
        <div class="main">
            <navbar::Navbar scr_y={*scr_y.as_ref().unwrap()}/>
            <banner::Banner />
            <projects::Projects />
            <contact::Contact />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}