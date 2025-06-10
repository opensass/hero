use yew::prelude::*;

use hero::yew::pride::Hero;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
            <Hero />
        }
}
