use std::{ops::Add, str::FromStr};

use leptos::*;

use crate::components::common::components::common_icons::Icon;

use super::common_attributes::ComponentSize;

#[allow(non_snake_case)]
#[component]
pub fn InputText(
    name: String,
    label: String,
    #[prop(optional)]
    placeholder: String,
    #[prop(optional)]
    size: Option<ComponentSize>,
    // #[prop(optional)]
    // tab_index: u8,
    #[prop(optional)]
    custom_class: Option<String>,
    #[prop(optional)]
    icon: Option<String>
) -> impl IntoView {
    let size_class = match size {
        None => "input input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::ExtraSmall) => "input input-xs input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Small) => "input input-sm input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Base) => "input input-md input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Large) => "input input-lg input-bordered w-full rounded hover:shadow-md",
    };

    let input_component = match custom_class {
        Some(custom_class) => {
            view! {
                <input type="text" name=name.clone() placeholder=placeholder class=custom_class autocomplete required/>
            }
        },
        None => {
            view! {
                <input type="text" name=name.clone() placeholder=placeholder class=size_class autocomplete required/>
            }
        }
    };

    let input_component = match icon {
        None => {
            input_component
        }.into_view(),
        Some(icon) => {
            view! {
                <div class="join">
                    <div class="rounded-md bg-success join-item flex items-center p-3">
                        <Icon
                            title=icon.clone()
                            size="h-5 w-5".to_string()
                        />
                    </div>
                    {input_component}
                </div>
            }.into_view()
        }
    };

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
    size: Option<ComponentSize>,
    // #[prop(optional)]
    // tab_index: u8,
    #[prop(optional)]
    custom_class: Option<String>,
    #[prop(optional)]
    icon: Option<String>
) -> impl IntoView {
    let size_class = match size {
        None => "input input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::ExtraSmall) => "input input-xs input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Small) => "input input-sm input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Base) => "input input-md input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Large) => "input input-lg input-bordered w-full rounded hover:shadow-md",
    };

    let input_component = match custom_class {
        Some(custom_class) => {
            view! {
                <input type="password" name=name.clone() placeholder=placeholder class=custom_class autocomplete required/>
            }
        },
        None => {
            view! {
                <input type="password" name=name.clone() placeholder=placeholder class=size_class autocomplete required/>
            }
        }
    };

    let input_component = match icon {
        None => {
            input_component
        }.into_view(),
        Some(icon) => {
            view! {
                <div class="join">
                    <div class="rounded-md bg-success join-item flex items-center p-3">
                        <Icon
                            title=icon.clone()
                            size="h-5 w-5".to_string()
                        />
                    </div>
                    {input_component}
                </div>
            }.into_view()
        }
    };

    view! {
        <label for=name.clone() class="label">
            <span class="label-text">{label}</span>
        </label>
        {input_component}
    }
}

#[allow(non_snake_case)]
#[component]
pub fn InputNumber<T>(
    name: String,
	label: String,
    #[prop(optional)]
	size: Option<ComponentSize>,
	value: RwSignal<T>,
	min: T,
	step: T,
	// #[prop(optional)]
	// tab_index: u8,
	#[prop(optional)]
	custom_class: Option<String>
) -> impl IntoView 
where
T: Add + Copy + 'static + FromStr + Default + IntoAttribute,
{
    let function = move |e| {
        let val = event_target_value(&e).parse::<T>().unwrap_or_default();
        value.set(val);
    };
    let size_class = match size {
        None => "input input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::ExtraSmall) => "input input-xs input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Small) => "input input-sm input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Base) => "input input-md input-bordered w-full rounded hover:shadow-md",
        Some(ComponentSize::Large) => "input input-lg input-bordered w-full rounded hover:shadow-md",
    };
    let input_component = match custom_class {
        Some(custom_class) => view! {
            <input type="number" class = custom_class
                name=name.clone()
                value = value
                min = move || min
                step = move || step
                on:change = function
            />
        },
        None => {
            view! {
                <input type="number" class=size_class
                    name=name.clone()
                    value = value
                    min = move || min
                    step = move || step
                    on:change = function
                    required
                />
            }
        }
    };

    view! {
        <label for=name.clone() class="label">
            <span class="label-text">{label}</span>
        </label>
        {input_component}
    }
}