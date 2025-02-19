use maud::{html, Markup, PreEscaped};

static SEND_SVG: &str = include_str!("../../../static/icons/send.svg");

pub fn send_icon() -> Markup {
    html! {(PreEscaped(SEND_SVG))}
}
