use yew::{function_component, Html, html};

#[function_component(Navbar)]
pub fn return_navbar() -> Html {
    html! { 
        <div class="navbar">
            <div class="navbar-home">{"Cody Ross, software developer"}</div>
            <div class="navbar-tail">
                <div class="navbar-resume">{"Resume"}</div>
                <div class="navbar-github">{"Github"}</div>
                <div class="navbar-contact">{"Contact"}</div>
            </div>
        </div>
    }
}