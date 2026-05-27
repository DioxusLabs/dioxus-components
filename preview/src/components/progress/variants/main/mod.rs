use super::super::component::*;
use dioxus::prelude::*;
use futures::StreamExt;
use gloo_timers::future::IntervalStream;

#[component]
pub fn Demo() -> Element {
    let mut progress = use_signal(|| 0);

    use_effect(move || {
        spawn(async move {
            let mut interval = IntervalStream::new(1000);
            while interval.next().await.is_some() {
                let random_value = (js_sys::Math::random() * 30.0).floor() as usize;
                let mut progress = progress.write();
                *progress = (*progress + random_value) % 101;
            }
        });
    });

    rsx! {
        Progress { aria_label: "Progressbar Demo", value: progress() as f64 }
    }
}
