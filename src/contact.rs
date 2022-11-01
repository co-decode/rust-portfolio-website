use yew::{function_component, Html, html};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div id="contact" class="contact">
        <div class="contact-inner">
        <form action="POST" method="post">
            <div class="contact-fields">
                <div class="contact-half-field">
                    <label for="name">{"NAME"}</label>
                    <input id="name" />
                </div>
                <div class="contact-half-field">
                    <label for="email">{"EMAIL"}</label>
                    <input id="email" />
                </div>
                <div class="contact-field">
                    <label for="message">{"MESSAGE"}</label>
                    <textarea id="message" rows="6"></textarea>
                </div>
                <div class="contact-field">
                    <div class="contact-captcha">
                        <div data-netlify-recaptcha="true"></div>
                    </div>
                </div>
            </div>
            <div class="contact-buttons">
                <button class="contact-submit">{"SEND MESSAGE"}</button>
                <button class="contact-clear">{"CLEAR"}</button>
            </div>
        </form>
        </div>
        </div>
    }
}