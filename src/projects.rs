use yew::{function_component, Html, html,Event, Callback, MouseEvent, use_effect};
use yew_router::prelude::*;
use gloo_events::EventListener;
use web_sys::{window, HtmlMediaElement, Element, EventTarget};
use js_sys::Object;
use wasm_bindgen::JsCast;

use crate::Route;

#[function_component(Projects)]
pub fn projects() -> Html { 
    let navigator = use_navigator().unwrap();

    use_effect({
        move || {
            let mut fullscreenchange_listener = None;
            if let Some(element) = window().unwrap().document() {
                let onfullscreenchange = Callback::from(move |_: Event| {
                    if window().unwrap().document().unwrap().fullscreen_element().is_none() {
                        let item = Object::from(window().unwrap().document().unwrap().get_elements_by_class_name("video"));
                        let array = Object::values(&item);
                        array.iter().for_each(move |el| {
                            let el = HtmlMediaElement::from(el);
                            el.clone().remove_attribute("controls").expect("Element successfully drops controls attribute");
                            el.clone().pause().unwrap()
                        });
                    }
                });
                let listener = EventListener::new(
                    &element,
                    "fullscreenchange",
                    move |e| onfullscreenchange.emit(e.clone())
                );
                fullscreenchange_listener = Some(listener);
            }
            move || drop(fullscreenchange_listener)
        }
    });

    let onclick:Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Videos));

    let onmouseenter = {
        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<Element>().ok());
            let video = input.unwrap().get_elements_by_tag_name("video")
                .get_with_index(0).expect("There should be a video element")
                .dyn_into::<HtmlMediaElement>().ok();
            if let Some(video) = video {
                video.set_muted(true);
                let _result = wasm_bindgen_futures::JsFuture::from(video.play().unwrap());
            }
        })
    };
    let onmouseleave = {
        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<Element>().ok());
            let video = input.unwrap().get_elements_by_tag_name("video")
                .get_with_index(0).expect("There should be a video element")
                .dyn_into::<HtmlMediaElement>().ok();
            if let Some(video) = video {
                video.pause().unwrap()
            }
        })
    };
    let onclick_fs ={
        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<Element>().ok());
            let video = input.unwrap().parent_element()
                .unwrap().parent_element()
                .unwrap().parent_element()
                .unwrap().get_elements_by_tag_name("video")
                .get_with_index(0).expect("There should be a video element")
                .dyn_into::<HtmlMediaElement>().ok();
            if let Some(video) = video {
                video.set_controls(true); 
                video.request_fullscreen().expect("Video should go fullscreen")
            }
        })
    };

    html! {
        <div class="projects-container" id="projects">
            <a id="projectsMarker"></a> 
            <header>
                <p>{"\u{1F980} This website was built with:"}</p>
                <div class="techTags">
                    <span class="project-svg">
                        <img src="/assets/images/rust.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/yew.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/sass.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/webassembly.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/nginx.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/vultr.svg" alt="" />
                    </span>
                </div>
                <a href="https://github.com/co-decode/rust-portfolio-website" target="_blank">{"Source"}</a>
            </header>
            /* Project 1 */
            <div class="weightlifting_log" onmouseenter={onmouseenter.clone()} onmouseleave={onmouseleave.clone()}>
                <div class="project-inner">
                <h3><a onclick={onclick_fs.clone()}>{"Weightlifting Log"}</a></h3>
                <div class="project-nav">
                    <a href="https://lifting-log.netlify.app/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toLL">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                <video id="liftingRecord" poster="/assets/images/barbellgym.svg" loop={true} muted={true}>
                    <source src="/assets/video/liftingLogWEBM.webm" type="video/webm"/>
                    <source src="/assets/video/liftingLogMP4.mp4" type="video/mp4"/>
                    <p>{"Your browser doesn't support HTML5 video."}</p>
                </video>
                </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/javascript.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/react.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/reactrouter.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/sass.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/d3.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/mswjs.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/jest.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/axios.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/express.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/postgresql.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/bcrypt.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/passportjs.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/docker.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/railway.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/netlify.svg" alt="" />
                    </span>
                </div>
            </div>

            /* Project 2 */
            
            <div class="fitness_store" onmouseenter={onmouseenter.clone()} onmouseleave={onmouseleave.clone()}>
                <div class="project-inner">
                <h3><a onclick={onclick_fs.clone()}>{"Fitness Store"}</a></h3>
                <div class="project-nav">
                    <a href="https://node-store-front.netlify.app/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/storeFront" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toMSF">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                    <video id="storeFront" class="storeFront" loop={true}>
                        <source src="/assets/video/storeFrontWebM.webm" type="video/webm"/>
                        <source src="/assets/video/storeFrontMute.mp4" type="video/mp4"/>
                        <p>{"Your browser doesn't support HTML5 video."}</p>
                    </video>
                </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/typescript.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/react.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/reactrouter.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/redux.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/vite.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/css3.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/bootstrap.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/nodejs.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/express.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/mongodb.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/graphql.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/urql.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/apollographql.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/heroku.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/netlify.svg" alt="" />
                    </span>
                </div>
            </div>

            /* Project 3 */

            <div class="connect_four" onmouseenter={onmouseenter.clone()} onmouseleave={onmouseleave.clone()}>
                <div class="project-inner">
                <h3><a>{"Connect Four"}</a></h3>
                <div class="project-nav">
                    <a href="https://connectfour-production.up.railway.app/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/connectFour" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toCF">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                    <video id="connectFour" class="connectFour" loop={true}>
                        <source src="/assets/video/connectFourWebM.webm" type="video/webm"/>
                        <source src="/assets/video/connectFourMP4.mp4" type="video/mp4"/>
                        <p>{"Your browser doesn't support HTML5 video."}</p>
                    </video>
                </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/typescript.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/react.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/nextjs.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/socketio.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/cypress.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/css3.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/github.svg" alt="" />
                    </span>
                </div>
            </div>

            /* Project - */

            /* <div class="portfolio_in_rust">
                <div class="project-inner">
                <h3><a>{"Portfolio in Rust"}</a></h3>
                <div class="project-nav">
                    <a href="https://github.com/co-decode/rust-portfolio-website" target="_blank">{"Source"}</a>
                </div>
                </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/rust.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/yew.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/sass.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/webassembly.svg" alt="" />
                    </span>
                </div>
            </div> */
            
            /* Project 4 */

            <div class="sheet_music_generator" onmouseenter={onmouseenter.clone()} onmouseleave={onmouseleave.clone()}>
                <div class="project-inner">
                <h3><a>{"Sheet Music Generator"}</a></h3>
                <div class="project-nav">
                    <a href="https://sheet-generator.netlify.app/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/sheetGenerator" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toSG">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                    <video id="sheetGen" loop={true}>
                    <source src="/assets/video/sheetGeneratorWEBM.webm" type="video/webm"/>
                    <source src="/assets/video/sheetGeneratorMP4.mp4" type="video/mp4"/>
                    <p>{"Your browser doesn't support HTML5 video."}</p>
                    </video>
                </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/javascript.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/css3.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/html5.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/netlify.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/github.svg" alt="" />
                    </span>
                </div>
            </div>

            /* Project 5 */

            <div class="arithmetic_trainer" /* onmouseenter={onmouseenter.clone()} onmouseleave={onmouseleave.clone()} */>
                <div class="project-inner">
                <h3><a>{"Arithmetic Trainer"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/arithmeticProject/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/arithmeticProject" target="_blank">{"Source"}</a>
                    <a href="#toA" onclick={onclick.clone()} >{"Video"}</a>
                </div>
                </div>

                // <div class="project-video">
                // <video id="arithmetic" loop={true}>
                //     <source src="/assets/video/ArhythmeticWebM.webm" type="video/webm"/>
                //     <source src="/assets/video/AryhthmeticMP42.mp4" type="video/mp4"/>
                //     <p>{"Your browser doesn't support HTML5 video."}</p>
                // </video>
                // </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/javascript.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/css3.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/html5.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/github.svg" alt="" />
                    </span>
                </div>
            </div>

            /* Project 6 */
    
            <div class="lift_equivalence_calculator" 
               /*  onmouseenter={onmouseenter.clone()} onmouseleave={onmouseleave.clone()} */>
                <div class="project-inner">
                <h3><a>{"Lift Equivalence Calculator"}</a></h3>
                <div class="project-nav">
                    <a href="https://equivalence-tabler.netlify.app/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/tabler" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toLEC">{"Video"}</a>
                </div>
                </div>

                // <div class="project-video">
                // <video id="liftEquivalence" loop={true}>
                //     <source src="/assets/video/liftEquivalenceWebM.webm" type="video/webm"/>
                //     <source src="/assets/video/liftEquivalenceMP4.mp4" type="video/mp4"/>
                //     <p>{"Your browser doesn't support HTML5 video."}</p>
                // </video>
                // </div>

                <div class="techTags absolute">
                    <span class="project-svg">
                        <img src="/assets/images/javascript.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/react.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/css3.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/html5.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/netlify.svg" alt="" />
                    </span>
                    <span class="project-svg">
                        <img src="/assets/images/github.svg" alt="" />
                    </span>
                </div>
            </div>
            
        </div>
    }
}