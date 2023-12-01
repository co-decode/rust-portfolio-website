use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};
use yew_router::prelude::*;

use crate::Route;

#[function_component(LiftLog)]
pub fn liftlog() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Home));
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
                <a class="navbar-resume word" href="/assets/Cody Ross - Resume.pdf" target="_blank">{"RESUME"}</a>
                <a class="navbar-github word" href="https://github.com/co-decode" target="_blank">
                {"GITHUB"}</a>
                <a class="navbar-contact word" onclick={onclick.clone()}>
                {"PORTFOLIO"}</a>
                /* SVG for < 420px */
                <a class="navbar-resume img" href="/assets/Cody Ross - Resume.pdf" target="_blank">
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
        <div class="videos-main liftlog-main">
            <div class="videos-title">
                <h1>{"Welcome to LiftLog"}</h1>
            </div>
            <div class="liftlog-tabs">
                <a href="#v3">
                    <h2>{"v3"}</h2>
                </a>
                <a href="#v2">
                    <h2>{"v2"}</h2>
                </a>
                <a href="#v1">
                    <h2>{"v1"}</h2>
                </a>
            </div>
            <div class="liftlog-label">
            <label id="v3">{"LIFTLOG"}</label>
                <a href="https://liftlog-co-decode.vercel.app/" target="_blank">{"Site"}</a>
                <a href="https://github.com/co-decode/liftlog" target="_blank">{"Source"}</a>
            </div>
            <p class="liftlog-description">{"Write programs, perform workouts and record you weightlifting data with LiftLog"}</p>
            <div class="liftlog-grid">
            <div class="liftlog-cont">
                <div class="liftlog-exp">
                    <video class="liftlog-video" width="305" controls={true}>
                        <source src="../assets/video/liftLog3-login.webm" type="video/webm"/>
                        <source src="../assets/video/liftLog3-login.mp4" type="video/mp4"/>
                    </video>
                    <img class="liftlog-frame" src="../assets/images/phone_frame2.png" width="350"/>

                </div>
                <p>{ "Authentication" }</p>
            </div>
            <div class="liftlog-cont">
                <div class="liftlog-exp">
                    <video class="liftlog-video" width="305" controls={true}>
                        <source src="../assets/video/liftLog3-programs.webm" type="video/webm"/>
                        <source src="../assets/video/liftLog3-programs.mp4" type="video/mp4"/>
                    </video>
                    <img class="liftlog-frame" src="../assets/images/phone_frame2.png" width="350"/>

                </div>
                <p>{ "Program Training" }</p>
            </div>
            <div class="liftlog-cont">
                <div class="liftlog-exp">
                    <video class="liftlog-video" width="305" controls={true}>
                        <source src="../assets/video/liftLog3-workout.webm" type="video/webm"/>
                        <source src="../assets/video/liftLog3-workout.mp4" type="video/mp4"/>
                    </video>
                    <img class="liftlog-frame" src="../assets/images/phone_frame2.png" width="350"/>

                </div>
                <p>{ "Workout" }</p>
            </div>
            <div class="liftlog-cont">
                <div class="liftlog-exp">
                    <video class="liftlog-video" width="305" controls={true}>
                        <source src="../assets/video/liftLog3-sessions.webm" type="video/webm"/>
                        <source src="../assets/video/liftLog3-sessions.mp4" type="video/mp4"/>
                    </video>
                    <img class="liftlog-frame" src="../assets/images/phone_frame2.png" width="350"/>

                </div>
                <p>{ "Log Sessions" }</p>
            </div>
            <div class="liftlog-cont">
                <div class="liftlog-exp">
                    <video class="liftlog-video" width="305" controls={true}>
                        <source src="../assets/video/liftLog3-analysis.webm" type="video/webm"/>
                        <source src="../assets/video/liftLog3-analysis.mp4" type="video/mp4"/>
                    </video>
                    <img class="liftlog-frame" src="../assets/images/phone_frame2.png" width="350"/>

                </div>
                <p>{ "Analyse Data" }</p>
            </div>
            <div class="liftlog-cont">
                <div class="liftlog-exp">
                    <video class="liftlog-video" width="305" controls={true}>
                        <source src="../assets/video/liftLog3-schedule.webm" type="video/webm"/>
                        <source src="../assets/video/liftLog3-schedule.mp4" type="video/mp4"/>
                    </video>
                    <img class="liftlog-frame" src="../assets/images/phone_frame2.png" width="350"/>

                </div>
                <p>{ "Track Schedule" }</p>
            </div>
            </div>
            <div class="liftlog-label ll-mt">
            <label id="v2">{"LIFTING LOG"}</label>
                <a href="https://lifting-log.netlify.app/" target="_blank">{"Site"}</a>
                <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
            </div>
            <video class="videos-video" controls={true}>
                <source src="/assets/video/liftingLogWEBM.webm" type="video/webm"/>
                <source src="/assets/video/liftingLogMP4.mp4" type="video/mp4"/>
                <p>{"You browser doesn't support HTML5 video."}</p>
            </video>
            <label id="v1">{"MY LIFTING RECORD"}</label>
            <video class="videos-video" controls={true}>
                <source src="../assets/video/liftingRecordWebM.webm" type="video/webm"/>
                <source src="../assets/video/liftingRecordMP4.mp4" type="video/mp4"/>
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
