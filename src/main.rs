use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        input {
            name: "name",
            required: false,
            readonly: "{false}",
            disabled: "false",
            "type": "text"
        }
    )
}
