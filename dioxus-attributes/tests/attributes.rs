use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use dioxus_attributes::attributes;

fn attribute_list() -> Element {
    let additional = vec![Attribute {
        name: "title",
        value: AttributeValue::Text("Additional".to_string()),
        namespace: None,
        volatile: false,
    }];
    let attributes: Vec<Attribute> = attributes!(button {
        id: "trigger",
        "data-state": "open",
        onclick: move |_| {},
        ..additional,
    });

    rsx! { button { ..attributes } }
}

#[test]
fn builds_owned_attributes_and_spreads() {
    let mut dom = VirtualDom::new(attribute_list);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains("id=\"trigger\""));
    assert!(html.contains("data-state=\"open\""));
    assert!(html.contains("title=\"Additional\""));
}
