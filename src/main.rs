use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let content = use_state(|| String::from("Hello, world!"));
    let spans = use_state(|| vec![]);
    let onmouseover = {
        let spans = spans.clone();
        Callback::from(move |_| {
            spans.set(vec![(
                bhava::Mask::Byte(5),
                bhava::Mask::Byte(9),
                "bg-red".to_owned(),
            )]);
        })
    };
    let onmouseout = {
        let spans = spans.clone();
        Callback::from(move |_| {
            spans.set(vec![]);
        })
    };

    html! {
        <>
            <bhava::Editor content={(*content).clone()} spans={(*spans).clone()} />













            <a href="#" {onmouseover}>{ "add span" }</a>
            { " " }
            <a href="#" {onmouseout}>{ "remove span" }</a>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
