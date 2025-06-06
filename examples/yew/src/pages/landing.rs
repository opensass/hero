use web_sys::HtmlInputElement;
use yew::prelude::*;

use hero::yew::hero1::Hero as HeroOne;
use hero::yew::hero2::Hero as HeroTwo;
use hero::yew::hero3::Hero as HeroThree;
use hero::yew::hero4::Hero as HeroFour;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let title1 = use_state(|| "Rust for Modern Web Development".to_string());
    let description1 = use_state(|| {
        "Ship blazingly fast full stack Rust web applications with Open SASS.".to_string()
    });

    let title2 = use_state(|| "Build Ultra-Fast Web Apps with Open SASS".to_string());
    let description2 =
        use_state(|| "Open SASS brings modern Rust-powered speed to your web stack.".to_string());

    let on_title1_change = {
        let title1 = title1.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                title1.set(input.value());
            }
        })
    };

    let on_description1_change = {
        let description1 = description1.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                description1.set(input.value());
            }
        })
    };

    let on_title2_change = {
        let title2 = title2.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                title2.set(input.value());
            }
        })
    };

    let on_description2_change = {
        let description2 = description2.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                description2.set(input.value());
            }
        })
    };
    let heading_text = use_state(|| "Rust for Modern Web Development".to_string());
    let subheading_text = use_state(|| {
        "Ship blazingly fast full stack Rust web applications with Open SASS.".to_string()
    });

    let drive_text = use_state(|| "Build ".to_string());
    let growth_text = use_state(|| "Next-gen Apps".to_string());
    let through_text = use_state(|| "Effortlessly".to_string());
    let description_text = use_state(|| {
        "Open SASS lets you create, deploy, and scale fast apps using Rust and WebAssembly."
            .to_string()
    });

    let on_heading_text_change = {
        let heading_text = heading_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                heading_text.set(input.value());
            }
        })
    };

    let on_subheading_text_change = {
        let subheading_text = subheading_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                subheading_text.set(input.value());
            }
        })
    };

    let on_drive_text_change = {
        let drive_text = drive_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                drive_text.set(input.value());
            }
        })
    };

    let on_growth_text_change = {
        let growth_text = growth_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                growth_text.set(input.value());
            }
        })
    };

    let on_through_text_change = {
        let through_text = through_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                through_text.set(input.value());
            }
        })
    };

    let on_description_text_change = {
        let description_text = description_text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                description_text.set(input.value());
            }
        })
    };
    html! {
            <section class="m-6 min-h-screen flex flex-col items-center justify-center">
                <h1 class="text-3xl font-bold mb-8 text-black">{ "Hero Yew Examples" }</h1>
                <div class="grid grid-cols-1 gap-8 w-full">
                    <div class="flex flex-col items-center bg-black p-6 rounded-lg shadow-lg w-full">
                        <h2 class="text-xl font-semibold mb-4 text-white">{ "Hero Component 1" }</h2>
                        <label>{"Heading Text: "}<input type="text" value={(*heading_text).clone()} oninput={on_heading_text_change.clone()} class="mb-4 p-2 border rounded w-full text-white" /></label>
                        <label>{"Subheading Text: "}<input type="text" value={(*subheading_text).clone()} oninput={on_subheading_text_change.clone()} class="mb-4 p-2 border rounded w-full text-white" /></label>

                        <pre class="text-xs bg-gray-800 text-white p-4 rounded mb-4 overflow-x-auto">{r#"use yew::prelude::*;
use hero::yew::hero4::Hero;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <Hero
            heading_text={"Rust for Modern Web Development"}
            subheading_strong_text={"Ship blazingly fast full stack Rust web applications with Open SASS."}
        />
    }
}"#}</pre>

                        <HeroFour
                            heading_text={(*heading_text).clone()}
                            subheading_strong_text={(*subheading_text).clone()}
                        />
                    </div>
                    <div class="flex flex-col items-center bg-black p-6 rounded-lg shadow-lg w-full">
                        <h2 class="text-xl font-semibold mb-4 text-white">{ "Hero Component 2" }</h2>
                        <label class="mb-2 text-sm text-white">{ "Title:" }</label>
                        <input type="text" class="mb-4 p-2 border rounded w-full text-white" value={(*title1).clone()} oninput={on_title1_change.clone()} />
                        <label class="mb-2 text-sm text-white">{ "Description:" }</label>
                        <input type="text" class="mb-4 p-2 border rounded w-full text-white" value={(*description1).clone()} oninput={on_description1_change.clone()} />

                        <HeroOne
                            heading={(*title1).clone()}
                            description={(*description1).clone()}
                        />
                        <pre class="w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto">
    { r#"use yew::prelude::*;
use web_sys::HtmlInputElement;
use hero::yew::hero1::Hero;

#[function_component(ExampleOne)]
pub fn example_one() -> Html {
    let title = use_state(|| "Rust for Modern Web Development".to_string());
    let description = use_state(|| "Ship blazingly fast full stack Rust web applications with Open SASS.".to_string());

    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                title.set(input.value());
            }
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                description.set(input.value());
            }
        })
    };

    html! {
        <>
            <input value={(*title).clone()} oninput={on_title_change} />
            <input value={(*description).clone()} oninput={on_description_change} />
            <Hero title={title.clone()} description={description.clone()} />
        </>
    }
}"# }
                        </pre>
                    </div>

                    <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg w-full">
                        <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Hero Component 3" }</h2>
                        <pre class="w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto">
    { r#"use yew::prelude::*;
use web_sys::HtmlInputElement;
use hero::yew::hero2::Hero;

#[function_component(ExampleTwo)]
pub fn example_two() -> Html {
    let heading = use_state(|| "Build Ultra-Fast Web Apps with Open SASS".to_string());
    let description = use_state(|| "Open SASS brings modern Rust-powered speed to your web stack.".to_string());

    let on_heading_change = {
        let heading = heading.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                heading.set(input.value());
            }
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                description.set(input.value());
            }
        })
    };

    html! {
        <>
            <input value={(*heading).clone()} oninput={on_heading_change} />
            <input value={(*description).clone()} oninput={on_description_change} />
            <Hero heading={heading.clone()} description={description.clone()} />
        </>
    }
}"# }
                        </pre>
                        <label class="mb-2 text-sm text-gray-700">{ "Heading:" }</label>
                        <input type="text" class="mb-4 p-2 border rounded w-full text-white" value={(*title2).clone()} oninput={on_title2_change.clone()} />
                        <label class="mb-2 text-sm text-gray-700">{ "Description:" }</label>
                        <input type="text" class="mb-4 p-2 border rounded w-full text-white" value={(*description2).clone()} oninput={on_description2_change.clone()} />

                        <HeroTwo
                            heading={(*title2).clone()}
                            description={(*description2).clone()}
                        />
                    </div>

                    <div class="flex flex-col items-center p-6 rounded-lg shadow-lg w-full">
                        <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Hero Component 4" }</h2>
                        <label>{"Drive Text: "}<input type="text" value={(*drive_text).clone()} oninput={on_drive_text_change.clone()} class="mb-4 p-2 border rounded w-full text-white" /></label>
                        <label>{"Growth Text: "}<input type="text" value={(*growth_text).clone()} oninput={on_growth_text_change.clone()} class="mb-4 p-2 border rounded w-full text-white" /></label>
                        <label>{"Through Text: "}<input type="text" value={(*through_text).clone()} oninput={on_through_text_change.clone()} class="mb-4 p-2 border rounded w-full text-white" /></label>
                        <label>{"Description Text: "}<input type="text" value={(*description_text).clone()} oninput={on_description_text_change.clone()} class="mb-4 p-2 border rounded w-full text-white" /></label>

                        <pre class="text-xs bg-gray-800 text-white p-4 rounded mb-4 overflow-x-auto">{r#"use yew::prelude::*;
use hero::yew::hero3::Hero;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <Hero
            drive_text={"Build "}
            growth_text={"Next-gen Apps"}
            through_text={"Effortlessly"}
            description_text={"Open SASS lets you create, deploy, and scale fast apps using Rust and WebAssembly."}
        />
    }
}"#}</pre>

                        <HeroThree
                            drive_text={(*drive_text).clone()}
                            growth_text={(*growth_text).clone()}
                            through_text={(*through_text).clone()}
                            description_text={(*description_text).clone()}
                        />
                    </div>
                </div>
            </section>
        }
}
