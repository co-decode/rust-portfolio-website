use yew::{function_component, Html, html, Properties};
use yew::functional::{use_effect_with_deps};
use web_sys::{ /* console ,*/ window, /* Element, */ /* DomTokenList */};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub scr_y: f64,
}

#[function_component(Navbar)]
pub fn return_navbar(props: &Props) -> Html {
    let document = window().unwrap().document().unwrap();
    let scr_y = props.scr_y.clone();
    use_effect_with_deps(
        move |_| {
            if scr_y + 10.0 + f64::from(document.get_element_by_id("navbar").unwrap().client_height()) > window().unwrap().inner_height().unwrap().as_f64().unwrap() {
                document.get_element_by_id("navbar").unwrap().class_list().remove_1("anchor")
                    .expect("anchor class should be removed from navbar element");
                document.get_element_by_id("navbar").unwrap().class_list().add_1("fixed")
                    .expect("fixed class should be added from navbar element");
            } else {
                document.get_element_by_id("navbar").unwrap().class_list().add_1("anchor")
                    .expect("anchor class should be added to navbar element");
                document.get_element_by_id("navbar").unwrap().class_list().remove_1("fixed")
                    .expect("fixed class should be removed from navbar element");
            }
        },
        scr_y,
    );

    // let wind = window().unwrap();
    // console::log_1(&format!("{:?}", props.scr_y.clone()).into());

    html! { 
        <div id="navbar" class="navbar anchor">
            <a class="navbar-home" href="#banner">
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
                <a class="navbar-contact word" href="#contact">
                {"CONTACT"}</a>
                /* SVG for < 420px */
                <a class="navbar-resume img" href="/assets/Cody Ross - Resume - 031022.pdf" target="_blank">
					<img src="/assets/images/resume.svg" />
				</a>
                <a class="navbar-github img" href="https://github.com/co-decode" target="_blank">
					<img src="/assets/images/github.svg" />
				</a>
                <a class="navbar-contact img" href="#contact">
					<img src="/assets/images/email.svg" />
				</a>
            </div>
        </div>
    }
}