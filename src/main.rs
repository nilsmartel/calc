use dioxus::prelude::*;
fn main() {
    dioxus::desktop::launch(App);
}

const TEXT_COLOR: &'static str = "#e9e9e9";
const BACKGROUND_COLOR: &'static str = "#232323";

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",

            // headerline
            div {background_color: "#232323",color: "#e9e9e9", "Hello, world!" }

            // controls
            div {
                display: "flex",
                flex_direction: "row",
                Button { text: "AC".to_string()}
                Button { text: "+-".to_string()}
                Button { text: "%".to_string()}
                Button { text: "/".to_string()}
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { text: "7".to_string()}
                Button { text: "8".to_string()}
                Button { text: "9".to_string()}
                Button { text: "x".to_string()}
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { text: "4".to_string()}
                Button { text: "5".to_string()}
                Button { text: "6".to_string()}
                Button { text: "-".to_string()}
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { text: "1".to_string()}
                Button { text: "2".to_string()}
                Button { text: "3".to_string()}
                Button { text: "+".to_string()}
            }
            div {
                display: "flex",
                flex_direction: "row",
                DoubleButton { text: "0".to_string()}
                Button { text: ",".to_string()}
                Button { text: "=".to_string()}
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct ButtonProps {
    text: String,
}

fn Button(cx: Scope<ButtonProps>) -> Element {
    cx.render(rsx! {
        div {
            height: "64px",
            width: "96px",
            "{cx.props.text}",
        }
    })
}

fn DoubleButton(cx: Scope<ButtonProps>) -> Element {
    cx.render(rsx! {
        div {
            height: "64px",
            width: "192px",
            "{cx.props.text}",
        }
    })
}
