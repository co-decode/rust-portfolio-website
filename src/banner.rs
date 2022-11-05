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
            // <div class="banner-skills-container-inner">
            <div class="banner-top-container">
                <div class="banner-skills-top">
                    <span class="banner-svg">
                    <img src="/assets/images/javascript.svg" alt="" />
                    <p>{"Javascript"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/typescript.svg" alt="" />
                        <p>{"Typescript"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/react.svg" alt="" />
                        <p>{"React"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/html5.svg" alt="" />
                        <p>{"HTML5"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/css3.svg" alt="" />
                        <p>{"CSS3"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/sass.svg" alt="" />
                        <p>{"SASS"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/d3.svg" alt="" />
                        <p>{"D3"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/jest.svg" alt="" />
                        <p>{"Jest"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/mswjs.svg" alt="" />
                        <p>{"MockSW"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/git.svg" alt="" />
                        <p>{"Git"}</p>
                    </span>
                    /* The items are doubled for infinite scroll animation */
                    <span class="banner-svg duplicate">
                    <img src="/assets/images/javascript.svg" alt="" />
                    <p>{"Javascript"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/typescript.svg" alt="" />
                        <p>{"Typescript"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/react.svg" alt="" />
                        <p>{"React"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/html5.svg" alt="" />
                        <p>{"HTML5"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/css3.svg" alt="" />
                        <p>{"CSS3"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/sass.svg" alt="" />
                        <p>{"SASS"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/d3.svg" alt="" />
                        <p>{"D3"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/jest.svg" alt="" />
                        <p>{"Jest"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/mswjs.svg" alt="" />
                        <p>{"MockSW"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/git.svg" alt="" />
                        <p>{"Git"}</p>
                    </span>
                </div>
            </div>
                <div class="banner-skills-bottom">
                    <span class="banner-svg">
                    <img src="/assets/images/nodejs.svg" alt="" />
                    <p>{"Node"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/express.svg" alt="" />
                        <p>{"Express"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/postgresql.svg" alt="" />
                        <p>{"PostgreSQL"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/mongodb.svg" alt="" />
                        <p>{"MongoDB"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/graphql.svg" alt="" />
                        <p>{"GraphQL"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/bcrypt.svg" alt="" />
                        <p>{"Bcrypt"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/passportjs.svg" alt="" />
                        <p>{"Passport"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/docker.svg" alt="" />
                        <p>{"Docker"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/railway.svg" alt="" />
                        <p>{"Railway"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/netlify.svg" alt="" />
                        <p>{"Netlify"}</p>
                    </span>
                    <span class="banner-svg">
                        <img src="/assets/images/github.svg" alt="" />
                        <p>{"Github"}</p>
                    </span>
                    /* The items are doubled for infinite scroll animation */
                    <span class="banner-svg duplicate">
                    <img src="/assets/images/nodejs.svg" alt="" />
                    <p>{"Node"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/express.svg" alt="" />
                        <p>{"Express"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/postgresql.svg" alt="" />
                        <p>{"PostgreSQL"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/mongodb.svg" alt="" />
                        <p>{"MongoDB"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/graphql.svg" alt="" />
                        <p>{"GraphQL"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/bcrypt.svg" alt="" />
                        <p>{"Bcrypt"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/passportjs.svg" alt="" />
                        <p>{"Passport"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/docker.svg" alt="" />
                        <p>{"Docker"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/railway.svg" alt="" />
                        <p>{"Railway"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/netlify.svg" alt="" />
                        <p>{"Netlify"}</p>
                    </span>
                    <span class="banner-svg duplicate">
                        <img src="/assets/images/github.svg" alt="" />
                        <p>{"Github"}</p>
                    </span>
                </div>
            </div>
            </div>
        // </div>
        </div>
    }
}