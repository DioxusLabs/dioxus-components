use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct Payment {
    id: usize,
    status: &'static str,
    email: &'static str,
    amount: &'static str,
}

// Ek jagah define karo — name + extractor function
struct ColumnDef {
    name: &'static str,
    get: fn(&Payment) -> &'static str,
}

static COLUMNS: &[ColumnDef] = &[
    ColumnDef { name: "Status", get: |p| p.status },
    ColumnDef { name: "Email",  get: |p| p.email  },
    ColumnDef { name: "Amount", get: |p| p.amount  },
];

static DATA: &[Payment] = &[
    Payment { id: 1, status: "Success",    email: "ken99@example.com",       amount: "$316.00" },
    Payment { id: 2, status: "Success",    email: "abe45@example.com",       amount: "$242.00" },
    Payment { id: 3, status: "Processing", email: "monserrat44@example.com", amount: "$837.00" },
    Payment { id: 4, status: "Success",    email: "silas22@example.com",     amount: "$874.00" },
    Payment { id: 5, status: "Failed",     email: "carmella@example.com",    amount: "$721.00" },
    Payment { id: 6, status: "Failed",     email: "carmella2@example.com",   amount: "$521.00" },
];

#[component]
pub fn Demo() -> Element {
    let mut filter = use_signal(|| String::new());
    let mut selected = use_signal(|| vec![false; DATA.len()]);
    let mut active_menu = use_signal(|| None::<usize>);
    let mut show_columns_menu = use_signal(|| false);

    // COLUMNS.len() se automatically sahi size milegi
    let mut col_visible = use_signal(|| vec![true; COLUMNS.len()]);

    let f = filter.read().to_lowercase();
    let filtered: Vec<(usize, &'static Payment)> = DATA
        .iter()
        .enumerate()
        .filter(|(_, p)| f.is_empty() || p.email.to_lowercase().contains(f.as_str()))
        .collect();

    let selected_count = selected.read().iter().filter(|&&v| v).count();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("../../style.css") }

        div { class: "dx-table-wrapper",

            // Toolbar
            div { class: "dx-table-toolbar",
                input {
                    class: "dx-table-filter",
                    r#type: "text",
                    placeholder: "Filter emails...",
                    value: "{filter}",
                    oninput: move |e| filter.set(e.value()),
                }

                div { style: "position: relative;",
                    button {
                        class: "dx-columns-btn",
                        onclick: move |_| {
                            let current = *show_columns_menu.read();
                            show_columns_menu.set(!current);
                        },
                        "Columns "
                        span { class: "dx-chevron", "⌄" }
                    }

                    if *show_columns_menu.read() {
                        div { class: "dx-columns-menu",
                            {COLUMNS.iter().enumerate().map(|(i, col)| {
                                let visible = col_visible.read()[i];
                                rsx! {
                                    label { class: "dx-columns-item",
                                        key: "{col.name}",
                                        input {
                                            r#type: "checkbox",
                                            class: "dx-checkbox",
                                            checked: visible,
                                            onchange: move |e| {
                                                col_visible.write()[i] = e.checked();
                                            }
                                        }
                                        span { "{col.name}" }
                                    }
                                }
                            })}
                        }
                    }
                }
            }

            // Table
            div { class: "dx-table-container",
                table { class: "dx-table",
                    thead {
                        tr {
                            th {
                                input {
                                    r#type: "checkbox",
                                    class: "dx-checkbox",
                                    onchange: move |e| {
                                        let checked = e.checked();
                                        selected.write().iter_mut().for_each(|v| *v = checked);
                                    }
                                }
                            }
                            {COLUMNS.iter().enumerate().map(|(i, col)| {
                                let visible = col_visible.read()[i];
                                rsx! {
                                    if visible {
                                        th { key: "{col.name}", "{col.name}" }
                                    }
                                }
                            })}
                            th {}
                        }
                    }
                    tbody {
                        {filtered.iter().map(|(i, p)| {
                            let i = *i;
                            let p: &'static Payment = p;
                            let is_selected = selected.read()[i];
                            let show_menu = active_menu.read().is_some_and(|id| id == p.id);
                            rsx! {
                                tr {
                                    key: "{p.id}",
                                    class: if is_selected { "dx-row dx-row-selected" } else { "dx-row" },
                                    td {
                                        input {
                                            r#type: "checkbox",
                                            class: "dx-checkbox",
                                            checked: is_selected,
                                            onchange: move |e| {
                                                selected.write()[i] = e.checked();
                                            }
                                        }
                                    }
                                    {COLUMNS.iter().enumerate().map(|(ci, col)| {
                                        let visible = col_visible.read()[ci];
                                        let value = (col.get)(p);
                                        rsx! {
                                            if visible {
                                                td { key: "{col.name}", "{value}" }
                                            }
                                        }
                                    })}
                                    td {
                                        div { style: "position: relative;",
                                            button {
                                                class: "dx-action-btn",
                                                onclick: move |_| {
                                                    if active_menu.read().is_some_and(|id| id == p.id) {
                                                        active_menu.set(None);
                                                    } else {
                                                        active_menu.set(Some(p.id));
                                                    }
                                                },
                                                "···"
                                            }
                                            if show_menu {
                                                div {
                                                    class: "dx-action-menu",
                                                    onclick: move |_| { active_menu.set(None); },
                                                    button { "Copy payment ID" }
                                                    button { "View customer" }
                                                    button { "View payment details" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // Footer
            div { class: "dx-table-footer",
                span { class: "dx-footer-info",
                    "{selected_count} of {DATA.len()} row(s) selected."
                }
                div { class: "dx-pagination-buttons",
                    button { class: "dx-pagination-button", "Previous" }
                    button { class: "dx-pagination-button", "Next" }
                }
            }
        }

        if active_menu.read().is_some() {
            div {
                class: "dx-menu-overlay",
                onclick: move |_| active_menu.set(None),
            }
        }
        if *show_columns_menu.read() {
            div {
                class: "dx-menu-overlay",
                onclick: move |_| show_columns_menu.set(false),
            }
        }
    }
}