use std::time::Duration;

use dioxus::prelude::*;

use crate::client::components::Slider;
use crate::monkey::model::parameters::DurationSchema;

#[component]
pub fn Control(
    name: String,
    title: String,
    initial: serde_json::Value,
    schema: DurationSchema,
) -> Element {
    let mut value = use_signal(|| {
        initial
            .as_str()
            .and_then(|val| val.parse().ok())
            .unwrap_or_default()
    });
    let label = use_memo(move || format!("{title}: {}", format_time(from_millis(value()))));
    rsx! {
        input { class: "hidden", r#type: "number", id: name.clone(), name: name.clone(), value },
        label { for: name, "{label}" },
        Slider {
            horizontal: true,
            min: into_millis(schema.from),
            max: into_millis(schema.to),
            step: into_millis(schema.to - schema.from) / 100.0,
            id: name.clone(),
            on_value_change: move |updated: f64| value.set(updated as u64),
            value: Some(value() as f64)
        }
    }
}

fn into_millis(duration: Duration) -> f64 {
    duration.as_millis() as f64
}

fn from_millis(millis: u64) -> Duration {
    Duration::from_millis(millis)
}

fn format_time(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2} s", (duration.as_millis() as f64) / 1000.0)
    } else {
        format!("{} ms", duration.as_millis())
    }
}
