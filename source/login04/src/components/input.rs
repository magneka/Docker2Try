use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub input_type: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {    
    html! {
        <>   
            <br/>            
            <div class="form-group">
                <label>{props.label.clone()}</label>
                <input 
                    class="form-control"
                    type={props.input_type.clone()} 
                    name={props.name.clone()} 
                    value={props.value.clone()}
                    onchange={props.onchange.clone()}
                />
            </div> 
        </>       
    }
}