use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BackgroundElementProps {
    #[prop_or_default]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,
    #[prop_or_default]
    pub aria_label: &'static str,
}

#[function_component(BackgroundElement)]
pub fn background_element(props: &BackgroundElementProps) -> Html {
    html! {
        <div
            style={props.style}
            class={props.class}
            role="img"
            aria-hidden="true"
            aria-label={props.aria_label}
        />
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BackgroundElementsProps {
    #[prop_or(
        "width: 100%; height: clamp(200px, 50vw, 1141.089px); background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/yvgzditun47qlrthkqjh.png'); background-size: cover; background-repeat: no-repeat; position: absolute; left: -236px; z-index: 0;"
    )]
    pub bg1_style: &'static str,

    #[prop_or(
        "display: flex; width: clamp(200px, 25vw, 323.028px); height: clamp(200px, 30vw, 370.887px); flex-direction: column; gap: -5px; align-items: center; background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/41xl5eclo5v2mlcwfpez.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 511px; left: -182.514px; z-index: 18;"
    )]
    pub bg2_style: &'static str,

    #[prop_or(
        "display: flex; width: clamp(200px, 25vw, 323.028px); height: clamp(200px, 30vw, 370.887px); flex-direction: column; gap: -5px; align-items: center; background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/x74eknqpzqn1ublthl77.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 511px; left: 1297.486px; z-index: 28;"
    )]
    pub bg3_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1440px; height: clamp(50px, 10vw, 115.947px); background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/yrjdctfobseuguyixw8p.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 528.053px; left: 50%; transform: translateX(-50.07%); z-index: 4;"
    )]
    pub bg4_style: &'static str,
}

#[function_component(BackgroundElements)]
pub fn background_elements(props: &BackgroundElementsProps) -> Html {
    html! {
        <>
            <BackgroundElement style={props.bg1_style} />
            <BackgroundElement style={props.bg2_style} />
            <BackgroundElement style={props.bg3_style} />
            <BackgroundElement style={props.bg4_style} />
        </>
    }
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct HeroContentProps {
    #[prop_or(
        "display: flex; width: 100%; max-width: 1037px; min-height: 298px; flex-direction: column; gap: 32px; align-items: center; position: absolute; top: 0; left: 50%; transform: translateX(-37%); z-index: 29;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; gap: 16px; align-items: center; position: relative; z-index: 30;"
    )]
    pub content_style: &'static str,

    #[prop_or(
        "font-family: Geist; font-size: clamp(32px, 8vw, 72px); font-weight: 600; line-height: 1.3; letter-spacing: -1.44px; position: relative; text-align: center; z-index: 31;"
    )]
    pub heading_style: &'static str,

    #[prop_or("font-size: clamp(28px, 6vw, 63px); font-weight: 700; color:rgb(44, 21, 249);")]
    pub heading_accent_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1037px; font-family: Geist; font-size: clamp(14px, 4vw, 16px); font-weight: 500; line-height: 1.6; letter-spacing: -0.16px; position: relative; text-align: center; white-space: nowrap; z-index: 32;"
    )]
    pub subheading_style: &'static str,

    #[prop_or("font-weight: 700; color: #000;")]
    pub subheading_strong_style: &'static str,

    #[prop_or("font-weight: 500; color: rgba(0,0,0,0.7);")]
    pub subheading_soft_style: &'static str,

    #[prop_or("font-weight: 500; color:rgb(44, 21, 249);")]
    pub subheading_highlight_style: &'static str,

    #[prop_or("Build Full-Stack Rusty Apps,")]
    pub heading_text: &'static str,

    #[prop_or("Blazingly Fast with Open SASS")]
    pub heading_accent_text: &'static str,

    #[prop_or("Developer-friendly & Production-ready")]
    pub subheading_strong_text: &'static str,

    #[prop_or(".")]
    pub subheading_soft_text: &'static str,

    #[prop_or(" Start scaling Rust apps today.")]
    pub subheading_highlight_text: &'static str,
    #[prop_or_default]
    pub cta_text: &'static str,
    #[prop_or_default]
    pub cta_variant: &'static str,
    #[prop_or_default]
    pub cta_icon: &'static str,
    #[prop_or_default]
    pub cta_aria_label: &'static str,
}

#[function_component(HeroContent)]
pub fn hero_content(props: &HeroContentProps) -> Html {
    html! {
        <div style={props.container_style}>
            <div style={props.content_style}>
                <h1 style={props.heading_style}>
                    <span style="font-weight: 700; color: #000;">
                        { props.heading_text }
                        <br />
                    </span>
                    <span style={props.heading_accent_style}>{ props.heading_accent_text }</span>
                </h1>
                <p style={props.subheading_style}>
                    <span style={props.subheading_strong_style}>
                        { props.subheading_strong_text }
                    </span>
                    <span style={props.subheading_soft_style}>{ props.subheading_soft_text }</span>
                    <span style={props.subheading_highlight_style}>
                        { props.subheading_highlight_text }
                    </span>
                </p>
            </div>
            <CTAButton
                text={props.cta_text}
                variant={props.cta_variant}
                icon={props.cta_icon}
                aria_label={props.cta_aria_label}
            />
        </div>
    }
}
#[derive(Properties, PartialEq)]
pub struct RecentWorkIndicatorProps {
    #[prop_or(
        "display: flex; width: clamp(80px, 20vw, 115px); height: clamp(14px, 3vw, 19px); justify-content: center; font-family: 'Gochi Hand'; font-size: clamp(12px, 4vw, 16px); font-weight: 400; line-height: 1.2; color: rgb(44, 21, 249); position: absolute; top: 383px; left: 1245px; z-index: 2; text-align: center; text-transform: lowercase; white-space: nowrap;"
    )]
    pub style: &'static str,
}

#[function_component(RecentWorkIndicator)]
pub fn recent_work_indicator(props: &RecentWorkIndicatorProps) -> Html {
    html! {
        <span style={props.style} role="text" aria-label="Open SASS featured projects">
            { "Featured Open SASS Projects" }
        </span>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroSectionProps {
    #[prop_or(
        "width: 100%; max-width: 1160px; min-height: 302px; position: relative; z-index: 29; margin: clamp(48px, 10vw, 95px) clamp(16px, 5vw, 202px) 0 0;"
    )]
    pub style: &'static str,

    #[prop_or_default]
    pub content_props: HeroContentProps,

    #[prop_or_default]
    pub cta_text: &'static str,
    #[prop_or_default]
    pub cta_variant: &'static str,
    #[prop_or_default]
    pub cta_icon: &'static str,
    #[prop_or_default]
    pub cta_aria_label: &'static str,

    #[prop_or_default]
    pub recent_work_style: &'static str,
}

#[function_component(HeroSection)]
pub fn hero_section(props: &HeroSectionProps) -> Html {
    html! {
        <section style={props.style} role="region" aria-labelledby="hero-heading">
            <HeroContent
                cta_text={props.cta_text}
                cta_variant={props.cta_variant}
                cta_icon={props.cta_icon}
                cta_aria_label={props.cta_aria_label}
                ..props.content_props.clone()
            />
            <RecentWorkIndicator style={props.recent_work_style} />
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ArrowIndicatorProps {
    #[prop_or(
        "width: 133.581px; height: 71.334px; background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/sinvuvbm2bhew519kdl5.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 66.818px; left: 883.545px; z-index: 3;"
    )]
    pub style: &'static str,
}

#[function_component(ArrowIndicator)]
pub fn arrow_indicator(props: &ArrowIndicatorProps) -> Html {
    html! {
        <div
            style={props.style}
            role="img"
            aria-label="Decorative arrow pointing to portfolio items"
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct CTAButtonProps {
    #[prop_or_default]
    pub text: &'static str,
    #[prop_or_default]
    pub variant: &'static str,

    #[prop_or_default]
    pub icon: &'static str,

    #[prop_or_default]
    pub icon_url: &'static str,
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub aria_label: &'static str,
    #[prop_or(
        "display: flex; gap: 8px; justify-content: center; align-items: center; flex-shrink: 0; flex-wrap: nowrap; border-radius: 200px; position: relative; border: none; cursor: pointer; transition: all 0.2s ease; font-family: Geist; font-weight: 600; text-decoration: none; width: 222px; padding: 12px 24px; background-color: rgb(44, 21, 249); box-shadow: 0 24px 40px 0 rgba(249,115,21,0.12); z-index: 33;"
    )]
    pub primary_style: &'static str,
    #[prop_or("width: 244px; padding: 8px 24px; background-color: #fff; z-index: 15;")]
    pub secondary_style: &'static str,
    #[prop_or(
        "height: 26px; flex-shrink: 0; font-size: 16px; line-height: 25.6px; color: #fff; letter-spacing: -0.16px; position: relative; text-align: left; white-space: nowrap; z-index: 35;"
    )]
    pub primary_text_style: &'static str,
    #[prop_or(
        "height: 18px; flex-shrink: 0; font-size: 14px; line-height: 18px; color:rgb(44, 21, 249); letter-spacing: -0.14px; position: relative; text-align: left; text-transform: capitalize; white-space: nowrap; z-index: 16;"
    )]
    pub secondary_text_style: &'static str,
    #[prop_or(
        "width: 24px; height: 24px; flex-shrink: 0; border-radius: 200px; position: relative; overflow: hidden; z-index: 34;"
    )]
    pub icon_style: &'static str,
    #[prop_or_default]
    pub base_button_style: &'static str,

    #[prop_or("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/7iwgnzk0wvdvnckbt5ls.png")]
    pub calendar_icon_url: &'static str,

    #[prop_or("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/02ck0xlx5ezom2zcrzou.png")]
    pub default_icon_url: &'static str,
}

#[function_component(CTAButton)]
pub fn cta_button(props: &CTAButtonProps) -> Html {
    let button_style = format!(
        "{} {}",
        props.base_button_style,
        if props.variant == "primary" {
            props.primary_style
        } else {
            props.secondary_style
        }
    );

    let text_style = if props.variant == "primary" {
        props.primary_text_style
    } else {
        props.secondary_text_style
    };

    let icon_url = if !props.icon_url.is_empty() {
        props.icon_url
    } else if props.icon == "calendar" {
        props.calendar_icon_url
    } else {
        props.default_icon_url
    };

    let icon = html! {
        <div
            style={format!(
                "{} background-image: url('{}'); background-size: cover; background-repeat: no-repeat;",
                props.icon_style,
                icon_url
            )}
            role="img"
            aria-hidden="true"
        />
    };

    html! {
        <button
            style={button_style}
            onclick={props.on_click.clone()}
            aria-label={props.aria_label}
        >
            { icon }
            <span style={text_style}>{ props.text }</span>
        </button>
    }
}

#[derive(Properties, PartialEq, Default, Clone)]
pub struct PortfolioCardProps {
    pub id: &'static str,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub description: &'static str,
    #[prop_or_default]
    pub image: &'static str,
    #[prop_or_default]
    pub icon: &'static str,
    #[prop_or_default]
    pub background_image: &'static str,
    #[prop_or("top: 0px; left: 0px;")]
    pub position: &'static str,
    #[prop_or(1)]
    pub z_index: i32,
}

#[function_component(PortfolioCard)]
pub fn portfolio_card(props: &PortfolioCardProps) -> Html {
    let is_main_card = !props.title.is_empty() && !props.description.is_empty();

    let base_style = format!(
        "display: flex; width: {}; height: {}; flex-direction: column; gap: -5px; align-items: center; flex-wrap: nowrap; position: absolute; {}; z-index: {};",
        if is_main_card { "270px" } else { "297.647px" },
        if is_main_card { "329px" } else { "351.28px" },
        props.position,
        props.z_index,
    );

    if !is_main_card {
        return html! {
            <article
                style={format!(
                    "{} background-image: {}; background-size: cover; background-repeat: no-repeat;",
                    base_style,
                    props.background_image
                )}
                role="img"
                aria-label="Portfolio showcase item"
            />
        };
    }

    html! {
        <article style={base_style}>
            <div
                style={format!("width: 28px; height: 42px; background-image: url('{}'); background-size: cover; background-repeat: no-repeat; border-radius: 4px; position: relative; overflow: hidden; z-index: 21;", props.icon)}
                role="img"
                aria-label={format!("{} application icon", props.title)}
            />
            <div
                style="display: flex; padding: 12px; flex-direction: column; gap: 12px; align-items: flex-start; align-self: stretch; flex-shrink: 0; flex-wrap: nowrap; background-color: #fff; border-radius: 24px; border: 1px solid rgba(0,0,0,0.1); position: relative; box-sizing: content-box; z-index: 22;"
            >
                <div
                    style={format!("height: 180px; align-self: stretch; flex-shrink: 0; background-image: url('{}'); background-size: cover; background-repeat: no-repeat; border-radius: 16px; position: relative; z-index: 23;",  props.image)}
                    role="img"
                    aria-label={format!("Screenshot of {} application", props.title)}
                />
                <div
                    style="display: flex; padding: 0 8px 8px 8px; flex-direction: column; gap: 4px; align-items: flex-start; align-self: stretch; flex-shrink: 0; flex-wrap: nowrap; position: relative; z-index: 24;"
                >
                    <h3
                        style="height: 26px; align-self: stretch; flex-shrink: 0; font-family: Geist; font-size: 20px; font-weight: 500; line-height: 26px; color: #000; position: relative; text-align: left; text-transform: capitalize; white-space: nowrap; z-index: 25;"
                    >
                        { props.title }
                    </h3>
                    <p
                        style="display: flex; width: 230px; height: 38px; justify-content: flex-start; align-items: flex-start; align-self: stretch; flex-shrink: 0; font-family: Geist; font-size: 12px; font-weight: 400; line-height: 19.2px; color: rgba(0,0,0,0.6); position: relative; text-align: left; z-index: 26;"
                    >
                        { props.description }
                    </p>
                </div>
            </div>
        </article>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PortfolioGridProps {
    pub cards: Vec<PortfolioCardProps>,
}

#[function_component(PortfolioGrid)]
pub fn portfolio_grid(props: &PortfolioGridProps) -> Html {
    html! {
        <div style="width: 100%; height: 100%; position: relative;">
            { for props.cards.iter().cloned().map(|item| html! { <PortfolioCard ..item /> }) }
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PortfolioSectionProps {
    #[prop_or(
        "width: 100%; max-width: 1037.647px; height: auto; min-height: 363px; position: relative; z-index: 27; margin-top: 35.242px; margin-left: 201.177px; margin-right: 0; margin-bottom: 0;"
    )]
    pub style: &'static str,

    pub arrow_indicator_props: ArrowIndicatorProps,
    pub portfolio_grid_props: PortfolioGridProps,
}

#[function_component(PortfolioSection)]
pub fn portfolio_section(props: &PortfolioSectionProps) -> Html {
    html! {
        <section style={props.style} role="region" aria-labelledby="portfolio-heading">
            <h2 id="portfolio-heading" style="position: absolute; left: -9999px;">
                { "Recent Portfolio Work" }
            </h2>
            <ArrowIndicator ..props.arrow_indicator_props.clone() />
            <PortfolioGrid ..props.portfolio_grid_props.clone() />
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroProps {
    #[prop_or(
        "width: 100%; max-width: 1440px; height: auto; min-height: 1024px; background-color: #fff6f3; border-radius: 24px; border: 1px solid rgba(0,0,0,0.1); position: relative; overflow: hidden; margin: auto;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1438px; height: auto; min-height: 1024px; position: absolute; top: -1px; left: 50%; transform: translateX(-50%); overflow: hidden; z-index: 1;"
    )]
    pub inner_container_style: &'static str,

    #[prop_or(
        "width: 100%; height: clamp(200px, 50vw, 1141.089px); background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/yvgzditun47qlrthkqjh.png'); background-size: cover; background-repeat: no-repeat; position: absolute; left: -236px; z-index: 0;"
    )]
    pub bg1_style: &'static str,

    #[prop_or(
        "display: flex; width: clamp(200px, 25vw, 323.028px); height: clamp(200px, 30vw, 370.887px); flex-direction: column; gap: -5px; align-items: center; background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/41xl5eclo5v2mlcwfpez.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 511px; left: -182.514px; z-index: 18;"
    )]
    pub bg2_style: &'static str,

    #[prop_or(
        "display: flex; width: clamp(200px, 25vw, 323.028px); height: clamp(200px, 30vw, 370.887px); flex-direction: column; gap: -5px; align-items: center; background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/x74eknqpzqn1ublthl77.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 511px; left: 1297.486px; z-index: 28;"
    )]
    pub bg3_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1440px; height: clamp(50px, 10vw, 115.947px); background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/yrjdctfobseuguyixw8p.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 528.053px; left: 50%; transform: translateX(-50.07%); z-index: 4;"
    )]
    pub bg4_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1160px; min-height: 302px; position: relative; z-index: 29; margin: clamp(48px, 10vw, 95px) clamp(16px, 5vw, 202px) 0 0;"
    )]
    pub hero_section_style: &'static str,

    #[prop_or(
        "display: flex; width: 100%; max-width: 1037px; min-height: 298px; flex-direction: column; gap: 32px; align-items: center; position: absolute; top: 0; left: 50%; transform: translateX(-37%); z-index: 29;"
    )]
    pub content_container_style: &'static str,

    #[prop_or(
        "display: flex; flex-direction: column; gap: 16px; align-items: center; position: relative; z-index: 30;"
    )]
    pub content_style: &'static str,

    #[prop_or(
        "font-family: Geist; font-size: clamp(32px, 8vw, 72px); font-weight: 600; line-height: 1.3; letter-spacing: -1.44px; position: relative; text-align: center; z-index: 31;"
    )]
    pub heading_style: &'static str,

    #[prop_or("font-size: clamp(28px, 6vw, 63px); font-weight: 700; color:rgb(44, 21, 249);")]
    pub heading_accent_style: &'static str,

    #[prop_or(
        "width: 100%; max-width: 1037px; font-family: Geist; font-size: clamp(14px, 4vw, 16px); font-weight: 500; line-height: 1.6; letter-spacing: -0.16px; position: relative; text-align: center; white-space: nowrap; z-index: 32;"
    )]
    pub subheading_style: &'static str,

    #[prop_or("font-weight: 700; color: #000;")]
    pub subheading_strong_style: &'static str,

    #[prop_or("font-weight: 500; color: rgba(0,0,0,0.7);")]
    pub subheading_soft_style: &'static str,

    #[prop_or("font-weight: 500; color:rgb(44, 21, 249);")]
    pub subheading_highlight_style: &'static str,

    #[prop_or("Build Full-Stack Rusty Apps,")]
    pub heading_text: &'static str,

    #[prop_or("Blazingly Fast with Open SASS")]
    pub heading_accent_text: &'static str,

    #[prop_or("Developer-friendly & Production-ready")]
    pub subheading_strong_text: &'static str,

    #[prop_or(".")]
    pub subheading_soft_text: &'static str,

    #[prop_or(" Start scaling Rust apps today.")]
    pub subheading_highlight_text: &'static str,

    #[prop_or("Get Started")]
    pub cta_text: &'static str,

    #[prop_or("primary")]
    pub cta_variant: &'static str,

    #[prop_or("calendar")]
    pub cta_icon: &'static str,

    #[prop_or("Start using Open SASS for free")]
    pub cta_aria_label: &'static str,

    #[prop_or(
        "display: flex; width: clamp(80px, 20vw, 115px); height: clamp(14px, 3vw, 19px); justify-content: center; font-family: 'Gochi Hand'; font-size: clamp(12px, 4vw, 16px); font-weight: 400; line-height: 1.2; color: rgb(44, 21, 249); position: absolute; top: 383px; left: 1245px; z-index: 2; text-align: center; text-transform: lowercase; white-space: nowrap;"
    )]
    pub recent_work_style: &'static str,

    #[prop_or(vec![
        PortfolioCardProps {
            id: "left-card",
            background_image: "url(https://dev-to-uploads.s3.amazonaws.com/uploads/articles/wql84vmhe5723e8g9oac.png)",
            position: "top: 165px; left: 0;",
            z_index: 19,
            ..Default::default()
        },
        PortfolioCardProps {
            id: "center-card",
            title: "Open SASS",
            description: "OpenSass gives you everything you need to create, and deploy full-stack apps in Rust.",
            image: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/nlqpnq1e0jhqkyn87j3w.png",
            icon: "https://dev-to-uploads.s3.amazonaws.com/uploads/articles/219bfobabbc36oqd3o8n.png",
            position: "top: 200px; left: 383.823px;",
            z_index: 20,
            ..Default::default()
        },
        PortfolioCardProps {
            id: "right-card",
            background_image: "url(https://dev-to-uploads.s3.amazonaws.com/uploads/articles/6kryjan3hxyqvmidotsv.png)",
            position: "top: 168px; left: 740px;",
            z_index: 27,
            ..Default::default()
        },
    ])]
    pub cards: Vec<PortfolioCardProps>,
    #[prop_or("width: 133.581px; height: 71.334px; background-image: url('https://dev-to-uploads.s3.amazonaws.com/uploads/articles/sinvuvbm2bhew519kdl5.png'); background-size: cover; background-repeat: no-repeat; position: absolute; top: 66.818px; left: 883.545px; z-index: 3;")]
    pub arrow_indicator_style: &'static str,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <main
            style={props.container_style}
            role="main"
            aria-label="Product development landing page"
        >
            <BackgroundElements
                bg1_style={props.bg1_style}
                bg2_style={props.bg2_style}
                bg3_style={props.bg3_style}
                bg4_style={props.bg4_style}
            />
            <div style={props.inner_container_style}>
                <HeroSection
                    style={props.hero_section_style}
                    content_props={HeroContentProps {
                        container_style: props.content_container_style,
                        content_style: props.content_style,
                        heading_style: props.heading_style,
                        heading_accent_style: props.heading_accent_style,
                        subheading_style: props.subheading_style,
                        subheading_strong_style: props.subheading_strong_style,
                        subheading_soft_style: props.subheading_soft_style,
                        subheading_highlight_style: props.subheading_highlight_style,
                        heading_text: props.heading_text,
                        heading_accent_text: props.heading_accent_text,
                        subheading_strong_text: props.subheading_strong_text,
                        subheading_soft_text: props.subheading_soft_text,
                        subheading_highlight_text: props.subheading_highlight_text,
                        ..Default::default()
                    }}
                    cta_text={props.cta_text}
                    cta_variant={props.cta_variant}
                    cta_icon={props.cta_icon}
                    cta_aria_label={props.cta_aria_label}
                    recent_work_style={props.recent_work_style}
                />
                <PortfolioSection
                    arrow_indicator_props={
                        ArrowIndicatorProps {
                            style: props.arrow_indicator_style
                        }
                    }
                    portfolio_grid_props={PortfolioGridProps {
                        cards: props.cards.clone(),
                    }}
                />
            </div>
        </main>
    }
}
