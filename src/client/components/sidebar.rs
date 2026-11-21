use crate::client::components::ButtonSize;

use super::button::{Button, ButtonVariant};
use dioxus::core::use_drop;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronLeft, ChevronRight};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use dioxus_primitives::use_controlled;

#[css_module("/src/client/components/sidebar.css")]
struct Styles;

// constants
const SIDEBAR_WIDTH: &str = "10rem";
const SIDEBAR_WIDTH_ICON: &str = "2.5rem";
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarState {
    Expanded,
    #[default]
    Collapsed,
}

impl SidebarState {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarState::Expanded => "expanded",
            SidebarState::Collapsed => "collapsed",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarSide {
    #[default]
    Left,
}

impl SidebarSide {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarSide::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
}

impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum SidebarCollapsible {
    #[default]
    Icon,
}

impl SidebarCollapsible {
    pub fn as_str(&self) -> &'static str {
        match self {
            SidebarCollapsible::Icon => "icon",
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct SidebarCtx {
    pub state: Memo<SidebarState>,
    pub side: Signal<SidebarSide>,
    // From use_controlled:
    open: Memo<bool>,
    set_open: Callback<bool>,
    // Mobile state:
    open_mobile: Signal<bool>,
}

impl SidebarCtx {
    /// Toggle the sidebar open/closed state
    pub fn toggle(&self) {
        self.set_open.call(!self.open());
    }

    /// Get the current open state (desktop)
    pub fn open(&self) -> bool {
        self.open.cloned()
    }
}

pub fn use_sidebar() -> SidebarCtx {
    use_context::<SidebarCtx>()
}

#[component]
pub fn SidebarProvider(
    #[props(default = true)] default_open: bool,
    #[props(default)] open: ReadSignal<Option<bool>>,
    #[props(default)] on_open_change: Callback<bool>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let side = use_signal(|| SidebarSide::Left);
    let open_mobile = use_signal(|| false);

    let (open, set_open) = use_controlled(open, default_open, on_open_change);

    let state = use_memo(move || {
        if open() {
            SidebarState::Expanded
        } else {
            SidebarState::Collapsed
        }
    });

    let ctx = SidebarCtx {
        state,
        side,
        open,
        set_open,
        open_mobile,
    };

    use_context_provider(|| ctx);

    use_effect(move || {
        spawn(async move {
            let js_code = format!(
                r#"
                function sidebarKeyHandler(event) {{
                    if (event.key === '{SIDEBAR_KEYBOARD_SHORTCUT}' && (event.metaKey || event.ctrlKey)) {{
                        event.preventDefault();
                        dioxus.send(true);
                    }}
                }}
                window.__sidebarKeyHandler = sidebarKeyHandler;
                window.addEventListener('keydown', window.__sidebarKeyHandler);
                "#
            );
            let mut eval = document::eval(&js_code);

            loop {
                if eval.recv::<bool>().await.is_ok() {
                    ctx.toggle();
                }
            }
        });
    });

    use_drop(|| {
        _ = document::eval(
            r#"
            window.removeEventListener('keydown', window.__sidebarKeyHandler);
            delete window.__sidebarKeyHandler;
            "#,
        );
    });

    let sidebar_style = format!(
        r#"--dx-sidebar-width: {SIDEBAR_WIDTH}; --dx-sidebar-width-icon: {SIDEBAR_WIDTH_ICON}"#
    );

    let base = attributes!(div {
        class: Styles::dx_sidebar_wrapper,
        class: "bg-inherit",
        "data-slot": "sidebar-wrapper",
        style: sidebar_style,
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn Sidebar(
    #[props(default)] side: SidebarSide,
    #[props(default)] variant: SidebarVariant,
    #[props(default)] collapsible: SidebarCollapsible,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctx = use_sidebar();
    let mut ctx_side = ctx.side;
    if *ctx_side.peek() != side {
        ctx_side.set(side);
    }

    let state = ctx.state;

    let collapsible_str = if state() == SidebarState::Collapsed {
        collapsible.as_str()
    } else {
        ""
    };

    let container_base = attributes!(div {
        class: Styles::dx_sidebar_container,
        "data-slot": "sidebar-container",
    });
    let container_attrs = merge_attributes(vec![container_base, attributes]);

    rsx! {
        div {
            class: Styles::dx_sidebar_desktop,
            "data-state": state().as_str(),
            "data-collapsible": collapsible_str,
            "data-variant": variant.as_str(),
            "data-side": side.as_str(),
            "data-slot": "sidebar",
            div { class: Styles::dx_sidebar_gap, "data-slot": "sidebar-gap" }
            div {..container_attrs,
                div {
                    class: Styles::dx_sidebar_inner,
                    "data-sidebar": "sidebar",
                    "data-slot": "sidebar-inner",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn SidebarTrigger(
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = button)]
    attributes: Vec<Attribute>,
) -> Element {
    let ctx = use_sidebar();

    let base = attributes!(button {
        class: "leading-none",
        "data-sidebar": "trigger",
        "data-slot": "sidebar-trigger",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Icon,
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
                ctx.toggle();
            },
            attributes: merged,
            if ctx.open() {
                ChevronLeft {}
            } else {
                ChevronRight {}
            }
        }
    }
}

#[component]
pub fn SidebarRail(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    let ctx = use_sidebar();

    let base = attributes!(button {
        class: Styles::dx_sidebar_rail,
        "data-sidebar": "rail",
        "data-slot": "sidebar-rail",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        button {
            aria_label: "Toggle Sidebar",
            tabindex: -1,
            onclick: move |_| ctx.toggle(),
            title: "Toggle Sidebar",
            ..merged,
        }
    }
}

#[component]
pub fn SidebarInset(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(main {
        class: Styles::dx_sidebar_inset,
        class: "p-4 items-center",
        "data-slot": "sidebar-inset",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        main { ..merged,{children} }
    }
}

#[component]
pub fn SidebarHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_header,
        "data-slot": "sidebar-header",
        "data-sidebar": "header",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}

#[component]
pub fn SidebarContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_sidebar_content,
        "data-slot": "sidebar-content",
        "data-sidebar": "content",
    });
    let merged = merge_attributes(vec![base, attributes]);

    rsx! {
        div { ..merged,{children} }
    }
}
