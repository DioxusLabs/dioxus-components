use super::super::component::*;
use dioxus::prelude::*;
use dioxus_primitives::sleep;
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 0);

    use_effect(move || {
        spawn(async move {
            let mut seed = 0x9e37_79b9_u32;
            loop {
                sleep(Duration::from_secs(1)).await;
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                let random_value = (seed % 30) as usize;
                let mut progress = progress.write();
                *progress = (*progress + random_value) % 101;
            }
        });
    });

    rsx! {
        Progress { aria_label: "Progressbar Demo", value: progress() as f64 }
    }
}
