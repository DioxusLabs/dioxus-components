use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use dioxus::core::queue_effect;
use dioxus::document::{
    Document as DioxusDocument, Eval, LinkProps, MetaProps, NoOpDocument as DioxusNoOpDocument,
    ScriptProps, StyleProps,
};
use dioxus::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(inline_js = r#"
export function dx_set_title(title) {
    if (globalThis.document) {
        globalThis.document.title = title;
    }
}

export function dx_upsert_head_element(tag, id, key, attributesJson, textContent, hasTextContent) {
    const document = globalThis.document;
    if (!document || !document.head) {
        return;
    }

    let element = document.getElementById(id);
    if (!element) {
        element = document.createElement(tag);
    }

    for (const [name, value] of JSON.parse(attributesJson)) {
        element.setAttribute(name, value);
    }

    element.textContent = hasTextContent ? textContent : "";
    element.setAttribute("id", id);
    element.setAttribute("data-dx-head-key", key);

    if (!element.parentNode) {
        document.head.appendChild(element);
    }
}
"#)]
extern "C" {
    fn dx_set_title(title: &str);

    fn dx_upsert_head_element(
        tag: &str,
        id: &str,
        key: &str,
        attributes_json: &str,
        text_content: &str,
        has_text_content: bool,
    );
}

/// Provide a document implementation that mutates the browser head through wasm-bindgen-x.
pub fn use_wasm_bindgen_document() {
    use_context_provider(|| Rc::new(WasmBindgenDocument) as Rc<dyn DioxusDocument>);
}

/// Insert or update a `<link>` element in the document head.
#[component]
pub fn HeadLink(rel: String, href: Asset) -> Element {
    let href = href.to_string();

    use_effect(move || {
        let key = format!("link:{rel}:{href}");
        upsert_head_element(
            "link",
            &key,
            [("rel", rel.as_str()), ("href", href.as_str())],
            None,
        );
    });

    rsx! {}
}

/// Insert or update a `<script>` element in the document head.
#[component]
pub fn HeadScript(src: Asset, #[props(default)] defer: bool) -> Element {
    let src = src.to_string();

    use_effect(move || {
        let key = format!("script:{src}");
        if defer {
            upsert_head_element(
                "script",
                &key,
                [("src", src.as_str()), ("defer", "true")],
                None,
            );
        } else {
            upsert_head_element("script", &key, [("src", src.as_str())], None);
        }
    });

    rsx! {}
}

#[derive(Clone)]
struct WasmBindgenDocument;

impl DioxusDocument for WasmBindgenDocument {
    fn eval(&self, js: String) -> Eval {
        DioxusNoOpDocument.eval(js)
    }

    fn set_title(&self, title: String) {
        queue_effect(move || {
            dx_set_title(&title);
        });
    }

    fn create_meta(&self, props: MetaProps) {
        queue_effect(move || insert_head_element("meta", &props.attributes(), None));
    }

    fn create_script(&self, props: ScriptProps) {
        queue_effect(move || {
            let contents = props.script_contents().ok();
            insert_head_element("script", &props.attributes(), contents.as_deref());
        });
    }

    fn create_style(&self, props: StyleProps) {
        queue_effect(move || {
            let contents = props.style_contents().ok();
            insert_head_element("style", &props.attributes(), contents.as_deref());
        });
    }

    fn create_link(&self, props: LinkProps) {
        queue_effect(move || insert_head_element("link", &props.attributes(), None));
    }
}

fn insert_head_element(tag: &str, attributes: &[(&'static str, String)], contents: Option<&str>) {
    let key = head_element_key(tag, attributes, contents);
    upsert_head_element(
        tag,
        &key,
        attributes
            .iter()
            .map(|(name, value)| (*name, value.as_str())),
        contents,
    );
}

fn upsert_head_element<'a>(
    tag: &str,
    key: &str,
    attributes: impl IntoIterator<Item = (&'a str, &'a str)>,
    contents: Option<&str>,
) {
    let id = head_element_id(key);
    let attributes = attributes.into_iter().collect::<Vec<_>>();
    let Ok(attributes_json) = serde_json::to_string(&attributes) else {
        return;
    };
    dx_upsert_head_element(
        tag,
        &id,
        key,
        &attributes_json,
        contents.unwrap_or_default(),
        contents.is_some(),
    );
}

fn head_element_key(
    tag: &str,
    attributes: &[(&'static str, String)],
    contents: Option<&str>,
) -> String {
    let mut hasher = DefaultHasher::new();
    tag.hash(&mut hasher);
    attributes.hash(&mut hasher);
    contents.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn head_element_id(key: &str) -> String {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    format!("dx-head-{:x}", hasher.finish())
}
