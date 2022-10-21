use yew::prelude::*;

mod navbar;
mod banner;
mod projects;
mod contact;

#[function_component]
fn App() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    html! {
        <div class="main">
            <navbar::Navbar />
            <banner::Banner />
            <projects::Projects />
/*             <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p> */
            <contact::Contact />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}