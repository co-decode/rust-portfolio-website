use yew::{function_component, Html, html};

#[function_component(Projects)]
pub fn projects() -> Html { 
    html! {
        <div class="projects-container" id="projects">
            <a id="projectsMarker"></a>
            /* Project 1 */
            <div class="projects-1">
                <div class="project-inner">
                <h3><a>{"Weightlifting Log"}</a></h3>
                <div class="project-nav">
                    <a href="https://co-decode.github.io/liftingHistory-Client/" target="_blank">{"Site"}</a>
                    <a href="https://github.com/co-decode/liftingHistory" target="_blank">{"Source"}</a>
                    <a href="/videos#toLH" target="_blank">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                <video id="liftingRecord" loop={true}>
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
                    <a href="/videos#toLH" target="_blank">{"Video"}</a>
                </div>
                </div>

                <div class="project-video">
                    <video id="storeFront" loop={true}>
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
                    <a href="/videos#toLH" target="_blank">{"Video"}</a>
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
                    <a href="/videos#toLH" target="_blank">{"Video"}</a>
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
                    <a href="/videos#toLH" target="_blank">{"Video"}</a>
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
                    // <a href="/videos#toLH" target="_blank">{"Video"}</a>
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