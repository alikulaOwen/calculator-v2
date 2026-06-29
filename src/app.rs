use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="mx-auto my-10 max-w-3xl rounded-3xl bg-white p-8 shadow-2xl shadow-slate-300/50 ring-1 ring-slate-200">
            <div class="space-y-6">
                <div class="text-center">
                    <h1 class="text-4xl font-semibold text-slate-900">"Welcome to Tauri + Leptos"</h1>
                    <p class="mt-3 text-slate-500">"Click on the Tauri and Leptos logos to learn more."</p>
                </div>

                <div class="flex flex-wrap justify-center gap-6">
                    <a href="https://tauri.app" target="_blank" class="transform transition hover:-translate-y-1">
                        <img src="public/tauri.svg" class="h-24" alt="Tauri logo"/>
                    </a>
                    <a href="https://docs.rs/leptos/" target="_blank" class="transform transition hover:-translate-y-1">
                        <img src="public/leptos.svg" class="h-24" alt="Leptos logo"/>
                    </a>
                </div>

                <form class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-center" on:submit=greet>
                    <input
                        id="greet-input"
                        placeholder="Enter a name..."
                        on:input=update_name
                        class="w-full rounded-2xl border border-slate-200 px-4 py-3 text-slate-900 shadow-sm outline-none transition focus:border-slate-400 focus:ring-2 focus:ring-slate-200 sm:w-auto"
                    />
                    <button type="submit" class="rounded-2xl bg-slate-900 px-6 py-3 text-white transition hover:bg-slate-700">
                        "Greet"
                    </button>
                </form>

                <p class="min-h-[1.5rem] text-center text-slate-600">{ move || greet_msg.get() }</p>
            </div>
        </main>
    }
}
