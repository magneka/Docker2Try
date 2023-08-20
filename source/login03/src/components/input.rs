use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {    
    html! {
        <>   
            <br/>            
            <div class="form-group">
                <label>{props.label.clone()}</label>
                <input 
                    type={props.input_type.clone()} 
                    name={props.name.clone()} 
                />
            </div> 
        </>       
    }
}