use yew::{function_component, Html, html};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div id="contact" class="contact">
        <div class="contact-inner">
        <form>
            <div class="contact-fields">
                <div class="contact-half-field">
                    <label for="name">{"Name"}</label>
                    <input id="name" />
                </div>
                <div class="contact-half-field">
                    <label for="email">{"Email"}</label>
                    <input id="email" />
                </div>
                <div class="contact-field">
                    <label for="message">{"Message"}</label>
                    <textarea id="message" rows="6"></textarea>
                </div>
                <div class="contact-field">
                    <div class="contact-captcha">
                    </div>
                </div>
            </div>
            <div class="contact-buttons">
                <button class="contact-submit">{"Send Message"}</button>
                <button class="contact-clear">{"Clear"}</button>
            </div>
        </form>
        </div>
        </div>
    }
}