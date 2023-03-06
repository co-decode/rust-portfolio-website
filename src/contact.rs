use yew::{function_component, Html, Callback, html};
use yew::prelude::*;
use web_sys::{Event, EventTarget, console, Element, HtmlInputElement, HtmlTextAreaElement};
use wasm_bindgen::{JsCast, /* JsValue */};
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Deserialize)]
struct Resp {
    success: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mail {
    name: String,
    email: String,
    message: String
}

#[function_component(Contact)]
pub fn contact() -> Html {
    let contact_name:UseStateHandle<String> = use_state(||"".into());
    let contact_email:UseStateHandle<String> = use_state(||"".into());
    let contact_message:UseStateHandle<String> = use_state(||"".into());

    let name_ref = use_node_ref();
    let email_ref = use_node_ref();
    let message_ref = use_node_ref();

    let n_ref = name_ref.clone();
    let e_ref = email_ref.clone();
    let m_ref = message_ref.clone();

    // let dep_name = contact_name.clone();
    // let dep_email = contact_email.clone();
    // let dep_message = contact_message.clone();
    // let clear_form = || {
    //     contact_name.set("".to_string());
    // };

    let onchange:Callback<Event> = {
        let contact_name = contact_name.clone();
        let contact_email = contact_email.clone();
        let contact_message = contact_message.clone();
        Callback::from(move|change_event: Event| {
            let target:EventTarget = change_event.target().unwrap();
            let element:Element = target.dyn_into::<Element>().ok().unwrap();
            let element_id = element.id();
            let node_name = element.node_name();
            if node_name == "INPUT" {
                let input_el: HtmlInputElement = element.dyn_into::<HtmlInputElement>().ok().unwrap();
                if element_id == "name"
                    {contact_name.set(input_el.value());}
                else if element_id == "email"
                    {contact_email.set(input_el.value())}
            }
            else if node_name == "TEXTAREA" {
                let textarea_el: HtmlTextAreaElement = element.dyn_into::<HtmlTextAreaElement>().ok().unwrap();
                contact_message.set(textarea_el.value());
            }
        })
    };

    let onclick:Callback<MouseEvent> = {
        let contact_name = contact_name.clone();
        let contact_email = contact_email.clone();
        let contact_message = contact_message.clone();
        

        Callback::from(move|_| {
            contact_name.set("".to_string());
            contact_email.set("".to_string());
            contact_message.set("".to_string());
        })
    };

    // use_effect_with_deps(move |_| {
    //     console::log_1(&JsValue::from_str(&*contact_name.to_string()));
    // }, (dep_name, dep_email, dep_message));

    let onsubmit:Callback<SubmitEvent> = {
        let data = Mail {
            name: String::from(&*contact_name),
            email: String::from(&*contact_email),
            message: String::from(&*contact_message),
        };
        
        Callback::from(move|submit_event: SubmitEvent| {
            let contact_name = contact_name.clone();
            let contact_email = contact_email.clone();
            let contact_message = contact_message.clone();
            
            let name_ref = name_ref.clone();
            let email_ref = email_ref.clone();
            let message_ref = message_ref.clone();

            let json_data = serde_json::to_string(&data).ok().unwrap();
            Event::prevent_default(&submit_event); 
            wasm_bindgen_futures::spawn_local(async move {
            let fetched_script:Resp = Request::post("https://cody-ross.net/contact.php")
                .header("Content-Type", "application/json")
                .body(json_data)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            if fetched_script.success == 1 {
                contact_name.set("".to_string());
                contact_email.set("".to_string());
                contact_message.set("".to_string());
                name_ref.cast::<HtmlInputElement>().unwrap().set_value("");
                email_ref.cast::<HtmlInputElement>().unwrap().set_value("");
                message_ref.cast::<HtmlTextAreaElement>().unwrap().set_value("");
            }
            else if fetched_script.success == 2 {console::log_1(&"The message was unable to be sent".into())}
            else if fetched_script.success == 0 {console::log_1(&"Please ensure the input is valid".into())}
            else {console::log_1(&"Something went wrong...".into())}
            });
        })
    };
    

    html! {
        <div id="contact" class="contact">
        <div class="contact-inner">
        // action="https://formsubmit.co/c2d4746b27e005dde2bef9cea234c549" method="POST"
        <form onsubmit={onsubmit.clone()} >
            <div class="contact-fields">
                <div class="contact-half-field">
                    <label for="name">{"NAME"}</label>
                    <input onchange={onchange.clone()} ref={n_ref} id="name" name="Name" />
                </div>
                <div class="contact-half-field">
                    <label for="email">{"EMAIL"}</label>
                    <input onchange={onchange.clone()} ref={e_ref} id="email" name="Email" />
                </div>
                <div class="contact-field">
                    <label for="message">{"MESSAGE"}</label>
                    <textarea onchange={onchange.clone()} ref={m_ref} id="message" name="Message" rows="6"/>
                </div>
                </div>
            <div class="contact-buttons">
                <button type="submit" class="contact-submit">{"SEND MESSAGE"}</button>
                <button type="reset" class="contact-clear" onclick={onclick.clone()}>{"CLEAR"}</button>
            </div>
        </form>
        </div>

        </div>
    }
}