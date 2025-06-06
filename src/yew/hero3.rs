use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BackgroundShapesProps {
    #[prop_or("position: absolute; top: 0; left: 0; width: 100%; height: 100%;")]
    pub container_style: &'static str,

    #[prop_or(
        "
        position: absolute;
        width: 80vw;
        max-width: 900px;
        height: auto;
        aspect-ratio: 2/3;
        top: -30vh;
        left: -20vw;
        transform: rotate(-31.91deg);
        filter: blur(12px);
        z-index: 23;
        border-radius: 70px;
        background: linear-gradient(135deg, rgba(105, 71, 255, 0.1), rgba(139, 77, 255, 0.1));
    "
    )]
    pub rectangle_style: &'static str,

    #[prop_or(
        "position: relative; width: 390.262px; height: 412.927px; margin: 711.381px 0 0 224.508px; filter: blur(25px); z-index: 58;"
    )]
    pub star_style: &'static str,

    #[prop_or(
        "
        position: absolute;
        width: 80vw;
        max-width: 900px;
        height: auto;
        aspect-ratio: 2/3;
        top: -30vh;
        left: -20vw;
        transform: rotate(-31.91deg);
        filter: blur(12px);
        z-index: 23;
        border-radius: 70px;
        background: linear-gradient(135deg, rgba(105, 71, 255, 0.1), rgba(139, 77, 255, 0.1));
    "
    )]
    pub rectangle1_style: &'static str,

    #[prop_or(
        "position: absolute; width: 280.238px; height: 328.237px; top: -150.746px; left: 50%; transform: translate(82.32%, 0); filter: blur(25px);"
    )]
    pub star2_style: &'static str,

    #[prop_or(
        "position: absolute; width: 372.744px; height: 259.888px; top: 650.956px; left: 1227.403px; filter: blur(25px); z-index: 57;"
    )]
    pub star1f_style: &'static str,
}

fn render_svg(color: &'static str) -> Html {
    html! {
        <svg viewBox="0 0 100 100" preserveAspectRatio="none" style="width: 100%; height: 100%;">
            <circle cx="50" cy="50" r="50" fill={color} />
        </svg>
    }
}

#[function_component(BackgroundShapes)]
pub fn background_shapes(props: &BackgroundShapesProps) -> Html {
    html! {
        <div style={props.container_style}>
            <div style={props.rectangle_style}>
                <div style={props.star_style}>{ render_svg("rgba(255, 255, 255, 0.12)") }</div>
            </div>
            <div style={props.rectangle1_style} />
            <div style={props.star2_style}>{ render_svg("rgba(139, 77, 255, 0.1)") }</div>
            <div style={props.star1f_style}>
                <i
                    class="fa-solid fa-star"
                    style="font-size: 240px; color: rgba(105, 71, 255, 0.12); opacity: 0.7;"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BackgroundLinesProps {
    #[prop_or("position: absolute; top: 0; left: 0; width: 100%; height: 100%;")]
    pub container_style: &'static str,
    #[prop_or(
        "display: flex; align-items: center; flex-wrap: nowrap; gap: 60px; position: absolute; width: 120px; height: 953.5px; top: 33px; left: 335px; z-index: 1;"
    )]
    pub left_lines_style: &'static str,
    #[prop_or("flex-shrink: 0; position: relative; width: 0.4px; height: 927px; z-index: 2;")]
    pub vector10_style: &'static str,
    #[prop_or("flex-shrink: 0; position: relative; width: 0.4px; height: 927px; z-index: 3;")]
    pub vector11_style: &'static str,
    #[prop_or("flex-shrink: 0; position: relative; width: 0.4px; height: 927px; z-index: 4;")]
    pub vector12_style: &'static str,
    #[prop_or(
        "display: flex; align-items: center; flex-wrap: nowrap; gap: 60px; position: absolute; width: 120px; height: 953.5px; top: 33px; left: 985px; z-index: 5;"
    )]
    pub right_lines_style: &'static str,
    #[prop_or("flex-shrink: 0; position: relative; width: 0.4px; height: 927px; z-index: 6;")]
    pub vector14_style: &'static str,
    #[prop_or("flex-shrink: 0; position: relative; width: 0.4px; height: 927px; z-index: 7;")]
    pub vector15_style: &'static str,
    #[prop_or("flex-shrink: 0; position: relative; width: 0.4px; height: 927px; z-index: 8;")]
    pub vector16_style: &'static str,
}

#[function_component(BackgroundLines)]
pub fn background_lines(props: &BackgroundLinesProps) -> Html {
    html! {
        <div style={props.container_style}>
            <div style={props.left_lines_style}>
                <div style={props.vector10_style} />
                <div style={props.vector11_style} />
                <div style={props.vector12_style} />
            </div>
            <div style={props.right_lines_style}>
                <div style={props.vector14_style} />
                <div style={props.vector15_style} />
                <div style={props.vector16_style} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    #[prop_or(
        "position: absolute; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 0;"
    )]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
    html! {
        <div style={props.style} class={props.class} role="presentation" aria-hidden="true">
            <BackgroundShapes />
            <BackgroundLines />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HeroBadgeProps {
    #[prop_or(
        "display: flex; align-items: center; justify-content: center; flex-wrap: nowrap; flex-shrink: 0; gap: 4px; position: relative; width: 224px; height: 34px; padding: 8px 12px; background: rgba(255, 255, 255, 0.65); border: 0.6px solid #ffffff; z-index: 18; border-radius: 100px; backdrop-filter: blur(30px);"
    )]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "flex-shrink: 0; flex-basis: auto; position: relative; height: 19px; color: #43454a; font-family: Inter, sans-serif; font-size: 16px; font-weight: 420; line-height: 19px; text-align: left; white-space: nowrap; z-index: 19;"
    )]
    pub text_style: &'static str,

    #[prop_or_default]
    pub text_class: &'static str,

    #[prop_or("Launching Soon".to_string())]
    pub text: String,
}

#[function_component(HeroBadge)]
pub fn hero_badge(props: &HeroBadgeProps) -> Html {
    html! {
        <div style={props.style} class={props.class} role="banner">
            <span style={props.text_style} class={props.text_class}>{ props.text.clone() }</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HeroTitleProps {
    #[prop_or(
        "
        align-self: stretch;
        flex-shrink: 0;
        position: relative;
        font-family: Helvetica Neue, sans-serif;
        font-size: clamp(24px, 8vw, 76px);
        font-weight: 700;
        line-height: 1.2;
        text-align: center;
        letter-spacing: -0.5px;
        z-index: 21;
        margin: 0;
    "
    )]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "position: relative; color: #141415; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px;"
    )]
    pub drive_style: &'static str,

    #[prop_or_default]
    pub drive_class: &'static str,

    #[prop_or(
        "position: relative; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px; background: linear-gradient(90deg, #6947ff, #6947ff); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
    )]
    pub growth_style: &'static str,

    #[prop_or_default]
    pub growth_class: &'static str,

    #[prop_or(
        "position: relative; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px; background: linear-gradient(90deg, #6e4dff, #8b4dff); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
    )]
    pub empty_style: &'static str,

    #[prop_or_default]
    pub empty_class: &'static str,

    #[prop_or(
        "position: relative; color: #141415; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px;"
    )]
    pub through_style: &'static str,

    #[prop_or_default]
    pub through_class: &'static str,

    #[prop_or("Build ".to_string())]
    pub drive_text: String,

    #[prop_or("Apps".to_string())]
    pub growth_text: String,

    #[prop_or(" ".to_string())]
    pub empty_text: String,

    #[prop_or("Blazingly Fast".to_string())]
    pub through_text: String,
}

#[function_component(HeroTitle)]
pub fn hero_title(props: &HeroTitleProps) -> Html {
    html! {
        <h1 style={props.style} class={props.class} id="hero-title">
            <span style={props.drive_style} class={props.drive_class}>{ props.drive_text.clone() }</span>
            <span style={props.growth_style} class={props.growth_class}>{ props.growth_text.clone() }</span>
            <span style={props.empty_style} class={props.empty_class}>{ props.empty_text.clone() }</span>
            <span style={props.through_style} class={props.through_class}>
                { props.through_text.clone() }
            </span>
        </h1>
    }
}

#[derive(Properties, PartialEq)]
pub struct HeroDescriptionProps {
    #[prop_or(
        "
        display: flex;
        align-items: flex-start;
        justify-content: center;
        flex-shrink: 0;
        position: relative;
        max-width: 90vw;
        height: auto;
        font-size: clamp(14px, 4vw, 20px);
        line-height: 1.5;
        padding: 0 16px;
        text-align: center;
        margin: 0 auto;
        color: #43454a;
        z-index: 22;
    "
    )]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "OpenSASS gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly. Developer-friendly tools, production-ready out of the box.".to_string()
    )]
    pub text: String,
}

#[function_component(HeroDescription)]
pub fn hero_description(props: &HeroDescriptionProps) -> Html {
    html! { <p style={props.style} class={props.class}>{ props.text.clone() }</p> }
}

#[derive(Properties, PartialEq)]
pub struct HeroActionsProps {
    #[prop_or(
        "
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        justify-content: center;
        gap: 12px;
        position: relative;
        width: 100%;
        max-width: 90vw;
        padding: 12px 16px;
        margin: 32px auto 0;
        z-index: 9;
    "
    )]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "
        background: linear-gradient(90deg, #7F37FF 0%, #8C2EFF 100%);
        color: white;
        font-size: 16px;
        font-weight: 600;
        border: none;
        border-radius: 9999px;
        padding: 14px 24px;
        height: 54px;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.05);
    "
    )]
    pub primary_button_style: &'static str,

    #[prop_or(
        "
        background: white;
        color: black;
        font-size: 16px;
        font-weight: 600;
        border: 1px solid #e5e7eb;
        border-radius: 9999px;
        padding: 14px 24px;
        height: 54px;
        cursor: pointer;
        display: flex;
        align-items: center;
        gap: 8px;
        transition: all 0.3s ease;
    "
    )]
    pub secondary_button_style: &'static str,

    #[prop_or("font-size: 18px;")]
    pub arrow_style: &'static str,
    #[prop_or("Get Started")]
    pub primary_button_label: &'static str,
    #[prop_or("Learn More")]
    pub secondary_button_label: &'static str,
    #[prop_or("→")]
    pub arrow_label: &'static str,
    #[prop_or_default]
    pub on_get_started: Callback<()>,
    #[prop_or_default]
    pub on_learn_more: Callback<()>,
}

#[function_component(HeroActions)]
pub fn hero_actions(props: &HeroActionsProps) -> Html {
    let handle_get_started = {
        let on_get_started = props.on_get_started.clone();
        Callback::from(move |_| on_get_started.emit(()))
    };

    let handle_learn_more = {
        let on_learn_more = props.on_learn_more.clone();
        Callback::from(move |_| on_learn_more.emit(()))
    };

    html! {
        <div style={props.style} class={props.class}>
            <button
                onclick={handle_get_started}
                aria-label={props.primary_button_label}
                style={props.primary_button_style}
            >
                { props.primary_button_label }
            </button>
            <button
                onclick={handle_learn_more}
                aria-label={props.secondary_button_label}
                style={props.secondary_button_style}
            >
                { props.secondary_button_label }
                <span style={props.arrow_style}>{ props.arrow_label }</span>
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CompanyLogoProps {
    #[prop_or_default]
    pub name: &'static str,

    #[prop_or_default]
    pub icon_class: &'static str,

    #[prop_or("font-size: 32px; color: #666; opacity: 0.7; transition: opacity 0.2s ease;")]
    pub icon_style: &'static str,

    #[prop_or("flex-shrink: 0; display: flex; align-items: center; justify-content: center;")]
    pub wrapper_style: &'static str,

    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(CompanyLogo)]
pub fn company_logo(props: &CompanyLogoProps) -> Html {
    html! {
        <div role="listitem" class={props.class} style={props.wrapper_style}>
            <i
                class={props.icon_class}
                style={props.icon_style}
                aria-label={format!("{} logo", props.name)}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CompaniesProps {
    #[prop_or(
        "
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: clamp(20px, 4vw, 40px);
        width: 90vw;
        max-width: 1028px;
        margin: 0 auto;
        z-index: 48;
        "
    )]
    pub style: &'static str,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "
        align-self: stretch;
        flex-shrink: 0;
        flex-basis: auto;
        position: relative;
        min-width: 0;
        color: #43454a;
        font-family: Helvetica Neue, sans-serif;
        font-size: clamp(16px, 2vw, 20px);
        font-weight: 400;
        line-height: 1.2;
        text-align: center;
        white-space: normal;
        z-index: 49;
        margin: 0;
        "
    )]
    pub title_style: &'static str,

    #[prop_or_default]
    pub title_class: &'static str,

    #[prop_or(
        "
        display: flex;
        align-items: center;
        justify-content: center;
        flex-wrap: nowrap;
        gap: clamp(30px, 5vw, 65px);
        width: 100%;
        max-width: 100%;
        flex-shrink: 0;
        position: relative;
        min-width: 0;
        z-index: 50;
        overflow-x: auto;
        -webkit-overflow-scrolling: touch;
        scrollbar-width: none; /* Firefox */
        "
    )]
    pub logos_style: &'static str,

    #[prop_or(
        "
        scrollbar-width: none;
    "
    )]
    pub logos_class: &'static str,

    #[prop_or("Trusted by teams building the future with Open SASS".to_string())]
    pub title_text: String,

    #[prop_or(vec![
        ("Google", "fa-brands fa-google"),
        ("Apple", "fa-brands fa-apple"),
        ("Microsoft", "fa-brands fa-microsoft"),
        ("Slack", "fa-brands fa-slack"),
        ("Github", "fa-brands fa-github"),
    ])]
    pub companies: Vec<(&'static str, &'static str)>,
}

#[function_component(Companies)]
pub fn companies(props: &CompaniesProps) -> Html {
    html! {
        <div style={props.style} class={props.class}>
            <p style={props.title_style} class={props.title_class}>{ props.title_text.clone() }</p>
            <div style={props.logos_style} class={props.logos_class}>
                { for props.companies.iter().map(|(name, icon)| html! {
                    <CompanyLogo name={name} icon_class={icon} />
                }) }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroProps {
    #[prop_or(
        "display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 42px; width: 100%; height: 50vh; z-index: 16;"
    )]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; align-items: center; align-self: stretch; flex-wrap: nowrap; flex-shrink: 0; gap: 18px; position: relative; min-width: 0; z-index: 17;"
    )]
    pub content_style: &'static str,
    #[prop_or_default]
    pub content_class: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; align-items: center; align-self: stretch; flex-wrap: nowrap; flex-shrink: 0; gap: 20px; position: relative; z-index: 20;"
    )]
    pub text_content_style: &'static str,
    #[prop_or_default]
    pub text_content_class: &'static str,

    #[prop_or(
        "display: flex; align-items: center; justify-content: center; flex-wrap: nowrap; flex-shrink: 0; gap: 4px; position: relative; width: 224px; height: 34px; padding: 8px 12px; background: rgba(255, 255, 255, 0.65); border: 0.6px solid #ffffff; z-index: 18; border-radius: 100px; backdrop-filter: blur(30px);"
    )]
    pub badge_style: &'static str,
    #[prop_or_default]
    pub badge_class: &'static str,
    #[prop_or(
        "flex-shrink: 0; flex-basis: auto; position: relative; height: 19px; color: #43454a; font-family: Inter, sans-serif; font-size: 16px; font-weight: 420; line-height: 19px; text-align: left; white-space: nowrap; z-index: 19;"
    )]
    pub badge_text_style: &'static str,
    #[prop_or_default]
    pub badge_text_class: &'static str,
    #[prop_or("Launching Soon".to_string())]
    pub badge_text: String,

    #[prop_or(
        "align-self: stretch; flex-shrink: 0; position: relative; font-family: Helvetica Neue, sans-serif; font-size: clamp(24px, 8vw, 76px); font-weight: 700; line-height: 1.2; text-align: center; letter-spacing: -0.5px; z-index: 21; margin: 0;"
    )]
    pub title_style: &'static str,
    #[prop_or_default]
    pub title_class: &'static str,

    #[prop_or(
        "position: relative; color: #141415; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px;"
    )]
    pub drive_style: &'static str,
    #[prop_or_default]
    pub drive_class: &'static str,

    #[prop_or(
        "position: relative; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px; background: linear-gradient(90deg, #6947ff, #6947ff); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
    )]
    pub growth_style: &'static str,
    #[prop_or_default]
    pub growth_class: &'static str,

    #[prop_or(
        "position: relative; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px; background: linear-gradient(90deg, #6e4dff, #8b4dff); -webkit-background-clip: text; -webkit-text-fill-color: transparent;"
    )]
    pub empty_style: &'static str,
    #[prop_or_default]
    pub empty_class: &'static str,

    #[prop_or(
        "position: relative; color: #141415; font-family: Helvetica Neue, sans-serif; font-size: clamp(32px, 6vw, 76px); font-weight: 700; line-height: clamp(40px, 6.3vw, 80px); text-align: center; letter-spacing: -0.76px;"
    )]
    pub through_style: &'static str,
    #[prop_or_default]
    pub through_class: &'static str,

    #[prop_or("Build ".to_string())]
    pub drive_text: String,
    #[prop_or("Apps".to_string())]
    pub growth_text: String,
    #[prop_or(" ".to_string())]
    pub empty_text: String,
    #[prop_or("Blazingly Fast".to_string())]
    pub through_text: String,

    #[prop_or(
        "display: flex; align-items: flex-start; justify-content: center; flex-shrink: 0; position: relative; max-width: 90vw; height: auto; font-size: clamp(14px, 4vw, 20px); line-height: 1.5; padding: 0 16px; text-align: center; margin: 0 auto; color: #43454a; z-index: 22;"
    )]
    pub description_style: &'static str,
    #[prop_or_default]
    pub description_class: &'static str,
    #[prop_or(
        "OpenSASS gives you everything you need to create, deploy, and scale full-stack applications using Rust and WebAssembly. Developer-friendly tools, production-ready out of the box.".to_string()
    )]
    pub description_text: String,

    #[prop_or(
        "display: flex; flex-direction: row; flex-wrap: wrap; justify-content: center; gap: 12px; position: relative; width: 100%; max-width: 90vw; padding: 12px 16px; margin: 32px auto 0; z-index: 9;"
    )]
    pub actions_style: &'static str,
    #[prop_or_default]
    pub actions_class: &'static str,

    #[prop_or(
        "background: linear-gradient(90deg, #7F37FF 0%, #8C2EFF 100%); color: white; font-size: 16px; font-weight: 600; border: none; border-radius: 9999px; padding: 14px 24px; height: 54px; cursor: pointer; transition: all 0.3s ease; box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.05);"
    )]
    pub primary_button_style: &'static str,

    #[prop_or(
        "background: white; color: black; font-size: 16px; font-weight: 600; border: 1px solid #e5e7eb; border-radius: 9999px; padding: 14px 24px; height: 54px; cursor: pointer; display: flex; align-items: center; gap: 8px; transition: all 0.3s ease;"
    )]
    pub secondary_button_style: &'static str,

    #[prop_or("font-size: 18px;")]
    pub arrow_style: &'static str,
    #[prop_or("Get Started")]
    pub primary_button_label: &'static str,
    #[prop_or("Learn More")]
    pub secondary_button_label: &'static str,
    #[prop_or("→")]
    pub arrow_label: &'static str,

    #[prop_or_default]
    pub on_get_started: Callback<()>,
    #[prop_or_default]
    pub on_learn_more: Callback<()>,

    #[prop_or(
        "
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: clamp(20px, 4vw, 40px);
        width: 90vw;
        max-width: 1028px;
        margin: 0 auto;
        z-index: 48;
        "
    )]
    pub companies_style: &'static str,
    #[prop_or_default]
    pub companies_class: &'static str,
    #[prop_or(
        "align-self: stretch; flex-shrink: 0; flex-basis: auto; position: relative; min-width: 0; color: #43454a; font-family: Helvetica Neue, sans-serif; font-size: clamp(16px, 2vw, 20px); font-weight: 400; line-height: 1.2; text-align: center; white-space: normal; z-index: 49; margin: 0;"
    )]
    pub companies_title_style: &'static str,
    #[prop_or_default]
    pub companies_title_class: &'static str,
    #[prop_or(
        "display: flex; align-items: center; justify-content: center; flex-wrap: nowrap; gap: clamp(30px, 5vw, 65px); width: 100%; max-width: 100%; flex-shrink: 0; position: relative; min-width: 0; z-index: 50; overflow-x: auto; -webkit-overflow-scrolling: touch; scrollbar-width: none;"
    )]
    pub companies_logos_style: &'static str,
    #[prop_or("scrollbar-width: none;")]
    pub companies_logos_class: &'static str,
    #[prop_or("Trusted by teams building the future with Open SASS".to_string())]
    pub companies_title_text: String,
    #[prop_or(vec![
        ("Google", "fa-brands fa-google"),
        ("Apple", "fa-brands fa-apple"),
        ("Microsoft", "fa-brands fa-microsoft"),
        ("Slack", "fa-brands fa-slack"),
        ("Github", "fa-brands fa-github"),
    ])]
    pub companies: Vec<(&'static str, &'static str)>,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <div>
            <Background />
            <main id="main-content">
                <section
                    style={props.style}
                    class={props.class}
                    role="main"
                    aria-labelledby="hero-title"
                >
                    <div style={props.content_style} class={props.content_class}>
                        <HeroBadge
                            style={props.badge_style}
                            class={props.badge_class}
                            text_style={props.badge_text_style}
                            text_class={props.badge_text_class}
                            text={props.badge_text.clone()}
                        />
                        <div style={props.text_content_style} class={props.text_content_class}>
                            <HeroTitle
                                style={props.title_style}
                                class={props.title_class}
                                drive_style={props.drive_style}
                                drive_class={props.drive_class}
                                growth_style={props.growth_style}
                                growth_class={props.growth_class}
                                empty_style={props.empty_style}
                                empty_class={props.empty_class}
                                through_style={props.through_style}
                                through_class={props.through_class}
                                drive_text={props.drive_text.clone()}
                                growth_text={props.growth_text.clone()}
                                empty_text={props.empty_text.clone()}
                                through_text={props.through_text.clone()}
                            />
                            <HeroDescription
                                style={props.description_style}
                                class={props.description_class}
                                text={props.description_text.clone()}
                            />
                        </div>
                    </div>
                    <HeroActions
                        style={props.actions_style}
                        class={props.actions_class}
                        primary_button_style={props.primary_button_style}
                        secondary_button_style={props.secondary_button_style}
                        arrow_style={props.arrow_style}
                        primary_button_label={props.primary_button_label}
                        secondary_button_label={props.secondary_button_label}
                        arrow_label={props.arrow_label}
                        on_get_started={props.on_get_started.clone()}
                        on_learn_more={props.on_learn_more.clone()}
                    />
                </section>
            </main>
            <Companies
                style={props.companies_style}
                class={props.companies_class}
                title_style={props.companies_title_style}
                title_class={props.companies_title_class}
                logos_style={props.companies_logos_style}
                logos_class={props.companies_logos_class}
                title_text={props.companies_title_text.clone()}
                companies={props.companies.clone()}
            />
        </div>
    }
}
