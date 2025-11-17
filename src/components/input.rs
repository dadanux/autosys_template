
use yew::prelude::*;
use web_sys::{console,  HtmlInputElement};


pub enum Msg {
    Hover,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub placeholder: String,
    pub name: String,
    pub value: String,
    pub on_update: Callback<(String, String)>,
}

pub struct InputComponent {
    node_ref: NodeRef,
}

impl Component for InputComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
    Self{
        node_ref: NodeRef::default(),
    }

    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hover => {
                console::log_1(&self.node_ref.cast::<HtmlInputElement>().unwrap().value().into());
                if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                ctx.props().on_update.emit((ctx.props().name.clone(), input.value()));
                console::log_1(&input.value().to_string().into());
            }
            false
        }
    }
}

    fn view(&self, ctx: &Context<Self>) -> Html {
        let placeholder = ctx.props().placeholder.clone();
        let name = ctx.props().name.clone();
        html! {
             <div class="grid auto-cols-max grid-flow-col p-1">
            <label>{ name }</label>
            <input
                type="text"
                class="outline-2 outline-offset-2 outline-blue-500 rounded"
                ref={self.node_ref.clone()}
                placeholder={placeholder}
                oninput={ctx.link().callback(|_| Msg::Hover)}
            />
            </div>
        }
    }
}

// #[function_component(MachineEditor)]
// pub fn machine_editor(props: &MachineEditorProps) -> Html {
//     let replacements = use_state(|| props.replacements.clone());

//     let on_change = {
//         let replacements = replacements.clone();
//         let on_update = props.on_update.clone();
//         Callback::from(move |(name, value): (String, String)| {
//             let mut new_map = (*replacements).clone();
//             new_map.insert(name.clone(), value.clone());
//             console::log_1(&format!("✅ Machine modifiée: {} → {}", name, value).into());
//             replacements.set(new_map.clone());
//             console::log_1(&format!("🔄 État des remplacements mis à jour: {:?}", replacements).into())  ;
//             on_update.emit(new_map);
//         })
//     };

//     html! {
//         <div class="">
//             <h3>{ "Machines:" }</h3>
//             {
//                 for props.machines.iter().map(|name| {
//                     let value = replacements.get(name).cloned().unwrap_or_default();
//                     html! {
//                         <div class="grid auto-cols-max grid-flow-col p-1">
//                             <label>{ name }</label>
//                             <input
//                                 type="text"
//                                 class="outline-2 outline-offset-2 outline-blue-500 rounded"
//                                 value={value.clone()}
//                                 placeholder="variable ex: machine_iis"
//                                 oninput={
//                                     let name = name.clone();
//                                     let on_change = on_change.clone();
//                                     Callback::from(move |e: InputEvent| {
//                                         let input: web_sys::HtmlInputElement = e.target_unchecked_into();
//                                         on_change.emit((name.clone(), input.value()));
//                                     })
//                                 }
//                             />
//                         </div>
//                     }
//                 })
//             }
//         </div>
//     }
// }
