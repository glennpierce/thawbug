use std::{convert::Infallible, fmt, str::FromStr};

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::{Combobox, ComboboxOption};

pub trait EnumOptions<T> {
    // Generates the list of label-value pairs
    fn to_options() -> Vec<(T, String)>;

    // Retrieves the label for a specific value
    fn label(&self) -> String;
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Granularity {
    All,
    Second,
    Minute,
    FiveMinute,
    FifteenMinute,
    TwentyMinute,
    ThirtyMinute,
    Hour,
    DayQuarter,
    Day,
    Week,
    Month,
    Year,
}

impl FromStr for Granularity {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "all" => Granularity::All,
            "second" => Granularity::Second,
            "minute" => Granularity::Minute,
            "fiveminute" | "5 minute" => Granularity::FiveMinute,
            "fifteenminute" | "15 minute" => Granularity::FifteenMinute,
            "twentyminute" | "20 minute" => Granularity::TwentyMinute,
            "thirtyminute" | "30 minute" => Granularity::ThirtyMinute,
            "hour" => Granularity::Hour,
            "dayquarter" => Granularity::DayQuarter,
            "day" => Granularity::Day,
            "week" => Granularity::Week,
            "month" => Granularity::Month,
            "year" => Granularity::Year,
            _ => Granularity::Day, // Default value for unrecognized input
        })
    }
}

// Use FromStr in From<String> to prevent code duplication
impl From<String> for Granularity {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(Granularity::Day) // Default to `Day` if parsing fails
    }
}

impl EnumOptions<Granularity> for Granularity {
    fn to_options() -> Vec<(Granularity, String)> {
        vec![
            (Granularity::All, "All".into()),
            // (Granularity::Second, "Second".into()),
            // (Granularity::Minute, "Minute".into()),
            (Granularity::FiveMinute, "5 Minute".into()),
            // (Granularity::FifteenMinute, "15 Minute".into()),
            // (Granularity::TwentyMinute, "20 Minute".into()),
            (Granularity::ThirtyMinute, "30 Minute".into()),
            (Granularity::Hour, "Hour".into()),
            // (Granularity::DayQuarter, "Day Quarter".into()),
            (Granularity::Day, "Day".into()),
            (Granularity::Week, "Week".into()),
            (Granularity::Month, "Month".into()),
            (Granularity::Year, "Year".into()),
        ]
    }

    fn label(&self) -> String {
        match self {
            Granularity::All => "All".into(),
            Granularity::Second => "Second".into(),
            Granularity::Minute => "Minute".into(),
            Granularity::FiveMinute => "5 Minute".into(),
            Granularity::FifteenMinute => "15 Minute".into(),
            Granularity::TwentyMinute => "20 Minute".into(),
            Granularity::ThirtyMinute => "30 Minute".into(),
            Granularity::Hour => "Hour".into(),
            Granularity::DayQuarter => "Day Quarter".into(),
            Granularity::Day => "Day".into(),
            Granularity::Week => "Week".into(),
            Granularity::Month => "Month".into(),
            Granularity::Year => "Year".into(),
        }
    }
}

impl Default for Granularity {
    fn default() -> Self {
        Granularity::Day
    }
}

impl fmt::Display for Granularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_str = match self {
            Granularity::All => "all",
            Granularity::Second => "second",
            Granularity::Minute => "minute",
            Granularity::FiveMinute => "fiveminute",
            Granularity::FifteenMinute => "fifteenminute",
            Granularity::TwentyMinute => "twentyminute",
            Granularity::ThirtyMinute => "thirtyminute",
            Granularity::Hour => "hour",
            Granularity::DayQuarter => "dayquarter",
            Granularity::Day => "day",
            Granularity::Week => "week",
            Granularity::Month => "month",
            Granularity::Year => "year",
        };
        write!(f, "{}", variant_str)
    }
}

#[component]
pub fn GranularityCombobox(granularity: RwSignal<Granularity>) -> impl IntoView {
    // Dynamically generate options using the trait
    let options = Granularity::to_options();

    // Create a signal for selected options
    let selected_options = RwSignal::new(granularity.get_untracked().label());  // Initialize with the current granularity value's label

    // Effect to update granularity when selected_options changes
    Effect::new(move |_| {
        let value = selected_options.get();
        
        // Update granularity signal based on the new selection
        if let Some(new_value) = Granularity::from_str(&value).ok() {
            granularity.set(new_value);
        }
    });

    view! {
        <Combobox 
            selected_options=selected_options
            // Placeholder for the combobox, showing the current granularity's label
            placeholder={granularity.get_untracked().label()}
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
