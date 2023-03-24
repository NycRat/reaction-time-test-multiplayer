use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::home_page::*;
use crate::settings_page::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/reaction-time-test-multiplayer.css"/>

        <Title text="Reaction Time Test"/>

        <Router>
            <Routes>
                <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                <Route path="settings" view=|cx| view! { cx, <SettingsPage/> }/>
            </Routes>
        </Router>
    }
}
