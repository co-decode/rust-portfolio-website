use yew::{function_component, Html, html, Callback, MouseEvent, /* use_effect */};
use yew_router::prelude::*;
// use gloo_events::EventListener;
use web_sys::{/* window, */ HtmlMediaElement, Element, EventTarget};
use wasm_bindgen::JsCast;

use crate::Route;

#[function_component(Projects)]
pub fn projects() -> Html { 
    let navigator = use_navigator().unwrap();

    let onclick:Callback<MouseEvent> = Callback::from(move |_| navigator.push(&Route::Videos));

    // let document = window().unwrap().document().unwrap();

        // let lifting_history = document.get_element_by_id("liftingRecord");
        // lifting_history.unwrap().muted();
    // let onmouseover = Callback::from(move |_| lifting_history.unwrap().dyn_into::<web_sys::HtmlMediaElement>().unwrap().play().unwrap());

        // let projectLH = document.get_element_by_id("projects-1") .unwrap()
        // .dyn_into::<web_sys::HtmlDivElement>()
        // .unwrap();

    // let on_mouseover = EventListener::new(&projectLH, "mouseover", move |_event| {
    //     let _video = lifting_history
    //     .unwrap()
    //     .dyn_into::<web_sys::HtmlMediaElement>()
    //     .unwrap();
    
    //     let promise=_video.play().unwrap();
        
    //     let _future = async{
    //         let _result= wasm_bindgen_futures::JsFuture::from(promise).await;
    //     };
    // });
   
    // use_effect({

    //     move || {
    //         let mut hover_listener = None;

    //         if let Some(element) = projectLH {
    //             let onmouseover = Callback::from(move |_: MouseEvent| {
    //                 lifting_history.unwrap().dyn_into::<web_sys::HtmlMediaElement>().unwrap().play()
    //             });

    //             let listener = EventListener::new(
    //                 &element,
    //                 "mouseover",
    //                 move |e| onmouseover.emit(e.clone())
    //             );
    //             hover_listener = Some(listener);
    //         }
    //         move || drop(hover_listener)
    //     }
    // });

    let on_mousein = {
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
    let on_mouseout = {
        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<Element>().ok());
            let video = input.unwrap().get_elements_by_tag_name("video")
                .get_with_index(0).expect("There should be a video element")
                .dyn_into::<HtmlMediaElement>().ok();
            if let Some(video) = video {
                // video.set_muted(true);
                // let _result = wasm_bindgen_futures::JsFuture::from(
                    video.pause().unwrap()
                // );
            }
        })
    };


    html! {
        <div class="projects-container" id="projects">
            <a id="projectsMarker"></a> 
            /* Project 1 */
            <div class="projects-1" onmouseenter={on_mousein} onmouseleave={on_mouseout}>
                <div class="project-inner">
                <h3><a>{"Weightlifting Log"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toLH">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                <video id="liftingRecord" loop={true} muted={true}>
                    <source src="/assets/video/liftingHistoryWebM.webm" type="video/webm"/>
                    <source src="/assets/video/liftingHistoryMP4.mp4" type="video/mp4"/>
                    <p>{"Your browser doesn't support HTML5 video."}</p>
                </video>
                </div>

                <div class="techTags">
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
            <div class="projects-2">
                <div class="project-inner">
                <h3><a>{"Fitness Store"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
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

                <div class="techTags">
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
            <div class="projects-3">
                <div class="project-inner">
                <h3><a>{"Sheet Music Generator"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
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

                <div class="techTags">
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
                        /* Project 4 */
            <div class="projects-4">
                <div class="project-inner">
                <h3><a>{"Arithmetic Trainer"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    <a href="#toA" onclick={onclick.clone()} >{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                <video id="arithmetic" loop={true}>
                    <source src="/assets/video/ArhythmeticWebM.webm" type="video/webm"/>
                    <source src="/assets/video/AryhthmeticMP42.mp4" type="video/mp4"/>
                    <p>{"Your browser doesn't support HTML5 video."}</p>
                </video>
                </div>

                <div class="techTags">
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
                        /* Project 5 */
            <div class="projects-5">
                <div class="project-inner">
                <h3><a>{"Lift Equivalence Calculator"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    <a onclick={onclick.clone()} href="#toLEC">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                <video id="liftEquivalence" loop={true}>
                    <source src="/assets/video/liftEquivalenceWebM.webm" type="video/webm"/>
                    <source src="/assets/video/liftEquivalenceMP4.mp4" type="video/mp4"/>
                    <p>{"Your browser doesn't support HTML5 video."}</p>
                </video>
                </div>

                <div class="techTags">
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
                        /* Project 6 */
            <div class="projects-6">
                <div class="project-inner">
                <h3><a>{"Portfolio in Rust"}</a></h3>
                <div class="project-nav">
                    // <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    // <a onclick={onclick.clone()} href="#toLH">{"Video"}</a>
                </div>
                </div>

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
                </div>
            </div>
        </div>
    }
}