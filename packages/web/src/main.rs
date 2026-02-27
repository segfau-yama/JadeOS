//! This example shows how to implement a simple drag-and-drop kanban board using Dioxus.
//! You can drag items between different categories and edit their contents.
//!
//! This example uses the `.data_transfer()` API to handle drag-and-drop events. When an item is dragged,
//! its ID is stored in the data transfer object. When the item is dropped into a new category, its ID is retrieved
//! from the data transfer object and used to update the item's category.
//!
//! Note that in a real-world application, you'll want more sophisticated drop handling, such as visual
//! feedback during dragging, and better drop-zone detection to allow dropping *between* items.

use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use std::rc::Rc;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        for _ in 0..5 {
            Movable { 
                Card {
                    color: "white",
                    shadow: "shadow-sm",
                    rounded: "rounded-lg",
                    CardBody {
                        size: "h-100 w-200",
                        Typography {
                            text: "card.title",
                            size: "text-xl",
                            color: "text-slate-800",
                            position: "text-left",
                            class: "my-2 font-semibold",
                        }
                        Typography {
                            text: "card.text",
                            size: "text-base",
                            color: "text-slate-600",
                            position: "text-left",
                            class: "leading-normal",
                        }
                    }
                }
            }
        }
    }
}
use dioxus::html::input_data::MouseButton;

#[derive(Props, PartialEq, Clone)]
struct MovableProps {
    children: Element,
}


#[component]
pub fn Movable(props: MovableProps) -> Element {
    let mut position = use_signal(|| (100.0, 100.0));
    let mut dragging = use_signal(|| false);
    let mut mounted = use_signal(|| Option::<Rc<MountedData>>::None);
    let mut active_pointer_id = use_signal(|| Option::<i32>::None);

    let mut click_origin = use_signal(|| (0.0, 0.0));
    let mut modal_origin = use_signal(|| (0.0, 0.0));

    let onmounted = move |evt: Event<MountedData>| {
        mounted.set(Some(evt.data()));
    };

    let onpointerdown = move |evt: Event<PointerData>| {
        if evt.data.trigger_button() != Some(MouseButton::Primary) {
            return;
        }

        let pointer_id = evt.data.pointer_id();

        if let Some(element) = mounted
            .read()
            .as_ref()
            .and_then(|m| m.as_ref().try_as_web_event())
        {
            let _ = element.set_pointer_capture(pointer_id);
        }

        let coords = evt.data.coordinates();
        let mouse = (
            coords.client().x as f64,
            coords.client().y as f64
        );

        click_origin.set(mouse);
        modal_origin.set(position());
        active_pointer_id.set(Some(pointer_id));
        dragging.set(true);
    };

    let onpointermove = move |evt: Event<PointerData>| {
        if !dragging() { return; }
        if active_pointer_id() != Some(evt.data.pointer_id()) { return; }

        let coords = evt.data.coordinates();
        let mouse = (
            coords.client().x as f64,
            coords.client().y as f64
        );

        let origin = click_origin();
        let modal = modal_origin();

        let delta = (
            mouse.0 - origin.0,
            mouse.1 - origin.1
        );

        position.set((
            modal.0 + delta.0,
            modal.1 + delta.1
        ));
    };

    let onpointerup = move |evt: Event<PointerData>| {
        if active_pointer_id() != Some(evt.data.pointer_id()) { return; }

        if let Some(element) = mounted
            .read()
            .as_ref()
            .and_then(|m| m.as_ref().try_as_web_event())
        {
            let _ = element.release_pointer_capture(evt.data.pointer_id());
        }

        active_pointer_id.set(None);
        dragging.set(false);
    };

    let onpointercancel = move |evt: Event<PointerData>| {
        if active_pointer_id() != Some(evt.data.pointer_id()) { return; }

        if let Some(element) = mounted
            .read()
            .as_ref()
            .and_then(|m| m.as_ref().try_as_web_event())
        {
            let _ = element.release_pointer_capture(evt.data.pointer_id());
        }

        active_pointer_id.set(None);
        dragging.set(false);
    };

    let onlostpointercapture = move |_| {
        active_pointer_id.set(None);
        dragging.set(false);
    };
    rsx! {
        div {
            style: format!(
                "position:absolute; left:{}px; top:{}px;",
                position().0,
                position().1,
            ),
            onmounted: onmounted,
            onpointerdown: onpointerdown,
            onpointermove: onpointermove,
            onpointerup: onpointerup,
            onpointercancel: onpointercancel,
            onlostpointercapture: onlostpointercapture,
            {props.children}
        }
    }
}
#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    color: String,
    shadow: String,
    rounded: String,
    #[props(default = "".to_string())]
    size: String,
    #[props(default = "".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div { class: "relative flex flex-col border border-slate-200 {props.color} {props.shadow} {props.rounded} {props.size} {props.class:?}",
            {props.children}
        }
    }
}


#[derive(PartialEq, Clone, Props)]
pub struct CardHeaderProps {
    color: String,
    size: String,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div { class: "relative overflow-hidden flex-auto {props.color} {props.size} {props.class:?}",
            {props.children}
        }
    }
}


#[derive(PartialEq, Clone, Props)]
pub struct CardBodyProps {
    #[props(default = "".to_string())]
    size: String,
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! {
        div { class: "p-4 {props.size}", {props.children} }

    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardFooterProps {
    #[props(default = "".to_string())]
    size: String,
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div { class: "px-4 pb-4 pt-0 mt-2 {props.size}", {props.children} }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TypographyProps {
    text: String,
    size: String,
    color: String,
    #[props(default="".to_string())]
    position: String,
    #[props(default="".to_string())]
    class : String,
}

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    rsx! {
        div { class: "{props.size} {props.color} {props.position} {props.class}", {props.text} }
    }
}