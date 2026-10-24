use std::time::Duration;

use dioxus::prelude::*;

use crate::client::components::Slider;
use crate::monkey::model::parameters::DurationSchema;

#[component]
pub fn Control(name: String, title: String, initial: String, schema: DurationSchema) -> Element {
    let initial = initial.parse().unwrap_or_default();
    let mut value = use_signal(|| initial);
    let label = use_memo(move || format!("{title}: {}", format_time(from_nanos(value()))));
    rsx! {
        input { class: "hidden", r#type: "number", id: name.clone(), name: name.clone(), value },
        label { for: name, "{label}" },
        Slider {
            horizontal: true,
            min: into_nanos(schema.from),
            max: into_nanos(schema.to),
            step: into_nanos(schema.to - schema.from) / 100.0,
            id: name.clone(),
            on_value_change: move |updated: f64| value.set(updated),
            value: Some(value())
        }
    }
}

fn into_nanos(duration: Duration) -> f64 {
    duration.as_nanos() as f64
}

fn from_nanos(nanos: f64) -> Duration {
    Duration::from_nanos(nanos as u64)
}

fn format_time(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2} s", (duration.as_millis() as f64) / 1000.0)
    } else {
        format!("{} ms", duration.as_millis())
    }
}
