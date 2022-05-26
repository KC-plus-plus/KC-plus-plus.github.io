use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
            <p>{ "This site is a work in progress." }</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
