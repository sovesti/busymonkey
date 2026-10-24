use std::time::Duration;

use dioxus::prelude::*;

use crate::monkey::model::parameters::{DurationSchema, FloatSchema};

pub(crate) mod duration;
pub(crate) mod float;

#[component]
pub fn Settings() -> Element {
    rsx! {
        form {
            class: "w-64",
            onsubmit: move |evt: FormEvent| async move {
                evt.prevent_default();
                debug!("{evt:?}");
            },
            float::Control { name: "speed", title: "Speed", initial: "0.0", schema: FloatSchema { from: 0.0, to: 100.0 } },
            duration::Control { name: "interval", title: "Interval", initial: "0.0", schema: DurationSchema { from: Duration::from_millis(1), to: Duration::from_secs(2) } },
            button {
                class: "rounded-m m-1 px-1 py-2 hover:bg-sky-100 cursor-pointer",
                r#type: "submit",
                "Apply"
            }
        }
    }
}
