use yew::{function_component, Html, html, Properties};
use web_sys::{ console, window};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub scr_y: f64,
}

#[function_component(Navbar)]
pub fn return_navbar(props: &Props) -> Html {

    let wind = window().unwrap();
    console::log_1(&format!("{:?}", props.scr_y.clone()).into());

    html! { 
        <div class="navbar anchor">
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
                <a onclick={move |_| {console::log_1(&format!("{:?} & {:?}", wind.inner_width(), "5").into())}} 
                   class="navbar-contact word" href="#contact">
                {"CONTACT"}</a>
                /* SVG for < 420px */
                <a class="navbar-resume img" href="/assets/Cody Ross - Resume - 031022.pdf" target="_blank">
					<img src="/assets/images/resume.svg" />
				</a>
                <a class="navbar-github img" href="https://github.com/co-decode" target="_blank">
					<img src="/assets/images/github.svg" />
				</a>
                <a class="navbar-contact img" href="#contact">
					<img src="/assets/images/email1.svg" />
				</a>
            </div>
        </div>
    }
}