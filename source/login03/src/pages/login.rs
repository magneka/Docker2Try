use yew::prelude::*;

use crate::components::input::Input;

#[function_component(Login)]
pub fn login() -> Html {    
    html! {        
        <form>
            <Input input_type="text" name="username" label="Brukernavn:" />
            <Input input_type="password" name="password" label="Passord:" />
            
            <br/>
            <button class="btn btn-primary" type="submit">{"Lagre"}</button>
        </form>
    }
}