use web_sys::{MediaQueryList, wasm_bindgen::JsCast, wasm_bindgen::prelude::*, window};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    #[prop_or_default]
    pub text: &'static str,

    #[prop_or_default]
    pub container_class: &'static str,

    #[prop_or_default]
    pub text_class: &'static str,

    #[prop_or(
        "display: flex; align-items: center; justify-content: center; gap: 0.625rem; width: 11.8125rem; padding: 0.5rem 0.75rem; background: rgba(255, 255, 255, 0.5); border-radius: 6.25rem; backdrop-filter: blur(2.8125rem);"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "height: 1.0625rem; color: #0c1f2e; font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: clamp(0.875rem, 2vw, 1rem); font-weight: 400; line-height: 1.0625rem; white-space: nowrap;"
    )]
    pub text_style: &'static str,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    html! {
        <div style={props.container_style} class={props.container_class}>
            <span style={props.text_style} class={props.text_class}>{ &props.text }</span>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub text: Option<&'static str>,

    #[prop_or_default]
    pub icon: Option<&'static str>,

    #[prop_or_default]
    pub icon_alt: &'static str,

    #[prop_or("#")]
    pub href: &'static str,

    #[prop_or("primary")]
    pub variant: &'static str,

    #[prop_or_default]
    pub button_class: &'static str,

    #[prop_or_default]
    pub text_class: &'static str,

    #[prop_or_default]
    pub icon_class: &'static str,

    #[prop_or(
        "display: flex; align-items: center; justify-content: center; gap: 0.375rem; height: 3rem; padding: 0.5rem 0.875rem; border-radius: 62.4375rem; text-decoration: none; transition: all 0.2s ease; cursor: pointer;"
    )]
    pub button_style: &'static str,

    #[prop_or(
        "height: 1.5rem; color: #19191a; font-family: Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: clamp(0.875rem, 2vw, 1rem); font-weight: 500; line-height: 1.5rem; white-space: nowrap; letter-spacing: -0.01rem;"
    )]
    pub text_style: &'static str,

    #[prop_or("width: 1.5rem; height: 1.5rem; object-fit: cover;")]
    pub icon_style: &'static str,

    #[prop_or_default]
    pub aria_label: Option<&'static str>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <a
            href={props.href}
            style={props.button_style}
            class={props.button_class}
            role="button"
            aria-label={props.aria_label.unwrap_or(props.text.unwrap_or_default())}
        >
            { if let Some(text) = &props.text {
                    html! {
                        <span style={props.text_style} class={props.text_class}>{text}</span>
                    }
                } else {
                    html! {}
                } }
            { if let Some(icon) = props.icon {
                    html! {
                        <i class={icon} alt={props.icon_alt} style={props.icon_style} class={props.icon_class} aria-hidden={props.icon_alt.is_empty().to_string()}></i>
                    }
                } else {
                    html! {}
                } }
        </a>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroContentProps {
    #[prop_or("Open SASS")]
    pub badge_text: &'static str,

    #[prop_or("Rust for Modern Web Development")]
    pub title: &'static str,

    #[prop_or(
        "Ship blazingly fast full stack Rust web applications in Rust with Open SASS; built for performance, productivity, and modern development."
    )]
    pub description: &'static str,

    #[prop_or("Get Started")]
    pub primary_button_text: &'static str,

    #[prop_or("#contact")]
    pub primary_button_href: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; align-items: flex-start; gap: 1.375rem; position: absolute; width: 40.0625rem; height: 28.5625rem; top: 50%; left: 3.75rem; transform: translate(0, -40.59%); z-index: 2;"
    )]
    pub section_style: &'static str,

    #[prop_or_default]
    pub section_class: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; align-items: flex-start; align-self: stretch; gap: 2.5rem;"
    )]
    pub main_style: &'static str,

    #[prop_or_default]
    pub main_class: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; align-items: flex-start; align-self: stretch; gap: 1.25rem;"
    )]
    pub text_style: &'static str,

    #[prop_or_default]
    pub text_class: &'static str,

    #[prop_or(
        "width: 40rem; height: 13.125rem; color: #19191a; font-family: Montserrat, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: clamp(2rem, 5vw, 3.75rem); font-weight: 580; line-height: clamp(2.5rem, 6vw, 4.375rem); letter-spacing: -0.0375rem; margin: 0;"
    )]
    pub title_style: &'static str,

    #[prop_or_default]
    pub title_class: &'static str,

    #[prop_or(
        "width: 41.9375rem; height: 5.25rem; color: #404146; font-family: Montserrat, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: clamp(1rem, 2.5vw, 1.125rem); font-weight: 462; line-height: clamp(1.5rem, 3vw, 1.75rem); margin: 0;"
    )]
    pub description_style: &'static str,

    #[prop_or_default]
    pub description_class: &'static str,

    #[prop_or_default]
    pub actions_class: &'static str,

    #[prop_or("display: flex; align-items: center; gap: 1rem;")]
    pub actions_style: &'static str,

    #[prop_or(
        "display: inline-flex; align-items: center; padding: 0.875rem 1.5rem; background-color: white; color: #19191a; font-weight: 600; border-radius: 2rem; text-decoration: none; font-family: Montserrat, sans-serif; font-size: 1rem; box-shadow: 0 4px 10px rgba(0, 0, 0, 0.05); transition: all 0.3s ease;"
    )]
    pub primary_button_style: &'static str,

    #[prop_or(
        "display: inline-flex; justify-content: center; align-items: center; width: 2.75rem; height: 2.75rem; background-color: #19191a; color: white; border-radius: 50%; text-decoration: none; font-size: 1rem; transition: all 0.3s ease;"
    )]
    pub icon_button_style: &'static str,

    #[prop_or("fas fa-arrow-up-right-from-square")]
    pub icon_button_class: &'static str,
}

#[function_component(HeroContent)]
pub fn hero_content(props: &HeroContentProps) -> Html {
    html! {
        <section
            style={props.section_style}
            class={props.section_class}
            aria-labelledby="hero-title"
        >
            <Badge text={props.badge_text} />
            <div style={props.main_style} class={props.main_class}>
                <div style={props.text_style} class={props.text_class}>
                    <h1 id="hero-title" style={props.title_style} class={props.title_class}>
                        { &props.title }
                    </h1>
                    <p style={props.description_style} class={props.description_class}>
                        { &props.description }
                    </p>
                </div>
                <div style={props.actions_style} class={props.actions_class}>
                    <Button
                        text={Some(props.primary_button_text)}
                        href={props.primary_button_href}
                        icon_style={props.primary_button_style}
                    />
                    <Button
                        icon={Some(props.icon_button_class)}
                        icon_alt="Arrow pointing up and right"
                        href={props.primary_button_href}
                        aria_label={Some("Get in touch - Contact us")}
                        icon_style={props.icon_button_style}
                    />
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroImageProps {
    #[prop_or_default]
    pub src: &'static str,

    #[prop_or_default]
    pub alt: &'static str,

    #[prop_or_default]
    pub container_class: &'static str,

    #[prop_or_default]
    pub img_class: &'static str,

    #[prop_or(
        "width: 28.625rem; height: 39.5rem; background: #c3a2a2; border-radius: 1.25rem; overflow: hidden;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "width: 33.5rem; height: 59.75rem; object-fit: cover; transform: translate(-10%, -10%); position: relative; top: 10%; left: 10%;"
    )]
    pub img_style: &'static str,
}

#[function_component(HeroImage)]
pub fn hero_image(props: &HeroImageProps) -> Html {
    html! {
        <div style={props.container_style} class={props.container_class}>
            <img src={props.src} alt={props.alt} style={props.img_style} class={props.img_class} />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ServiceCardProps {
    #[prop_or_default]
    pub icon: &'static str,

    #[prop_or_default]
    pub icon_alt: &'static str,

    #[prop_or_default]
    pub title: &'static str,

    #[prop_or_default]
    pub container_class: &'static str,

    #[prop_or_default]
    pub content_class: &'static str,

    #[prop_or_default]
    pub title_class: &'static str,

    #[prop_or(
        "width: 12.5rem; height: 11.375rem; background: rgba(255, 255, 255, 0.3); border-radius: 2.5rem; backdrop-filter: blur(1.3125rem); box-shadow: -3.125rem 1.5625rem 4.375rem 0 rgba(232, 224, 224, 0.7); overflow: hidden;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; align-items: center; gap: 1.25rem; width: 9.125rem; margin: 3.5625rem 0 0 1.6875rem;"
    )]
    pub content_style: &'static str,

    #[prop_or("width: 2.875rem; height: 2.875rem; object-fit: cover;")]
    pub icon_style: &'static str,

    #[prop_or(
        "display: flex; align-items: flex-start; justify-content: center; align-self: stretch; width: 100%; min-width: 0; height: 3.125rem; color: #19191a; font-family: Montserrat, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: clamp(1rem, 2.5vw, 1.25rem); font-weight: 600; line-height: clamp(1.25rem, 3vw, 1.5625rem); text-align: center; margin: 0;"
    )]
    pub title_style: &'static str,
}

#[function_component(ServiceCard)]
pub fn service_card(props: &ServiceCardProps) -> Html {
    html! {
        <article style={props.container_style} class={props.container_class}>
            <div style={props.content_style} class={props.content_class}>
                <i class={props.icon} aria-hidden="true" title={props.icon_alt} />
                <h3 style={props.title_style} class={props.title_class}>{ &props.title }</h3>
            </div>
        </article>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroSectionProps {
    #[prop_or("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/e0rnow0h59d13gwrafwg.png")]
    pub background_image: &'static str,

    #[prop_or("https://avatars.githubusercontent.com/u/62179149?v=4")]
    pub team_image: &'static str,

    #[prop_or("fa-solid fa-code")]
    pub web_dev_icon: &'static str,

    #[prop_or("fa-solid fa-palette")]
    pub ui_ux_icon: &'static str,

    #[prop_or("Rusty Web Dev")]
    pub web_dev_title: &'static str,

    #[prop_or("UI UX")]
    pub ui_ux_title: &'static str,

    #[prop_or("Web development icon")]
    pub web_dev_icon_alt: &'static str,

    #[prop_or("UI UX design icon")]
    pub ui_ux_icon_alt: &'static str,

    #[prop_or("Enthusiastic team discussing all things opensass")]
    pub team_image_alt: &'static str,

    #[prop_or(
        "position: relative; width: 100%; min-height: 100vh; background: linear-gradient(135deg, #fcf3f9, #fcf9f3); overflow: hidden; padding: 2rem; display: flex; align-items: center; justify-content: center; box-sizing: border-box;"
    )]
    pub container_style: &'static str,

    #[prop_or_default]
    pub container_class: &'static str,

    #[prop_or("Hero section")]
    pub container_aria: &'static str,

    #[prop_or(
        "position: absolute; top: 0; left: 0; width: 100%; height: 100%; background-size: cover; background-position: center; opacity: 0.1; z-index: 0;"
    )]
    pub background_style: &'static str,

    #[prop_or_default]
    pub background_class: &'static str,

    #[prop_or("true")]
    pub background_aria: &'static str,

    #[prop_or(
        "position: relative; display: grid; grid-template-columns: 1fr 1fr; gap: 4rem; width: 100%; max-width: 1200px; align-items: center; z-index: 1; box-sizing: border-box;"
    )]
    pub layout_style: &'static str,

    #[prop_or_default]
    pub layout_class: &'static str,

    #[prop_or("width: 100%; box-sizing: border-box; padding: 0 10rem;")]
    pub left_content_style: &'static str,

    #[prop_or_default]
    pub left_content_class: &'static str,

    #[prop_or("position: relative; width: 100%; box-sizing: border-box; padding-left: 10rem;")]
    pub right_content_style: &'static str,

    #[prop_or_default]
    pub right_content_class: &'static str,

    #[prop_or("position: absolute; top: -5.5rem; left: 3.5rem; z-index: 2;")]
    pub card_top_left_style: &'static str,

    #[prop_or("position: absolute; bottom: -5.5rem; right: -4.5rem; z-index: 2;")]
    pub card_bottom_right_style: &'static str,
}

#[function_component(Hero)]
pub fn hero(props: &HeroSectionProps) -> Html {
    let is_mobile = use_state(|| false);
    let is_mobil_val = *is_mobile;

    use_effect_with((), move |_| {
        let window = window().unwrap();
        let media_query = window.match_media("(max-width: 768px)").unwrap().unwrap();

        is_mobile.set(media_query.matches());

        let is_mobile_cb = is_mobile.clone();
        let callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let mql: MediaQueryList = target.dyn_into().unwrap();
            is_mobile_cb.set(mql.matches());
        }) as Box<dyn FnMut(_)>);

        let _ = media_query.add_listener_with_opt_callback(Some(callback.as_ref().unchecked_ref()));

        callback.forget();
    });

    let layout_style = if is_mobil_val {
        "display: grid; grid-template-columns: 1fr; grid-template-rows: auto auto; gap: 4rem; width: 100%; max-width: 1200px; align-items: center; z-index: 1; box-sizing: border-box;"
    } else {
        props.layout_style
    };

    let left_content_style = if is_mobil_val {
        "width: 100%; box-sizing: border-box; padding: 0;"
    } else {
        props.left_content_style
    };

    let right_content_style = if is_mobil_val {
        "position: relative; width: 100%; box-sizing: border-box; padding-left: 0;"
    } else {
        props.right_content_style
    };

    html! {
        <section
            id="main-content"
            aria-labelledby="hero-heading"
            aria-label={props.container_aria}
            style={props.container_style}
            class={props.container_class}
        >
            <div
                style={format!("background-image: url('{}'); {}", props.background_image, props.background_style)}
                class={props.background_class}
                aria-hidden={props.background_aria}
            />
            <div style={layout_style} class={props.layout_class}>
                <div style={left_content_style} class={props.left_content_class}>
                    <HeroContent />
                </div>
                <div style={right_content_style} class={props.right_content_class}>
                    <HeroImage src={props.team_image} alt={props.team_image_alt} />
                    <div style={props.card_top_left_style}>
                        <ServiceCard
                            icon={props.web_dev_icon}
                            title={props.web_dev_title}
                            icon_alt={props.web_dev_icon_alt}
                        />
                    </div>
                    <div style={props.card_bottom_right_style}>
                        <ServiceCard
                            icon={props.ui_ux_icon}
                            title={props.ui_ux_title}
                            icon_alt={props.ui_ux_icon_alt}
                        />
                    </div>
                </div>
            </div>
        </section>
    }
}
