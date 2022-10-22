use yew::{function_component, Html, html, Callback};
use web_sys::{Window, console, window};

#[function_component(Navbar)]
pub fn return_navbar() -> Html {

    let wind = window().unwrap();

    html! { 
        <div class="navbar">
            <div class="navbar-home">
            <div class="navbar-home-name"> 
                {"CODY ROSS"}
            </div>
            <div class="navbar-home-title">
                {"SOFTWARE DEVELOPER"}
            </div>
            </div>
            <div class="navbar-tail">
                <div class="navbar-resume">{"Resume"}</div>
                <div class="navbar-github">{"Github"}</div>
                <div 
                    onclick={move |_| {console::log_1(&format!("{:?}", wind.inner_width()).into())}} 
                    class="navbar-contact">{"Contact"}
                </div>
            </div>
        </div>
    }
}