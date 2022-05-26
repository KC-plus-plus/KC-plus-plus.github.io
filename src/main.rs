use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
            <p>
                { "This site is a work in progress." }
                <br/>
                { "It was made with the Rust language and using the Yew framework, go check out github repo " }
                <a href="https://github.com/KC-plus-plus/KC-plus-plus.github.io">{ "here" }</a>
                { " for more info." }
            </p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
