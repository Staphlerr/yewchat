#![recursion_limit = "512"]

mod components;
pub mod services;
use gloo_utils::document;

use components::about::About;
use components::chat::Chat;
use components::login::Login;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;

use web_sys::HtmlElement;

use std::cell::RefCell;
use std::rc::Rc;

pub type User = Rc<UserInner>;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login    => html! { <Login /> },
        Route::Chat     => html! { <Chat /> },
        Route::About    => html! { <About /> },
        Route::NotFound => html! { <h1>{ "404 – page not found" }</h1> },
    }
}

#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
        })
    });
    // State for dark/light mode
    let is_dark = use_state(|| false);

    // Sync the <html> class on changes
    {
        let is_dark = is_dark.clone();
        use_effect_with_deps(
            move |dark| {
                if let Some(html_el) = document().document_element() {
                    // Fallback to manipulating className directly
                    let mut classes = html_el.class_name();
                    if **dark {
                        if !classes.contains("dark") {
                            classes.push_str(" dark");
                            html_el.set_class_name(&classes);
                        }
                    } else {
                        // remove "dark" from class list
                        let filtered = classes
                            .split_whitespace()
                            .filter(|&c| c != "dark")
                            .collect::<Vec<_>>()
                            .join(" ");
                        html_el.set_class_name(&filtered);
                    }
                }
                || ()
            },
            is_dark.clone(),
        );
    }

    // Toggle callback
    let onclick = {
        let is_dark = is_dark.clone();
        Callback::from(move |_| is_dark.set(!*is_dark))
    };

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
            <BrowserRouter>
                <div class="flex flex-col w-screen h-screen">
                    // ── Header with Nav Links ───────────────────────
                    <header class="flex items-center justify-between p-4 bg-gray-800 text-white">
                        <h1 class="text-xl font-bold">{"💬 YewChat"}</h1>
                        <nav class="space-x-4">
                            <Link<Route> to={Route::Chat} classes="hover:underline">{"Chat"}</Link<Route>>
                            <Link<Route> to={Route::About} classes="hover:underline">{"💡 About"}</Link<Route>>
                            <button
                                {onclick}
                                class="px-4 py-2 rounded bg-gray-200 dark:bg-gray-800">
                                { if *is_dark { "☀️ Light" } else { "🌙 Dark" } }
                            </button>
                        </nav>
                    </header>

                    // ── Main Content ───────────────────────────────
                    <div class="flex-1 overflow-auto">
                        <Switch<Route> render={Switch::render(switch)} />
                    </div>
                </div>
            </BrowserRouter>
        </ContextProvider<User>>
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}
