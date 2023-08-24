https://www.programiz.com/rust/print-output
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html
https://yew.rs/docs/getting-started/introduction

// Siste eksempel
https://yew.rs/docs/concepts/router

Hello World
1. Cargo new hello
2. gå til hello folder
3. cargo run

Første YEW eksempel er en ren hello world.

1. cargo new yew-app
2. rediger cargo toml
   [dependencies]
   yew = { version = "0.20", features = ["csr"] }
3. Rediger main.rs

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
4. lag en index.html
5. trunk serve
6. cargo clean 
7. trunk clean

YEW - login01
cd /source
cargo new login01
cd login01
kopier index.html
kopier dependencies
kopier main.rs
Legg inn i html cdn for bootstrap:
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous">
Legg innen funksjon som returnerer html
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

YEW Login02
Skille ut i egen komponent file
Lag subfolder ./src/pages
legg inn to filer
mod.rs
pages.rs

YEW Login03
Lage en egen komponent for input

YEW Login04
Legge inn state
