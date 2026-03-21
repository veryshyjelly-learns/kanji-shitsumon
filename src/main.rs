mod kanji;

use dioxus::prelude::*;
use rand::prelude::SliceRandom;
use rand::rng;
use crate::kanji::Kanji;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const DATA: Asset = asset!("/assets/data.csv");

#[derive(Clone)]
struct KanjiDataState(Vec<Kanji>);

#[derive(Clone)]
struct CurrentIdx(usize);

#[derive(Clone)]
struct KanjiAmount(usize);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let kanjis = kanji::read_file(DATA).unwrap();
    use_context_provider(|| Signal::new(KanjiDataState(kanjis)));
    use_context_provider(|| Signal::new(CurrentIdx(0)));
    use_context_provider(|| Signal::new(KanjiAmount(0)));  // <-- Signal now

    let amount: Signal<KanjiAmount> = use_context();
    let KanjiAmount(n) = *amount.read();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        if n == 0 {
            Landing {}
        } else {
            Hero {}
        }
    }
}

#[component]
pub fn Landing() -> Element {
    let mut input_val = use_signal(|| "".to_string());
    let mut amount: Signal<KanjiAmount> = use_context();
    let mut kanjis: Signal<KanjiDataState> = use_context();

    rsx! {
        h2 {
            "How many kanjis wanna quiz?",
        }
        input { oninput: move |e| input_val.set(e.value()) }
        br {}
        button {
            style: "margin-top: 10px;",
            onclick: move |_| {
                if let Ok(n) = input_val.to_string().parse::<usize>() {
                    amount.set(KanjiAmount(n));
                    let mut kanjis_new = (&kanjis.read().clone().0[..n]).to_vec();
                    kanjis_new.shuffle(&mut rng());
                    kanjis.set(KanjiDataState(kanjis_new));
                }
            },
            "Start"
        }
    }
}

#[component]
pub fn Hero() -> Element {
    let data: Signal<KanjiDataState> = use_context();
    let mut idx: Signal<CurrentIdx> = use_context();
    let KanjiDataState(kanjis) = data.read().clone();
    let CurrentIdx(i) = *idx.read();
    let current_kanji = kanjis.get(i).unwrap().clone();
    let mut input_value = use_signal(|| "".to_string());

    rsx! {
        div {
            id: "question",
            div { id: "kanji", { current_kanji.kanji.clone() } },
            input {
                id: "keyword",
                value: "{input_value}",
                oninput: move |e| {
                    let input = e.value().to_lowercase();
                    let answer = current_kanji.keyword.trim().to_lowercase();
                    if input == answer {
                        if i == kanjis.len() - 1 {
                           idx.set(CurrentIdx(0));
                        } else {
                            idx.set(CurrentIdx(i + 1));
                        }
                        input_value.set("".to_string());
                    } else {
                        input_value.set(e.value());
                    }
                }
            }
        }
    }
}
