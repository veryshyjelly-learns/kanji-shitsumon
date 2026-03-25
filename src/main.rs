mod kanji;

use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;
use rand::prelude::SliceRandom;
use rand::rng;

const FAVICON:   Asset = asset!("/assets/favicon.ico");
const MAIN_CSS:  Asset = asset!("/assets/main.css");
const DATA:      Asset = asset!("/assets/data.csv");

#[derive(Clone)] struct KanjiData(Vec<kanji::Kanji>);
#[derive(Clone)] struct CurrentIdx(usize);
#[derive(Clone)] struct KanjiAmount(usize);

// ── Bootstrap ───────────────────────────────────────────────────────────────

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(WindowBuilder::new()
                    .with_title("Kanji")
                    .with_always_on_top(false)),
        )
        .launch(App);
}

// ── Root ─────────────────────────────────────────────────────────────────────

#[component]
fn App() -> Element {
    let kanjis = kanji::read_file(DATA).unwrap();
    use_context_provider(|| Signal::new(KanjiData(kanjis)));
    use_context_provider(|| Signal::new(CurrentIdx(0)));
    use_context_provider(|| Signal::new(KanjiAmount(0)));

    let amount: Signal<KanjiAmount> = use_context();
    let KanjiAmount(n) = *amount.read();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        if n == 0 { Landing {} } else { Hero {} }
    }
}

// ── Landing ──────────────────────────────────────────────────────────────────

#[component]
pub fn Landing() -> Element {
    let mut input_val          = use_signal(|| String::new());
    let mut amount: Signal<KanjiAmount> = use_context();
    let mut kanjis: Signal<KanjiData>   = use_context();

    rsx! {
        h2 { "How many kanjis wanna quiz?" }
        input { oninput: move |e| input_val.set(e.value()) }
        br {}
        button {
            onclick: move |_| {
                let Ok(n) = input_val.to_string().parse::<usize>() else { return };
                let mut deck = kanjis.read().0[..n].to_vec();
                deck.shuffle(&mut rng());
                kanjis.set(KanjiData(deck));
                amount.set(KanjiAmount(n));
            },
            "Start"
        }
    }
}

// ── Hero ─────────────────────────────────────────────────────────────────────

#[component]
pub fn Hero() -> Element {
    let mut data: Signal<KanjiData>   = use_context();
    let mut idx:  Signal<CurrentIdx>  = use_context();
    let mut input_value               = use_signal(|| String::new());

    let KanjiData(kanjis)  = data.read().clone();
    let CurrentIdx(i)      = *idx.read();
    let current            = kanjis[i].clone();

    rsx! {
        div { id: "question",
            div { id: "kanji", { current.kanji.clone() } }
            input {
                id: "keyword",
                value: "{input_value}",
                oninput: move |e| {
                    let input  = e.value().trim().to_lowercase();
                    let answer = current.keyword.trim().to_lowercase();

                    if input != answer {
                        input_value.set(e.value());
                        return;
                    }

                    let next = i + 1;
                    if next >= kanjis.len() {
                        let mut reshuffled = kanjis.clone();
                        reshuffled.shuffle(&mut rng());
                        data.set(KanjiData(reshuffled));
                        idx.set(CurrentIdx(0));
                    } else {
                        idx.set(CurrentIdx(next));
                    }
                    input_value.set(String::new());
                }
            }
        }
        div { id: "hint",
            "?"
            span { id: "hint-tooltip", { current.keyword.trim().to_string() } }
        }
    }
}
