use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {    
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <p>{"Ser du dette, har du din første yew applikasjon oppe!"}</p>            
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}