
use std::collections::HashMap;
use std::collections::HashSet;
use yew::prelude::*;
use web_sys::{console, InputEvent};

#[derive(Properties, PartialEq)]
pub struct MachineEditorProps {
    pub machines: HashSet<String>,
    pub replacements: HashMap<String, String>,
    pub on_update: Callback<HashMap<String, String>>,
}

#[function_component(MachineEditor)]
pub fn machine_editor(props: &MachineEditorProps) -> Html {
    let replacements = use_state(|| props.replacements.clone());

    let on_change = {
        let replacements = replacements.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut new_map = (*replacements).clone();
            new_map.insert(name.clone(), value.clone());
            console::log_1(&format!("✅ Machine modifiée: {} → {}", name, value).into());
            replacements.set(new_map.clone());
            console::log_1(&format!("🔄 État des remplacements mis à jour: {:?}", replacements).into())  ;
            on_update.emit(new_map);
        })
    };

    html! {
        <div class="">
            <h3>{ "Machines:" }</h3>
            {
                for props.machines.iter().map(|name| {
                    let value = replacements.get(name).cloned().unwrap_or_default();
                    html! {
                        <div class="grid auto-cols-max grid-flow-col p-1">
                            <label>{ name }</label>
                            <input
                                type="text"
                                class="outline-2 outline-offset-2 outline-blue-500 rounded"
                                value={value.clone()}
                                placeholder="variable ex: machine_iis"
                                oninput={
                                    let name = name.clone();
                                    let on_change = on_change.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        on_change.emit((name.clone(), input.value()));
                                    })
                                }
                            />
                        </div>
                    }
                })
            }
        </div>
    }
}
