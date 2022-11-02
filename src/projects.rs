use yew::{function_component, Html, html};

#[function_component(Projects)]
pub fn projects() -> Html { 
    html! {
        <div class="projects-container" id="projects">
            <a id="projectsMarker"></a>
            <div class="projects-1">
                <div class="project-inner">
                <h3 class="hoverBlue"><a>{"Weightlifting Log"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    <a href="/videos#toLH" target="_blank">{"Video"}</a>
                </div>
                </div>
            </div>
            <div class="projects-2">
                <div class="project-inner">
                </div>
            </div>
            <div class="projects-3">
                <div class="project-inner">
                </div>
            </div>
            <div class="projects-4">
                <div class="project-inner">
                </div>
            </div>
            <div class="projects-5">
                <div class="project-inner">
                </div>
            </div>
            <div class="projects-6">
                <div class="project-inner">
                </div>
            </div>
        </div>
    }
}