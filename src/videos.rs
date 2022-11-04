use web_sys::MouseEvent;
use yew::{function_component, Html, html, Callback};
use yew_router::prelude::*;

use crate::Route;


#[function_component(Videos)]
pub fn videos() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick:Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div class="wrapper">
        <div id="videos-navbar" class="navbar videos-navbar">
            <a class="navbar-home" onclick={onclick.clone()}>/* Navigate to home page */
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
                <a class="navbar-contact word" onclick={onclick.clone()}>
                {"PORTFOLIO"}</a>
                /* SVG for < 420px */ /* MUST ALTER THIS FOR VIDEOS */
                <a class="navbar-resume img" href="/assets/Cody Ross - Resume - 031022.pdf" target="_blank">
                    <img src="/assets/images/resume.svg" />
                </a>
                <a class="navbar-github img" href="https://github.com/co-decode" target="_blank">
                    <img src="/assets/images/github.svg" />
                </a>
                <a class="navbar-contact img" onclick={onclick.clone()}>
                    <img src="/assets/images/return.svg" />
                </a>
            </div>
        </div>
        <div class="videos-main">
            <div class="videos-title">
                <h1>{"Project Video Clips"}</h1>
            </div>
            <label id="toLH">{"LIFTING HISTORY"}</label>
            <video class="videos-video" controls={true}>
                <source src="/assets/video/liftingHistoryWebM.webm" type="video/webm"/>
                <source src="/assets/video/liftingHistoryMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toMSF">{"MY STORE FRONT"}</label>
            <video class="videos-video" controls={true}>
                <source src="/assets/video/storeFrontWebM.webm" type="video/webm"/>
                <source src="/assets/video/storeFrontMute.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toSG">{"SHEET MUSIC GENERATOR"}</label>
            <video class="videos-video" controls={true}>
                <source src="/assets/video/sheetGeneratorWEBM.webm" type="video/webm"/>
                <source src="/assets/video/sheetGeneratorMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toA">{"ARHYTHMETIC"}</label>
            <video class="videos-video" controls={true}>
                <source src="/assets/video/ArhythmeticWebM.webm" type="video/webm"/>
                <source src="/assets/video/AryhthmeticMP42.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="toLEC">{"LIFT EQUIVALENCE CALCULATOR"}</label>
            <video class="videos-video" controls={true}>
                <source src="/assets/video/liftEquivalenceWebM.webm" type="video/webm"/>
                <source src="/assets/video/liftEquivalenceMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            
            <div class="videos-buttons-container">
            <a onclick={onclick.clone()} class="videos-buttons">
                {"PORTFOLIO"}</a> 
            <a id="2top" class="videos-buttons" href="#videos-navbar">
                {"BACK TO TOP"}</a>
            </div>
        </div>
        </div>
    }
}