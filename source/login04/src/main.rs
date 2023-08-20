use yew::prelude::*;

mod pages;
mod components;

#[function_component(App)]
fn app() -> Html {    
    html! {  
        <>      
            <h1>{"Login"}</h1>
            <pages::login::Login />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}