use yew::prelude::*;
use web_sys::HtmlDialogElement;
use web_sys::wasm_bindgen::JsCast;

#[function_component]
fn App() -> Html {
    let onclick = Callback::from(move |_| {
        let document = gloo::utils::document();
        let dialog: HtmlDialogElement = document.get_element_by_id("error").expect("couldn't find error dialog box").unchecked_into();
        dialog.show_modal();
    });

    html! {
        <div>
          <dialog id="error">
            <button autofocus={true}>{ "Close" }</button>
            <p>{ "This modal dialog has a groovy backdrop!" }</p>
          </dialog>
          <button {onclick}>{ "Show the dialog" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
