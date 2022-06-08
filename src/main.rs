use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "听见音乐" }</h1>
    }

}

fn main() {
    yew::start_app::<App>();
}