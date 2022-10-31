use yew::{function_component, Html, html};
use web_sys::{ console, window};

#[function_component(Navbar)]
pub fn return_navbar() -> Html {

    let wind = window().unwrap();

    html! { 
        <div class="navbar">
            <a class="navbar-home">
            <div class="navbar-home-name"> 
                {"CODY ROSS"}
            </div>
            <div class="navbar-home-title">
                {"SOFTWARE DEVELOPER"}
            </div>
            </a>
            <div class="navbar-tail">
                <a class="navbar-resume word" href="/assets/Cody Ross - Resume - 031022.pdf" target="_blank">{"RESUME"}</a>
                <a class="navbar-github word" href="https://github.com/co-decode" target="_blank">
                {"GITHUB"}</a>
                <a onclick={move |_| {console::log_1(&format!("{:?}", wind.inner_width()).into())}} 
                   class="navbar-contact word" href="#contact">
                {"CONTACT"}</a>
                /* SVG for < 420px */
                <a class="navbar-resume img">
					<img src="/assets/images/resume.svg" alt="" />
				</a>

                <a class="navbar-github img">
					<img src="/assets/images/github.svg" alt="" />
				</a>
                <a class="navbar-contact img">
					<img src="/assets/images/email1.svg" alt="" />
				</a>
            </div>
        </div>
    }
}