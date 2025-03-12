use std::str::FromStr;

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use thaw::{Combobox, ComboboxOption};

#[component]
pub fn ReportStyleCombobox(report_style: RwSignal<Option<String>>) -> impl IntoView {
    // Static options for the combobox
    let options = vec![
        ("report_style1", "Report Style 1"),
        ("report_style2", "Report Style 2"),
        ("report_style3", "Report Style 3"),
    ];

    view! {
        <Combobox selected_options=report_style placeholder="Select an aggregation">
            {options.into_iter().map(|(value, text)| {
                view! {
                    <ComboboxOption value={value.to_string()} text={text.to_string()} />
                }
            }).collect_view()}
        </Combobox>
    }
}