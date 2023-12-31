use yew::prelude::*;

use web_sys::HtmlInputElement;
use crate::components::input::*;
use gloo_console::log;

#[function_component(LoginForm)]
pub fn login_form() -> Html {  
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();

    let username_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handle.set(input.value())
        }
    });

    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let password_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();        
        if let Some(input) = target {
            password_handle.set(input.value())
        }
    });

    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        log!("Submitting for");
        log!(cloned_username.clone());
        log!(cloned_password.clone());
    });

    html! {
        <form onsubmit={onsubmit}>
            <Input 
                input_type="text" 
                name="username" 
                label="Brukernavn:" 
                value={username}
                onchange={username_changed}
            />
            <Input 
                input_type="password" 
                name="password" 
                label="Passord:" 
                value={password}
                onchange={password_changed}
            />
            
            <br/>
            <button class="btn btn-primary" type="submit">{"Lagre"}</button>
        </form>      
    }
}