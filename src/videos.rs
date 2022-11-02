use yew::{function_component, Html, html};

#[function_component(Videos)]
pub fn videos() -> Html {
    html! {
        <div class="inner">
            <header class="major">
                <h1>{"Project Video Clips"}</h1>
            </header>
            <label id="toLH">{"Lifting History"}</label>
            <video class="vidPage" controls={true}>
                <source src="/assets/video/liftingHistoryWebM.webm" type="video/webm"/>
                <source src="/assets/video/liftingHistoryMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toMSF">{"My Store Front"}</label>
            <video class="vidPage" controls={true}>
                <source src="/assets/video/storeFrontWebM.webm" type="video/webm"/>
                <source src="/assets/video/storeFrontMute.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toSG">{"My Store Front"}</label>
            <video class="vidPage" controls={true}>
                <source src="/assets/video/sheetGeneratorWEBM.webm" type="video/webm"/>
                <source src="/assets/video/sheetGeneratorMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toLEC">{"Lift Equivalence Calculator"}</label>
            <video class="vidPage" controls={true}>
                <source src="/assets/video/liftEquivalenceWebM.webm" type="video/webm"/>
                <source src="/assets/video/liftEquivalenceMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toA">{"Arhythmetic"}</label>
            <video class="vidPage" controls={true}>
                <source src="/assets/video/ArhythmeticWebM.webm" type="video/webm"/>
                <source src="/assets/video/AryhthmeticMP42.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            
            <a href="/" target="_blank"  class="button bottomVids">{"Portfolio"}</a>
            <a id="2top" class="button bottomVids">{"Back to Top"}</a>
        </div>
    }
}