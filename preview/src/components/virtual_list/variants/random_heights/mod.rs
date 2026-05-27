use super::super::component::*;
use dioxus::prelude::*;

const INLINE_STYLE: &str = r#".dx-virtual-list-container {
  position: relative;
  max-height: 36rem;
  contain: layout paint;
  overflow-y: auto;
}

.dx-virtual-list-demo {
  display: flex;
  flex-direction: column;
  margin: 0 auto;
  gap: 0.75rem;
}

.dx-virtual-list-card {
  padding: 0.75rem 0.9rem;
  border: 1px solid var(--primary-color-6);
  border-radius: 0.625rem;
  background: var(--primary-color-2);
}

.dx-virtual-list-card h3 {
  margin: 0 0 0.3rem;
  color: var(--primary-color-12);
  font-size: 0.95rem;
}

.dx-virtual-list-card p {
  margin: 0;
  color: var(--primary-color-11);
  font-size: 0.875rem;
  line-height: 1.4;
}"#;

/// Simple hash to generate pseudo-random repeats per item (0-20 range)
fn pseudo_random_repeats(idx: usize) -> usize {
    // Mix bits for pseudo-randomness
    let mut x = idx.wrapping_mul(2654435761);
    x ^= x >> 16;
    x ^= x >> 8;
    x % 21 // 0-20 repeats
}

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { class: "dx-virtual-list-demo",
            p { style: "color: var(--primary-color-9); font-size: 0.9rem; margin-bottom: 0.75rem;",
                "Random heights variant - tests adaptive estimation with highly variable item sizes"
            }
            style { {INLINE_STYLE} }
            VirtualList {
                count: 2000usize,
                buffer: 12usize,
                // NO estimate_size - uses adaptive estimation which will struggle
                // with random heights until many items are measured
                render_item: move |idx: usize| {
                    let repeats = pseudo_random_repeats(idx);
                    let extra_text = "Variable content. ".repeat(repeats);
                    rsx! {
                        article { class: "dx-virtual-list-card",
                            h3 { "#{idx + 1} - Item {idx + 1} ({repeats} repeats)" }
                            p { "Index = {idx}" }
                            p { "{extra_text}" }
                        }
                    }
                },
            }
        }
    }
}
