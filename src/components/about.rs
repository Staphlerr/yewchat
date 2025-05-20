use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="p-8 max-w-2xl mx-auto space-y-4">
            <h2 class="text-2xl font-bold">{"Why Creativity Matters"}</h2>
            <img
                src="https://download.logo.wine/logo/World_Economic_Forum/World_Economic_Forum-Logo.wine.png"
                alt="WEF Logo"
                class="w-20 mx-auto"
            />
            <p>
                {"Creativity is one of the most critical traits for success in your future career. "}
                <a
                    href="https://www.weforum.org/agenda/2020/11/ai-automation-creativity-workforce-skill-fute-of-work/"
                    target="_blank"
                    class="text-blue-600 hover:underline"
                >
                  {"Read the WEF article →"}
                </a>
            </p>
        </div>
    }
}
