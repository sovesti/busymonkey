use dioxus::prelude::*;

use crate::client::components::Slider;
use crate::monkey::model::parameters::FloatSchema;

#[component]
pub fn Control(name: String, title: String, initial: String, schema: FloatSchema) -> Element {
    let mut value = use_signal(|| initial.parse().unwrap_or_default());
    let label = use_memo(move || format!("{title}: {:.2}", value()));
    rsx! {
        input { class: "hidden", r#type: "number", id: name.clone(), name: name.clone(), value },
        label { for: name, "{label}" },
        Slider {
            horizontal: true,
            min: schema.from,
            max: schema.to,
            step: (schema.to - schema.from) / 100.0,
            id: name.clone(),
            on_value_change: move |updated: f64| value.set(updated),
            value: Some(value())
        }
    }
}
