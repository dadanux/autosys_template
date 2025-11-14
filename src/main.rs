use autosys_template::components;
use autosys_template::transform::process_text;
use components::preview::Preview;
use components::machine_editor::MachineEditor;
use components::owner_editor::OwnerEditor;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, FileReader, HtmlInputElement,};
use web_sys::{Blob, Event,  HtmlAnchorElement, Url, DragEvent};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let file_content = use_state(|| String::new());
    let owners = use_state(|| HashSet::<String>::new());
    let machines = use_state(|| HashSet::<String>::new());
    let replacements = use_state(|| HashMap::new());
    let preview_text = use_state(|| "".to_string());
    let node_ref = use_node_ref();

    fn extract_names_from_text(text: &str) -> (HashSet<String>, HashSet<String>) {
        let mut machine_list = HashSet::<String>::new();
        let mut owner_list = HashSet::<String>::new();
        for line in text.lines() {
            if line.trim().starts_with("machine:") {
                // let mut split2 = line.splitn(2,':');
                if let Some(name) = line.splitn(2, ':').nth(1) {
                    machine_list.insert(name.trim().to_string());
                }
            }
            if line.trim().starts_with("owner:") {
                if let Some(name) = line.splitn(2, ':').nth(1) {
                    owner_list.insert(name.trim().to_string());
                }
            }
        }
        (machine_list, owner_list)
    }

    let drop = {
        let node_ref = node_ref.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(input) = node_ref.cast::<HtmlInputElement>() {
                if let Some(files) = e.data_transfer().and_then(|dt| dt.files()) {
                    input.set_files(Some(&files));
                    let event = Event::new("change").unwrap();
                    input.dispatch_event(&event).unwrap();
                }
            }
        })
    };

    let onchange = {
        let file_content = file_content.clone();
        let machines = machines.clone();
        let owners = owners.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let reader = FileReader::new().unwrap();
                    let fc = file_content.clone();
                    let m = machines.clone();
                    let o = owners.clone();

                    let onloadend = Closure::wrap(Box::new(move |e: web_sys::ProgressEvent| {
                        let reader: FileReader = e.target().unwrap().dyn_into().unwrap();
                        let result = reader.result().unwrap();
                        let text = result.as_string().unwrap();
                        extract_names_from_text(&text);
                        let (machine_list, owner_list) = extract_names_from_text(&text);
                        fc.set(text);
                        m.set(machine_list);
                        o.set(owner_list);
                    }) as Box<dyn FnMut(_)>);

                    reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                    reader.read_as_text(&file).unwrap();
                    onloadend.forget();
                }
            }
        })
    };

    let _update_machine = {
        let replacements = replacements.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut map = (*replacements).clone();
            map.insert(name.clone(), value.clone());
            console::log_1(&format!("✅ Machine modifiée: {} → {}", name, value).into());
            replacements.set(map);
        })
    };

//    let preview = {
//         console::log_1(&"🔄 Mise à jour de la prévisualisation".into());
//         console::log_1(&format!("🔄 État des remplacements mis à jour: {:?}", replacements).into());
//         process_text(&file_content, &(*replacements))
         
//     };
    
    let on_update = {
        let source_text = file_content.clone();
        let preview_text = preview_text.clone();
        let replacements = replacements.clone();
        console::log_1(&format!("🔄 État des remplacements avant: {:?}", &replacements).into())  ;
        Callback::from(move |new_map: HashMap<String, String>| {
            replacements.set(new_map.clone());
            console::log_1(&format!("🔄 État des remplacements apres: {:?}", &replacements).into())  ;
            let new_preview = process_text(&source_text, &replacements);
            preview_text.set(new_preview);
        })
    };

//  pub enum Msg {
//     Files(Option<web_sys::FileList>),
// }

    let on_export = {
        let preview = preview_text.clone();
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let blob_parts = js_sys::Array::new();
            blob_parts.push(&wasm_bindgen::JsValue::from_str(&preview));

            let blob = Blob::new_with_str_sequence(&blob_parts).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();

            let a = document
                .create_element("a")
                .unwrap()
                .dyn_into::<HtmlAnchorElement>()
                .unwrap();
            a.set_href(&url);
            a.set_download("plan.j2");
            a.click();

            Url::revoke_object_url(&url).unwrap();
        })
    };
        let noop_drag = Callback::from(|e: DragEvent| {
            e.prevent_default();
        });

   

    html! {
        <div>
            <h1>{"Éditeur JIL avec mise à jour dynamique"}</h1>
                <div id="wrapper">
                <label for="file-upload">
                <div
                        id="drop-container"
                        ondrop={&drop}
                        ondragover={&noop_drag}
                        ondragenter={&noop_drag}
                 >
                <input
                    id="file-upload"
                    type="file"
                    accept="text/*"
                    multiple={true}
                    onchange={onchange}
                    ref={node_ref}
                />
                     <i class="fa fa-cloud-upload"></i>
                    <p>{"Depose du jil"}</p>
                  
                    </div>
                </label>
      
                // <div id="preview-area">
                //     { for self.files.iter().map(Self::view_file) }
                // </div>
            </div>
            <button class="text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-300 dark:focus:ring-blue-800 shadow-lg shadow-blue-500/50 dark:shadow-lg dark:shadow-blue-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2 "
             type="button" onclick={on_export}>{ "Exporter le fichier modifié" }</button>
            <OwnerEditor
              owners={(*owners).clone()}
              replacements={(*replacements).clone()}
              on_update={on_update.clone()}
              />
            <MachineEditor
                machines={(*machines).clone()}
                replacements={(*replacements).clone()}
                on_update={on_update.clone()}
            />  


            <div class="grid grid-cols-2 gap-4">
                <div>
                   // <h3 class="text-lg font-semibold mb-2">{"Fichier original (.jil)"}</h3>
                    <h3>{ "Prévisualisation avant remplacements" }</h3>
                    <textarea
                        readonly=true
                        value={file_content.to_string()}
                        class="w-full h-96 p-2 border rounded bg-white text-sm"
                        style="height: 700px;"
                    />
                </div>
                <div>
                     <Preview text={(*preview_text).clone()} />
                   //  <Preview text={(preview).clone()} />
                </div>
            </div>

            // <h2>{"Prévisualisation du fichier"}</h2>
            // <pre>{ file_content.to_string() }</pre>
            // <Preview text={preview.clone()} />

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
