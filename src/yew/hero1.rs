use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    #[prop_or("fas fa-rocket")]
    pub icon: &'static str,

    #[prop_or("Launching Open SASS".to_string())]
    pub text: String,

    #[prop_or(
        "display: flex; width: 230px; padding: 8px 16px 8px 8px; gap: 11px; align-items: center; justify-content: center; background: rgba(252, 92, 64, 0.08); border-radius: 99px; overflow: hidden;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;"
    )]
    pub icon_style: &'static str,

    #[prop_or(
        "font-family: sans-serif; font-size: 14px; font-weight: 300; line-height: 16.8px; letter-spacing: 0.11px; color: white; white-space: nowrap;"
    )]
    pub text_style: &'static str,

    #[prop_or("badge-container")]
    pub container_class: &'static str,

    #[prop_or("badge-icon")]
    pub icon_class: &'static str,

    #[prop_or("badge-text")]
    pub text_class: &'static str,

    #[prop_or("image")]
    pub role: &'static str,

    #[prop_or("Badge")]
    pub aria_label: &'static str,

    #[prop_or("")]
    pub icon_alt: &'static str,

    #[prop_or("eager")]
    pub icon_loading: &'static str,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    html! {
        <div
            role={props.role}
            aria-label={props.aria_label}
            style={props.container_style}
            class={props.container_class}
        >
            <i class={props.icon} style={props.icon_style} aria-hidden="true" />
            <span style={props.text_style} class={props.text_class}>{ props.text.clone() }</span>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or("Start Building")]
    pub label: &'static str,

    #[prop_or("button")]
    pub button_type: &'static str,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or("primary-button")]
    pub class: &'static str,

    #[prop_or("fas fa-arrow-right")]
    pub icon: &'static str,

    #[prop_or("font-size: 16px; width: 1em; height: 1em; vertical-align: middle;")]
    pub icon_style: &'static str,

    #[prop_or("button-icon")]
    pub icon_class: &'static str,

    #[prop_or(
        "display: inline-flex; align-items: center; justify-content: center; gap: 8px; border: none; border-radius: 4px; font-family: sans-serif; font-weight: 300; cursor: pointer; transition: all 0.2s ease; text-decoration: none; white-space: nowrap; background:rgb(255, 38, 0); color: white; box-shadow: inset 0 8px 32px rgba(212, 212, 212, 0.64); height: 56px; padding: 0 24px; font-size: 18px; line-height: 21.6px; letter-spacing: 0.14px;"
    )]
    pub style: &'static str,

    #[prop_or("Get started with Open SASS")]
    pub aria_label: &'static str,

    #[prop_or("true")]
    pub aria_pressed: &'static str,
}
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            type={props.button_type}
            disabled={props.disabled}
            class={props.class}
            style={props.style}
            aria-label={props.aria_label}
            aria-pressed={props.aria_pressed}
        >
            { props.label }
            <i
                class={props.icon}
                style={props.icon_style}
                class={props.icon_class}
                aria-hidden="true"
            />
        </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroContentProps {
    #[prop_or("Build Ultra-Fast Web Apps with Open SASS".to_string())]
    pub heading: String,

    #[prop_or(
        "Open SASS brings modern Rust-powered speed to your web stack. Build scalable, reactive, and blazingly fast web apps with less effort and more performance.".to_string()
    )]
    pub description: String,

    #[prop_or("hero-container")]
    pub container_class: &'static str,

    #[prop_or("hero-wrapper")]
    pub wrapper_class: &'static str,

    #[prop_or("hero-content")]
    pub content_class: &'static str,

    #[prop_or("hero-title")]
    pub title_class: &'static str,

    #[prop_or("hero-description")]
    pub description_class: &'static str,

    #[prop_or("hero-cta")]
    pub cta_class: &'static str,

    #[prop_or(
        "width: 100%; padding: 40px 64px 64px; display: flex; flex-direction: column; align-items: center; position: relative; overflow: hidden; min-height: calc(100vh - 96vh);"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1440px; display: flex; flex-direction: column; align-items: center; gap: 80px; flex: 1; position: relative; z-index: 10;"
    )]
    pub wrapper_style: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; gap: 48px; align-items: center; width: 100%;"
    )]
    pub content_style: &'static str,

    #[prop_or("font-size: 48px; font-weight: bold; color: white; text-align: center;")]
    pub title_style: &'static str,

    #[prop_or(
        "max-width: 700px; text-align: center; color: white; font-size: 18px; font-weight: 300; line-height: 1.6;"
    )]
    pub description_style: &'static str,

    #[prop_or_default]
    pub cta_style: &'static str,

    #[prop_or("h1")]
    pub heading_tag: &'static str,

    #[prop_or("Hero Section")]
    pub aria_label: &'static str,
}

#[function_component(HeroContent)]
pub fn hero_content(props: &HeroContentProps) -> Html {
    html! {
        <div
            class={props.container_class}
            style={props.container_style}
            aria-label={props.aria_label}
        >
            <div
                class={props.wrapper_class}
                style={props.wrapper_style}
                aria-labelledby="hero-heading"
            >
                <div class={props.content_class} style={props.content_style}>
                    <Badge />
                    <@{props.heading_tag}
                        id="hero-heading"
                        class={props.title_class}
                        style={props.title_style}
                        role="heading"
                        aria-level="1"
                    >
                        { props.heading.clone() }
                    </@>
                    <p
                        class={props.description_class}
                        style={props.description_style}
                        aria-label="Platform description"
                    >
                        { props.description.clone() }
                    </p>
                </div>
                <div class={props.cta_class} style={props.cta_style} aria-label="Call to Action">
                    <Button />
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct TabButtonProps {
    #[prop_or_default]
    pub id: &'static str,

    #[prop_or_default]
    pub label: &'static str,

    #[prop_or_default]
    pub icon: &'static str,

    #[prop_or_default]
    pub is_active: bool,

    #[prop_or_default]
    pub on_click: Callback<()>,

    #[prop_or(
        "display: flex; height: 48px; padding: 0 24px; gap: 8px; justify-content: center; align-items: center; flex: 1; border-radius: 6px; border: 1px solid rgba(255, 255, 255, 0.24); background: rgba(0, 0, 0, 0.04); cursor: pointer; transition: all 0.2s ease; color: white;"
    )]
    pub style: &'static str,

    #[prop_or(
        "background: rgba(255, 255, 255, 0.16); border-color: rgba(255, 255, 255, 0.32); box-shadow: 0 4px 16px 0 rgba(51, 14, 11, 0.56);"
    )]
    pub active_style: &'static str,

    #[prop_or(
        "width: 24px; height: 24px; display: flex; align-items: center; justify-content: center;"
    )]
    pub icon_style: &'static str,

    #[prop_or(
        "font-family: sans-serif; font-size: 14px; font-weight: 400; line-height: 16.8px; letter-spacing: 0.11px; opacity: 0.8; white-space: nowrap;"
    )]
    pub label_style: &'static str,

    #[prop_or("opacity: 1;")]
    pub active_label_style: &'static str,
}

#[function_component(TabButton)]
pub fn tab_button(props: &TabButtonProps) -> Html {
    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    let onkeydown = {
        let on_click = props.on_click.clone();
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            if key == "Enter" || key == " " {
                e.prevent_default();
                on_click.emit(());
            }
        })
    };

    let combined_style = if props.is_active {
        format!("{} {}", props.style, props.active_style)
    } else {
        props.style.to_string()
    };

    let combined_label_style = if props.is_active {
        format!("{} {}", props.label_style, props.active_label_style)
    } else {
        props.label_style.to_string()
    };

    html! {
        <button
            id={format!("tab-{}", props.id)}
            role="tab"
            aria-selected={props.is_active.to_string()}
            aria-controls={format!("tabpanel-{}", props.id)}
            tabindex={if props.is_active { "0" } else { "-1" }}
            style={combined_style}
            onclick={onclick}
            onkeydown={onkeydown}
        >
            <i class={props.icon} style={props.icon_style} aria-hidden="true" />
            <span style={combined_label_style}>{ props.label }</span>
        </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProcessTabsProps {
    #[prop_or("display: flex; justify-content: center; align-items: center; width: 100%;")]
    pub container_style: &'static str,

    #[prop_or(
        "display: flex; gap: 16px; justify-content: center; align-items: center; border-radius: 12px; width: 100%; max-width: 800px;"
    )]
    pub tablist_style: &'static str,

    #[prop_or(&[
        ("connect", "Connect", "fas fa-link"),
        ("explore", "Explore", "fas fa-search"),
        ("activate", "Activate", "fas fa-bolt"),
    ])]
    pub tabs: &'static [(&'static str, &'static str, &'static str)],
}

#[function_component(ProcessTabs)]
pub fn process_tabs(props: &ProcessTabsProps) -> Html {
    let active_tab = use_state(|| "connect");

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |id: &'static str| {
            active_tab.set(id);
        })
    };

    html! {
        <div style={props.container_style}>
            <div role="tablist" aria-label="Platform process steps" style={props.tablist_style}>
                { for props.tabs.iter().map(|(id, label, icon)| {
                    let is_active = *active_tab == *id;
                    let on_click = {
                        let id = *id;
                        let on_tab_click = on_tab_click.clone();
                        Callback::from(move |_| on_tab_click.emit(id))
                    };
                    html! {
                        <TabButton
                            id={id}
                            label={label}
                            icon={icon}
                            is_active={is_active}
                            on_click={on_click}
                        />
                    }
                }) }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct HeroSectionProps {
    #[prop_or(
        "width: 100%; padding: 40px 64px 64px; display: flex; flex-direction: column; align-items: center; position: relative; overflow: hidden; min-height: calc(100vh - 96vh);"
    )]
    pub section_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1440px; display: flex; flex-direction: column; align-items: center; gap: 80px; flex: 1; position: relative; z-index: 10;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; gap: 48px; align-items: center; width: 100%;"
    )]
    pub content_style: &'static str,

    #[prop_or("Build Ultra-Fast Web Apps with Open SASS".to_string())]
    pub heading: String,

    #[prop_or(
        "Open SASS brings modern Rust-powered speed to your web stack. Build scalable, reactive, and blazingly fast web apps with less effort and more performance.".to_string()
    )]
    pub description: String,

    #[prop_or("hero-container")]
    pub container_class: &'static str,

    #[prop_or("hero-wrapper")]
    pub wrapper_class: &'static str,

    #[prop_or("hero-content")]
    pub content_class: &'static str,

    #[prop_or("hero-title")]
    pub title_class: &'static str,

    #[prop_or("hero-description")]
    pub description_class: &'static str,

    #[prop_or("hero-cta")]
    pub cta_class: &'static str,

    #[prop_or(
        "width: 100%; padding: 40px 64px 64px; display: flex; flex-direction: column; align-items: center; position: relative; overflow: hidden; min-height: calc(100vh - 96vh);"
    )]
    pub container_style_inner: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1440px; display: flex; flex-direction: column; align-items: center; gap: 80px; flex: 1; position: relative; z-index: 10;"
    )]
    pub wrapper_style: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; gap: 48px; align-items: center; width: 100%;"
    )]
    pub content_style_inner: &'static str,

    #[prop_or("font-size: 48px; font-weight: bold; color: white; text-align: center;")]
    pub title_style: &'static str,

    #[prop_or(
        "max-width: 700px; text-align: center; color: white; font-size: 18px; font-weight: 300; line-height: 1.6;"
    )]
    pub description_style: &'static str,

    #[prop_or_default]
    pub cta_style: &'static str,

    #[prop_or("h1")]
    pub heading_tag: &'static str,

    #[prop_or("Hero Section")]
    pub aria_label: &'static str,

    #[prop_or("display: flex; justify-content: center; align-items: center; width: 100%;")]
    pub tabs_container_style: &'static str,

    #[prop_or(
        "display: flex; gap: 16px; justify-content: center; align-items: center; border-radius: 12px; width: 100%; max-width: 800px;"
    )]
    pub tablist_style: &'static str,

    #[prop_or(&[
        ("connect", "Connect", "fas fa-link"),
        ("explore", "Explore", "fas fa-search"),
        ("activate", "Activate", "fas fa-bolt"),
    ])]
    pub tabs: &'static [(&'static str, &'static str, &'static str)],
}

#[function_component(Hero)]
pub fn hero(props: &HeroSectionProps) -> Html {
    html! {
        <section id="main-content" aria-labelledby="hero-heading" style={props.section_style}>
            <div style={props.container_style}>
                <div style={props.content_style}>
                    <HeroContent
                        heading={props.heading.clone()}
                        description={props.description.clone()}
                        container_class={props.container_class}
                        wrapper_class={props.wrapper_class}
                        content_class={props.content_class}
                        title_class={props.title_class}
                        description_class={props.description_class}
                        cta_class={props.cta_class}
                        container_style={props.container_style_inner}
                        wrapper_style={props.wrapper_style}
                        content_style={props.content_style_inner}
                        title_style={props.title_style}
                        description_style={props.description_style}
                        cta_style={props.cta_style}
                        heading_tag={props.heading_tag}
                        aria_label={props.aria_label}
                    />
                    <ProcessTabs
                        container_style={props.tabs_container_style}
                        tablist_style={props.tablist_style}
                        tabs={props.tabs}
                    />
                </div>
            </div>
        </section>
    }
}
