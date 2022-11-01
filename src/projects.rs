use yew::{function_component, Html, html};

#[function_component(Projects)]
pub fn projects() -> Html { 
    html! {
        <div class="projects-container" id="projects">
            <a id="projectsMarker"></a>
            <div class="projects-1">{"Project 1"}</div>
            <div class="projects-2">{"Project 2"}</div>
            <div class="projects-3">{"Project 3"}</div>
            <div class="projects-4">{"Project 4"}</div>
            <div class="projects-5">{"Project 5"}</div>
            <div class="projects-6">{"Project 6"}</div>
        </div>
    }
}