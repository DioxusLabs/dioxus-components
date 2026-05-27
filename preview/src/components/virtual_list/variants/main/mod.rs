use super::super::component::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "dx-virtual-list-demo",
            p { class: "dx-virtual-list-subtitle", "Scroll this page to verify virtualized rendering with dynamic row heights." }
            style { r#".dx-virtual-list-container {{
  position: relative;
  max-height: 36rem;
  contain: layout paint;
  overflow-y: auto;
}}

.dx-virtual-list-demo {{
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  gap: 0.75rem;
}}

.dx-virtual-list-demo .dx-virtual-list-subtitle {{
  margin: 0;
  margin-bottom: 0.75rem;
  color: var(--primary-color-9);
  font-size: 0.9rem;
}}

.dx-virtual-list-card {{
  padding: 0.75rem 0.9rem;
  border: 1px solid var(--primary-color-6);
  border-radius: 0.625rem;
  background: var(--primary-color-2);
}}

.dx-virtual-list-card h3 {{
  margin: 0 0 0.3rem;
  color: var(--primary-color-12);
  font-size: 0.95rem;
}}

.dx-virtual-list-card p {{
  margin: 0;
  color: var(--primary-color-11);
  font-size: 0.875rem;
  line-height: 1.4;
}}"# }
            VirtualList {
                count: 2000usize,
                buffer: 12usize,
                // Estimate height based on content pattern (idx % 6 repeats)
                // Measured: min=68.4px (0 repeats), max=127.2px (5 repeats)
                estimate_size: |idx| {
                    let base_height = 68;
                    let repeats: usize = idx % 6;
                    let lines = repeats.div_ceil(2); // 2 repeats per line
                    let per_line = 20;
                    base_height + lines as u32 * per_line
                },
                render_item: move |idx: usize| {
                    let extra_text = "Extra content to vary row height. ".repeat(idx % 6);
                    rsx! {
                        article { class: "dx-virtual-list-card",
                            h3 { "#{idx + 1} - Item {idx + 1}" }
                            p { "Virtualized row preview. Index = {idx}" }
                            p { "{extra_text}" }
                        }
                    }
                },
            }
        }
    }
}
