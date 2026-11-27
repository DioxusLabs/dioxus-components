use super::super::component::*;
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 0);

    use_future(move || async move {
        loop {
            dioxus_timers::sleep(Duration::from_secs(1)).await;
            let mut progress = progress.write();
            *progress = (*progress + fastrand::usize(0..30)) % 101;
        }
    });

    rsx! {
        Progress { aria_label: "Progressbar Demo", value: progress() as f64 }
    }
}
