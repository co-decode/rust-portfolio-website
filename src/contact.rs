use yew::{function_component, Html, html};

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div id="contact" class="contact">
        <div class="contact-inner">
        <form action="https://formsubmit.co/ed4e6df97742ba708b939b3af02cc62a" method="POST" >
            <div class="contact-fields">
                <div class="contact-half-field">
                    <label for="name">{"NAME"}</label>
                    <input id="name" name="Name" />
                </div>
                <div class="contact-half-field">
                    <label for="email">{"EMAIL"}</label>
                    <input id="email" name="Email" />
                </div>
                <div class="contact-field">
                    <label for="message">{"MESSAGE"}</label>
                    <textarea id="message" name="Message" rows="6"></textarea>
                </div>
                </div>
            <div class="contact-buttons">
                <button type="submit" class="contact-submit">{"SEND MESSAGE"}</button>
                <button type="reset" class="contact-clear">{"CLEAR"}</button>
            </div>
        </form>
        </div>
        </div>
    }
}