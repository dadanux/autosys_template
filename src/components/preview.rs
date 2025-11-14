
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PreviewProps {
    pub text: String,
}

#[function_component(Preview)]
pub fn preview(props: &PreviewProps) -> Html {
    html! {
        <div class="preview">
            <h3>{ "Prévisualisation après remplacements" }</h3>
            <textarea
                readonly=true
                value={props.text.clone()}
                //style="width: 100%; height: 300px;"
                class="w-full h-96 p-2 border rounded bg-white text-sm"
                style="height: 700px;"
            />
        </div>
    }
}
