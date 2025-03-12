use std::{convert::Infallible, fmt, str::FromStr};

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::{Combobox, ComboboxOption};

use super::granularity::EnumOptions;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Aggregate {
    Avg,
    Sum,
    Max,
    Min,
    Diff,
    Last,
}

impl FromStr for Aggregate {
    type Err = Infallible; // Use Infallible directly

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "avg" | "average" => Aggregate::Avg,
            "sum" => Aggregate::Sum,
            "max" | "maximum" => Aggregate::Max,
            "min" | "minimum" => Aggregate::Min,
            "diff" | "difference" => Aggregate::Diff,
            "last" => Aggregate::Last,
            _ => Aggregate::Avg, // Fallback as `Ok`
        })
    }
}

impl From<String> for Aggregate {
    fn from(s: String) -> Self {
        // Leverage the FromStr implementation to avoid code duplication
        Aggregate::from_str(&s).unwrap_or(Aggregate::Avg) // Default to `Aggregate::Avg` on parse error
    }
}

impl EnumOptions<Aggregate> for Aggregate {
    // Returns a list of (value, label) pairs
    fn to_options() -> Vec<(Aggregate, String)> {
        vec![
            (Aggregate::Avg, "Average".into()),
            (Aggregate::Sum, "Sum".into()),
            (Aggregate::Max, "Maximum".into()),
            (Aggregate::Min, "Minimum".into()),
            (Aggregate::Diff, "Difference".into()),
            (Aggregate::Last, "Last".into()),
        ]
    }

    // Returns the label for a specific `Aggregate` variant
    fn label(&self) -> String {
        match self {
            Aggregate::Avg => "Average".into(),
            Aggregate::Sum => "Sum".into(),
            Aggregate::Max => "Maximum".into(),
            Aggregate::Min => "Minimum".into(),
            Aggregate::Diff => "Difference".into(),
            Aggregate::Last => "Last".into(),
        }
    }
}

impl Default for Aggregate {
    fn default() -> Self {
        Aggregate::Avg
    }
}

impl fmt::Display for Aggregate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            Aggregate::Avg => "avg",
            Aggregate::Sum => "sum",
            Aggregate::Max => "max",
            Aggregate::Min => "min",
            Aggregate::Diff => "diff",
            Aggregate::Last => "last",
        };
        write!(f, "{}", variant_str)
    }
}

#[component]
pub fn AggregationCombobox(aggregate: RwSignal<Aggregate>) -> impl IntoView {
    // Dynamically generate options using the trait
    let options = Aggregate::to_options();

    // Create a signal for selected options
    let selected_options = RwSignal::new(aggregate.get_untracked().label());  // Initialize with the current aggregate value's label

    // Effect to update aggregate when selected_options changes
    Effect::new(move |_| {
        let value = selected_options.get_untracked();
        
        // log::error!("AggregationCombobox value: {:?} {:?}", value, Aggregate::from_str(&value));

        // Update aggregate signal based on the new selection
        if let Some(new_value) = Aggregate::from_str(&value).ok() {
            aggregate.set(new_value);
        }
    });

    view! {
        <Combobox 
            selected_options
            // Placeholder for the combobox, showing the current aggregate's label
            placeholder={aggregate.get_untracked().label()}
        >
            {options.into_iter().map(|(value, label)| {
                view! {
                    <ComboboxOption 
                        value={value.to_string()} 
                        text={label} 
                    />
                }
            }).collect_view()}
        </Combobox>
    }
}
