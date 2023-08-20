use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {    
    html! {        
        <form>
            <br/>
            <div class="form-group">
                <label>{"Username: "}</label>
                <input type="text" />
            </div>
            <br/>
            <div class="form-group">
                <label>{"Password: "} </label>
                <input type="text" />
            </div>
            <br/>
            <button class="btn btn-primary" type="submit">{"submit"}</button>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}