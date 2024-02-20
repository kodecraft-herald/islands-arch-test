use leptos::*;

use crate::components::common::functions::helpers::{get_input_size_class, join_icon};

use super::common_attributes::ComponentSize;

#[allow(non_snake_case)]
#[component]
pub fn InputText(
    name: String,
    label: String,
    #[prop(optional)]
    placeholder: String,
    #[prop(optional)]
    size: ComponentSize,
    // #[prop(optional)]
    // tab_index: u8,
    #[prop(optional)]
    custom_class: String,
    #[prop(optional)]
    icon: String,
) -> impl IntoView {

    let base_class = "input input-bordered w-full rounded hover:shadow-md".to_string();

    let size_class = get_input_size_class(size.clone(), base_class.clone());

    let input_component = if custom_class.is_empty() {
        view! {
            <input 
                type="text"
                name=name.clone()
                placeholder=placeholder
                class=size_class
                autocomplete
                required
            />
        }.into_view()
    } else {
        view! {
            <input 
                type="text"
                name=name.clone()
                placeholder=placeholder
                class=custom_class.clone()
                autocomplete
                required
            />
        }.into_view()
    };

    let input_component = join_icon(input_component, icon.clone(), size.clone());

    view! {
        <label for=name.clone() class="label">
            <span class="label-text">{label}</span>
        </label>
        {input_component}
    }
}


#[allow(non_snake_case)]
#[component]
pub fn InputPassword(
    name: String,
    label: String,
    #[prop(optional)]
    placeholder: String,
    #[prop(optional)]
    size: ComponentSize,
    // #[prop(optional)]
    // tab_index: u8,
    #[prop(optional)]
    custom_class: String,
    #[prop(optional)]
    icon: String
) -> impl IntoView {
    let base_class = "input input-bordered w-full rounded hover:shadow-md".to_string();

    let size_class = get_input_size_class(size.clone(), base_class.clone());

    let input_component = if custom_class.is_empty() {
        view! {
            <input 
                type="password"
                name=name.clone()
                placeholder=placeholder
                class=size_class
                autocomplete
                required
            />
        }.into_view()
    } else {
        view! {
            <input 
                type="password"
                name=name.clone()
                placeholder=placeholder
                class=custom_class.clone()
                autocomplete
                required
            />
        }.into_view()
    };

    let input_component = join_icon(input_component, icon.clone(), size.clone());

    view! {
        <label for=name.clone() class="label">
            <span class="label-text">{label}</span>
        </label>
        {input_component}
    }
}

#[allow(non_snake_case)]
#[component]
pub fn InputNumber(
    name: String,
	label: String,
    #[prop(optional)]
	size: ComponentSize,
	value: RwSignal<f64>,
	min: f64,
	step: f64,
	#[prop(optional)]
	custom_class: String
) -> impl IntoView {

    let function = move |e| {
        let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
        value.set(val);
    };

    let base_class = "input input-bordered w-full rounded hover:shadow-md".to_string();

    let size_class = get_input_size_class(size.clone(), base_class.clone());

    let input_component = if custom_class.is_empty() {
        view! {
            <input 
                type="number" 
                class = size_class
                name=name.clone()
                value = value
                min = move || min
                step = move || step
                on:change = function
                required
            />
        }.into_view()
    } else {
        view! {
            <input 
                type="number" 
                class = custom_class.clone()
                name=name.clone()
                value = value
                min = move || min
                step = move || step
                on:change = function
                required
            />
        }.into_view()
    };

    view! {
        <label for=name.clone() class="label">
            <span class="label-text">{label}</span>
        </label>
        {input_component}
    }
}

// #[allow(non_snake_case)]
// #[component]
// pub fn InputDatePicker (
// 	name: String,
// 	label: String,
//     signal: RwSignal<String>,
//     date_signal: RwSignal<DateTime<Utc>>
// 	value: String,
// 	size: ComponentSize,
//     #[prop(optional)]
// 	disabled: bool,
// 	#[prop(optional)]
// 	min: String,
// 	#[prop(optional)]
// 	custom_class: String
// ) -> impl IntoView {

//     let date_str_value = move || extract_date_as_string(date_signal.get());

//     let function = move |e| {
//         let val: String = event_target_value(&e);
//         let date_time = change_day(date_signal.get(), val).unwrap_or(date_signal.get());
//         date_signal.set(date_time);
//         signal.set(date_str_value());
//     };

//     let base_class = "input input-bordered w-full rounded hover:shadow-md".to_string();

//     let size_class = get_input_size_class(size.clone(), base_class.clone());

//     let input_component = if custom_class.is_empty() {
//         view! {
//             <input 
//                 type="number" 
//                 class = size_class
//                 name=name.clone()
//                 value = value
//                 min = move || min
//                 step = move || step
//                 on:change = function
//                 required
//             />
//         }.into_view()
//     } else {
//         view! {
//             <input 
//                 type="number" 
//                 class = custom_class.clone()
//                 name=name.clone()
//                 value = value
//                 min = move || min
//                 step = move || step
//                 on:change = function
//                 required
//             />
//         }.into_view()
//     };

//     view! {
//         <label for=name.clone() class="label">
//             <span class="label-text">{label}</span>
//         </label>
//         <input 
//             class = "input input-bordered w-full rounded hover:shadow-md"
//             type="date"
//             name={name.clone()}
//             prop:value = signal
//             min = min
//             prop:disabled = disabled 
//         />
//     }
// }