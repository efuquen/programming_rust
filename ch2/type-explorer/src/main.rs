use leptos::{ev::SubmitEvent, *};
use leptos::html::{Textarea, Select};

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
}

fn handle_char(text: &String) -> Result<String, TypeError> {
    if text.chars().count() != 1 {
        return Err(TypeError { message: format!("Text must be exactly 1 character, not {} characters.", text.len()).to_string() })
    }
    let ch: u32 = text.chars().next().unwrap() as u32;
    Ok(format!("{:X?}", ch.to_be_bytes()).to_string())
}

fn handle_string(text: &String) -> Result<String, TypeError> {
    Ok(format!("{:X?}", text.as_bytes()).to_string())
}

fn handle_type(text: &String, text_type: &String) -> Result<String, TypeError> {
    match text_type.as_str() {
        "char" => handle_char(text),
        "string" => handle_string(text),
        _ => Err(TypeError { message: format!("Type `{}` not handled.", text_type).to_string() }),
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (rust_bytes, set_rust_bytes) = create_signal(cx, "".to_string());
    let (err_text, set_err_text) = create_signal(cx, "".to_string());

    let textarea_element: NodeRef<Textarea> = create_node_ref(cx);
    let type_select_element: NodeRef<Select> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
    
        let text_value = textarea_element.get()
            .expect("<textarea> to exist")
            .value();

        let type_value = type_select_element.get()
            .expect("<select> to exist")
            .value();

        match handle_type(&text_value, &type_value) {
            Ok(bytes_value) => {
                set_rust_bytes.set(bytes_value);
                set_err_text.set("".to_string())
            },
            Err(e) => {
                set_err_text.set(e.message);
                set_rust_bytes.set("".to_string());
            },
        };
    };

    view! { cx,
        <form on:submit=on_submit>
            <textarea
                placeholder="Enter value ..."
                node_ref=textarea_element
            />
            <div>
                <label>Type:</label>
                <select
                    node_ref=type_select_element>
                    <option value="char">char</option>
                    <option value="string">string</option>
                    <option value="i32">i32</option>
                </select>
            </div>
            <input type="submit" value="Submit"/>
        </form>
        <div class="red">{move || err_text.get()}</div>
        <p>"Bytes: " {move || rust_bytes.get()}</p>
    }
}
