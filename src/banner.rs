use yew::{function_component, Html, html};

#[function_component(Banner)]
pub fn banner() -> Html {
    html! { 
        <div class="banner" id="banner">
        <div class="banner-inner">
            <div class="banner-header">
            <h1 class="banner-title">{"Cody Ross"}</h1>
            <p class="banner-subtitle">{"SOFTWARE DEVELOPER"}</p>
            </div>

            <div class="banner-actions">
                <div class="banner-resume">
                    <a href="/assets/Cody Ross - Resume - 031022.pdf" target="_blank">
                    {"MY RESUME"}</a></div>
                <div class="banner-projects">
                    <a href="#projectsMarker">
                    {"MY PROJECTS \u{27a4}"}</a></div>
            </div>

            <div class="banner-skills-container">
                <h4>{"My Skills"}</h4>
                <div class="banner-skills-top">
                </div>
                <div class="banner-skills-bottom">
                </div>
            </div>
        </div>
        </div>
    }
}