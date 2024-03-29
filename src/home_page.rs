use cfg_if::cfg_if;
use leptos::*;
use rand::Rng;

#[derive(Clone)]
pub struct TestResult {
    id: usize,
    time: i64,
}

pub fn get_now() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn get_next_start_time() -> i64 {
    get_now() + rand::thread_rng().gen_range(1500, 5000)
}

pub fn get_stored_results() -> Vec<TestResult> {
    // get from database, do this later
    vec![]
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (time, set_time) = create_signal(cx, 0i64);
    let (running, set_running) = create_signal(cx, false);
    let (start_time, set_start_time) = create_signal(cx, get_now());
    let (early_click, set_early_click) = create_signal(cx, false);
    let (next_start_time, set_next_start_time) = create_signal(cx, get_next_start_time());
    let (results, set_results) = create_signal(cx, get_stored_results());

    cfg_if! {
        if #[cfg(feature = "hydrate")] {
            let handle = set_interval_with_handle(
                move || {
                    if running() {
                        if !early_click() {
                            let elapsed = get_now() - start_time();
                            set_time(elapsed);
                        }
                    } else {
                        let now = get_now();
                        if now >= next_start_time() {
                            set_running(true);
                            set_start_time(now);
                            set_next_start_time(get_next_start_time());
                        }
                    }
                },
                std::time::Duration::from_millis(10),
                ).unwrap();

            on_cleanup(cx, move || {
                handle.clear();
            });

            use wasm_bindgen::prelude::*;

            let ws = web_sys::WebSocket::new("ws://localhost:3000/ws/").unwrap();
            let cloned_ws = ws.clone();

            let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MessageEvent| {
                if let Ok(str) = e.data().dyn_into::<js_sys::JsString>() {
                    log!("Recieved String: \"{}\"", str);
                } else {
                    log!("Received Unknown: {:?}", e.data());
                }
            });

            let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: web_sys::ErrorEvent| {
                log!("error event: {:?}", e);
            });

            let send_thing = move || {
                let _ = cloned_ws.send_with_str("");
            };

            let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                log!("socket opened");
            });

            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));

            onerror_callback.forget();
            onmessage_callback.forget();
            onopen_callback.forget();
        } else {
            let send_thing = || {};
        }
    }

    view! { cx,
        <div class="app">
            <div class="menu">
                <h1>"Reaction Speed Test"</h1>
                <h2> {
                    move || {
                        if !early_click() {
                            std::format!("Time: {}ms", time())
                        } else {
                            "Early".to_owned()
                        }
                    }
                } </h2>
                <a href="/settings">
                    "Settings"
                </a>
                <ResultsList results />
            </div>
            <div class=move || { if running() { "clicky red" } else { "clicky" } }
                on:click=move |_| {
                    send_thing();
                    if running() {
                        let now = get_now();
                        let elapsed = now - start_time();

                        if !early_click() {
                            set_results.update(|results| results.push(TestResult {
                                id: results.len(),
                                time: elapsed
                            }));
                        }
                        set_early_click(false);

                        set_time(0);
                        set_running(false);
                        set_start_time(now);
                        set_next_start_time(get_next_start_time());
                    } else {
                        if !early_click() {
                            set_results.update(|results| results.push(TestResult { id: results.len(), time: -1 }));
                            set_time(-1);
                            set_next_start_time(get_now());
                            set_early_click(true);
                        }
                    }
            }>
            <h2>
            {
                move || {
                    if running() {
                        if early_click() {
                            "Clicked too early, try again."
                        } else {
                            "Click Now!"
                        }
                    } else {
                        "Wait to turn red."
                    }
                }
            }
            </h2>
            </div>
        </div>
    }
}

#[component]
pub fn ResultsList(cx: Scope, results: ReadSignal<Vec<TestResult>>) -> impl IntoView {
    view! {
        cx,
        <div class="list">
            <For
                each=move || results().into_iter().rev()
                key=move |result| result.id
                view=move |cx, result: TestResult| {
                    view! {
                        cx,
                        <div>
                            {
                                if result.time >= 0 {
                                    std::format!("{}) {}ms", result.id+1, result.time)
                                } else {
                                    std::format!("{}) Clicked early", result.id+1)
                                }
                            }
                        </div>
                    }
                }
            />
        </div>
    }
}
