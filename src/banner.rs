use yew::{function_component, Html, html};

#[function_component(Banner)]
pub fn banner() -> Html {
    html! { 
        <div class="banner" id="banner">
        <div class="banner-inner">
            <div class="banner-header">
            <h1 class="banner-title">{"Cody Ross"}</h1>
            <p class="banner-subtitle">{"Software Developer"}</p>
            </div>

            <div class="banner-actions">
                <div class="banner-resume">{"Resume"}</div>
                <div class="banner-projects">{"My Projects \u{27a4}"}</div>
            </div>

            <div class="banner-skills-container">
                {"My Skills"}
                <div class="banner-skills-top">
                </div>
                <div class="banner-skills-bottom">
                </div>
            </div>
        </div>
        </div>
    }
}