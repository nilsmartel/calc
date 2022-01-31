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
            background_color: "#232323",
            flex_direction: "column",
            font_family: "Arial",

            // headerline
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "end",
                align_items: "center",
                width: "100%",
                padding: "12px",
                color: "#e9e9e9",
                p {"123"}
                "Hello, world!"
            }

            // controls
            div {
                display: "flex",
                flex_direction: "row",
                Button {
                    margin_left: false,
                    background_color: "#383838",
                    margin_top: false,
                    text: "AC".to_string()
                }
                Button {
                    margin_top: false,
                    background_color: "#383838",
                    text: "+-".to_string()}
                Button {
                    margin_top: false,
                    background_color: "#383838",
                    text: "%".to_string()}
                Button {
                    margin_top: false,
                    background_color: "#f2a23c",
                    text: "/".to_string()}
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { margin_left: false, text: "7".to_string()}
                Button { text: "8".to_string()}
                Button { text: "9".to_string()}
                Button {
                    background_color: "#f2a23c",
                    text: "x".to_string(),
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { margin_left: false, text: "4".to_string()}
                Button { text: "5".to_string()}
                Button { text: "6".to_string()}
                Button {
                    background_color: "#f2a23c",
                    text: "-".to_string(),
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { margin_left: false, text: "1".to_string()}
                Button { text: "2".to_string()}
                Button { text: "3".to_string()}
                Button {
                    background_color: "#f2a23c",
                    text: "+".to_string()
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                Button { margin_left: false, text: "0".to_string(), width: 57*2+1}
                Button { text: ",".to_string()}
                Button {
                    background_color: "#f2a23c",
                    text: "=".to_string()
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct ButtonProps {
    #[props(into)]
    text: String,

    #[props(default = 57)]
    width: u32,

    #[props(default = 47)]
    height: u32,

    #[props(default = "#5a5a5a")]
    background_color: &'static str,

    #[props(default = true)]
    margin_left: bool,

    #[props(default = true)]
    margin_top: bool,
}

fn Button(cx: Scope<ButtonProps>) -> Element {
    let margin_top = if cx.props.margin_top {
        "1px"
    } else {
        "0"
    };
    let margin_left = if cx.props.margin_left {
        "1px"
    } else {
        "0"
    };

    cx.render(rsx! {
        div {
            display: "flex",
            justify_content: "center",
            align_items: "center",
            margin_left: "{margin_left}",
            margin_top: "{margin_top}",
            background_color: "{cx.props.background_color}",
            color: "#e9e9e9",
            height: "{cx.props.height}px",
            width: "{cx.props.width}px",
            "{cx.props.text}",
        }
    })
}
