use yew::prelude::*;
use yew::{function_component, html, Html};
use gloo::console::log;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: u8,
}

#[function_component]
fn Boton(index: &Props) -> Html {
    let active = use_state(|| false);
    let onclick = {
        let active = active.clone();
        log!(format!("me tocaste, soy {} y estoy: {}", index.index.clone(), *active));
        Callback::from(move |_| active.set(!*active))
    };
    
    html! {
        <button {onclick} class="led" style={
                            format!("{}",
                            if *active {"color: #333; background-color: #ccc; box-shadow: 0px 0px 50px 20px #ccc"} else
                            {"color: #ccc; background-color: #333; z-index:99"}
                            )}>{index.index.clone()}</button> 

    }
}
#[function_component]
pub fn App() -> Html {
        html! {
        <div class="carta">
            <b>{"Current value:"}</b>
            <div class="buttonCard">
                    <Boton index={1}/>
                    <Boton index={2}/>
                    <Boton index={3}/>
                    <Boton index={4}/>
                    <Boton index={5}/>
                    <Boton index={6}/>
                    <Boton index={7}/>
                    <Boton index={8}/>
                    <Boton index={9}/>
            </div>
        </div>
    }
}
