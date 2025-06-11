use input_rs::yew::Input;
use pride_rs::Type;
use pride_rs::yew::FlagSection;
use regex::Regex;
use web_sys::{HtmlStyleElement, wasm_bindgen::JsCast, window};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CalendarButtonProps {
    #[prop_or("calendar-button-container")]
    pub container_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        top: -110px;
        left: 100px;
        z-index: 78;
    "#
    )]
    pub container_style: &'static str,

    #[prop_or("calendar-button")]
    pub button_class: &'static str,
    #[prop_or(
        r#"
        width: 327px;
        height: 94px;
        background-color: #ffffff;
        border-radius: 70px;
        border: 3px solid #000000;
        position: relative;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        padding: 0;
        animation: pulse 2s ease-in-out infinite;
    "#
    )]
    pub button_style: &'static str,
    #[prop_or("calendar-button pressed")]
    pub button_pressed_class: &'static str,

    #[prop_or("Add Event")]
    pub text: &'static str,
    #[prop_or("button-text")]
    pub text_class: &'static str,
    #[prop_or(
        r#"
        display: flex;
        width: 111px;
        height: 52px;
        justify-content: flex-start;
        align-items: flex-start;
        font-size: 20px;
        font-weight: 500;
        line-height: 26.4px;
        color: #000000;
        text-align: left;
        text-transform: capitalize;
        position: absolute;
        top: 31px;
        left: 38px;
        z-index: 81;
    "#
    )]
    pub text_style: &'static str,

    #[prop_or("button-icon")]
    pub icon_wrapper_class: &'static str,
    #[prop_or(
        r#"
        width: 172px;
        height: 94px;
        background-color: #7bcef2;
        border-radius: 70px;
        border: 3px solid #000000;
        position: absolute;
        top: -3px;
        left: 152px;
        z-index: 79;
        display: flex;
        align-items: center;
        justify-content: center;
    "#
    )]
    pub icon_wrapper_style: &'static str,

    #[prop_or("calendar-icon")]
    pub calendar_icon_class: &'static str,
    #[prop_or(r#"
        width: 69.693px;
        height: 48px;
        background-position: center;
        background-image: url("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ce6gy3t7cmsto11wk6gm.png");
        background-size: cover;
        background-repeat: no-repeat;
        z-index: 80;
    "#)]
    pub calendar_icon_style: &'static str,

    #[prop_or("Add Ferris Pride Dev Meetup event to your calendar")]
    pub aria_label: &'static str,
    #[prop_or("Ferris Pride Dev Meetup")]
    pub title: &'static str,
    #[prop_or("2025-06-22T18:00:00")]
    pub start: &'static str,
    #[prop_or("2025-06-22T22:00:00")]
    pub end: &'static str,
    #[prop_or(
        r#"
        Celebrate Rust and Pride with fellow devs at Ferris Cosmos Dev Community.
        Diversity, inclusion, and hacking welcome!
    "#
    )]
    pub description: &'static str,
    #[prop_or("Ferris Cosmos Dev Community")]
    pub location: &'static str,
}

#[function_component(CalendarButton)]
pub fn calendar_button(props: &CalendarButtonProps) -> Html {
    {
        use_effect_with((), move |_| {
            if let Some(document) = window().and_then(|w| w.document()) {
                if document.get_element_by_id("add-calendar").is_none() {
                    let style: HtmlStyleElement = document
                        .create_element("style")
                        .unwrap()
                        .dyn_into()
                        .unwrap();
                    style.set_id("add-calendar");
                    style.set_inner_text(include_str!("../css/pride/calendar-button.css"));
                    document.head().unwrap().append_child(&style).unwrap();
                }
            }
        });
    }

    let is_pressed = use_state(|| false);
    let onmousedown = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |_| is_pressed.set(true))
    };
    let onmouseup = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |_| is_pressed.set(false))
    };
    let onmouseleave = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |_| is_pressed.set(false))
    };

    let onclick = {
        let props = props.clone();
        Callback::from(move |_: MouseEvent| {
            let start = props.start.replace(['-', ':'], "").replace(['T', 'Z'], "");
            let end = props.end.replace(['-', ':'], "").replace(['T', 'Z'], "");
            let url = format!(
                "https://calendar.google.com/calendar/render?action=TEMPLATE&text={}&dates/{}Z/{}Z&details={}&location={}",
                urlencoding::encode(props.title),
                start,
                end,
                urlencoding::encode(props.description),
                urlencoding::encode(props.location),
            );
            if let Some(win) = window() {
                let _ = win.open_with_url_and_target(&url, "_blank");
            }
        })
    };

    let onkeydown = {
        let onclick = onclick.clone();
        let is_pressed = is_pressed.clone();
        Callback::from(move |e: KeyboardEvent| {
            if matches!(e.key().as_str(), "Enter" | " ") {
                e.prevent_default();
                is_pressed.set(true);
                onclick.emit(e.dyn_into().expect("event"));
            }
        })
    };
    let onkeyup = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |e: KeyboardEvent| {
            if matches!(e.key().as_str(), "Enter" | " ") {
                is_pressed.set(false);
            }
        })
    };

    html! {
        <div class={props.container_class} style={props.container_style}>
            <button
                type="button"
                class={if *is_pressed { props.button_pressed_class } else { props.button_class }}
                style={props.button_style}
                {onclick}
                {onkeydown}
                {onkeyup}
                {onmousedown}
                {onmouseup}
                {onmouseleave}
                aria-label={props.aria_label}
            >
                <span class={props.text_class} style={props.text_style}>{ props.text }</span>
                <div
                    class={props.icon_wrapper_class}
                    style={props.icon_wrapper_style}
                    aria-hidden="true"
                >
                    <div class={props.calendar_icon_class} style={props.calendar_icon_style} />
                </div>
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BrandLogoProps {
    #[prop_or("brand-logo")]
    pub container_class: &'static str,
    #[prop_or(
        r#"
        width: 399px;
        background-color: rgba(255, 255, 255, 0.01);
        position: absolute;
        top: 0;
        left: 498px;
        z-index: 5;
    "#
    )]
    pub container_style: &'static str,

    #[prop_or("logo-container")]
    pub logo_container_class: &'static str,
    #[prop_or_default]
    pub logo_container_style: &'static str,

    #[prop_or("logo-icon")]
    pub icon_class: &'static str,
    #[prop_or(
        r#"
        width: 11px;
        height: 12px;
        position: relative;
        z-index: 13;
        margin: 28.5px 0 0 189.5px;
    "#
    )]
    pub icon_style: &'static str,

    #[prop_or("brand-title")]
    pub title_class: &'static str,
    #[prop_or(
        r#"
        display: block;
        height: 29px;
        font-size: 24px;
        font-weight: 500;
        line-height: 29px;
        color: #000000;
        letter-spacing: 19.68px;
        text-align: left;
        text-transform: uppercase;
        white-space: nowrap;
        z-index: 14;
        margin: 11.5px 0 0 98px;
    "#
    )]
    pub title_style: &'static str,

    #[prop_or("Pride Month Flags")]
    pub sr_text: &'static str,
    #[prop_or("Pride Month Flags")]
    pub visible_text: &'static str,

    #[prop_or("logo-image")]
    pub image_class: &'static str,
    #[prop_or(r#"
        width: 337px;
        height: 309px;
        background-position: center;
        background-image: url("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/vf7ev9uxrqclih29tyz0.png");
        background-size: cover;
        background-repeat: no-repeat;
        position: relative;
        z-index: 48;
        margin: 16px 0 0 31px;
        animation: float 3s ease-in-out infinite;
    "#)]
    pub image_style: &'static str,

    #[prop_or("logo-graphic")]
    pub graphic_class: &'static str,
    #[prop_or(r#"
        width: 334px;
        height: 285.05px;
        background-position: center;
        background-image: url("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/usrbs6gw498pg7fcq8lu.png");
        background-size: cover;
        background-repeat: no-repeat;
        position: relative;
        z-index: 49;
        margin: 11px 0 0 2px;
        animation: float 3s ease-in-out infinite;
    "#)]
    pub graphic_style: &'static str,
}

#[function_component(BrandLogo)]
pub fn brand_logo(props: &BrandLogoProps) -> Html {
    use_effect_with((), |_| {
        if let Some(document) = window().and_then(|w| w.document()) {
            if document.get_element_by_id("brand-logo").is_none() {
                let style: HtmlStyleElement = document
                    .create_element("style")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                style.set_id("brand-logo");
                style.set_inner_text(include_str!("../css/pride/calendar-button.css"));
                document.head().unwrap().append_child(&style).unwrap();
            }
        }
    });

    html! {
        <section
            class={props.container_class}
            style={props.container_style}
            aria-labelledby="brand-title"
        >
            <div class={props.logo_container_class} style={props.logo_container_style}>
                <div class={props.icon_class} style={props.icon_style} aria-hidden="true" />
                <div
                    class={props.image_class}
                    style={props.image_style}
                    role="img"
                    aria-label="QUEPIT brand illustration"
                >
                    <div class={props.graphic_class} style={props.graphic_style} />
                </div>
            </div>
        </section>
    }
}

fn validate_email(email: String) -> bool {
    let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
    pattern.is_match(&email)
}

#[derive(Properties, PartialEq, Clone)]
pub struct EmailInputProps {
    pub on_submit: Callback<String>,

    #[prop_or_default]
    pub error: &'static str,

    #[prop_or_default]
    pub is_submitted: bool,

    #[prop_or("width:100%; margin-top:30px;")]
    pub form_style: &'static str,

    #[prop_or(
        "display:flex; width:556px; padding:24px 300px 24px 24px; gap:10px; align-items:center; border-radius:20px; border:2px solid black; background-color:white; position:relative; transition:border-color 0.2s ease;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "width:100%; height:26px; line-height:26px; background:none; border:none; outline:none;"
    )]
    pub input_style: &'static str,

    #[prop_or(
        "position:absolute; right:10px; top:50%; transform:translateY(-50%); width:40px; height:40px; background-color:pink; border:2px solid black; border-radius:50%; cursor:pointer; display:flex; align-items:center; justify-content:center; transition:all 0.2s ease;"
    )]
    pub button_style: &'static str,

    #[prop_or(
        "width:20px; height:20px; background-position:center; background-image:url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke='currentColor'%3E%3Cpath stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='M12 4l8 8-8 8M4 12h16'/%3E%3C/svg%3E\"); background-size:contain; background-repeat:no-repeat;"
    )]
    pub icon_style: &'static str,

    #[prop_or(
        "background-color:rgba(211,47,47,0.1); border:1px solid #d32f2f; border-radius:8px; padding:10px; margin-top:10px; color:#d32f2f; font-size:14px; text-align:left;"
    )]
    pub error_style: &'static str,
}

#[function_component(EmailInput)]
pub fn email_input(props: &EmailInputProps) -> Html {
    let email_ref = use_node_ref();
    let email = use_state(String::default);

    let focused = use_state(|| false);

    let valid_handle = use_state(|| true);

    let on_submit = {
        let email = email.clone();
        let on_submit_cb = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if email.trim().is_empty() {
                return;
            }
            on_submit_cb.emit(email.trim().to_string());
        })
    };

    let mut container_style = props.container_style.to_string();
    if *focused {
        container_style.push_str(" border-color:gold;");
    }

    let mut button_style = props.button_style.to_string();
    if props.is_submitted || email.trim().is_empty() {
        button_style.push_str(" opacity:0.5; cursor:not-allowed;");
    }

    html! {
        <form style={props.form_style} onsubmit={on_submit} novalidate=true>
            <div
                style={container_style}
                onfocusin={let focused = focused.clone(); Callback::from(move |_| focused.set(true))}
                onfocusout={let focused = focused.clone(); Callback::from(move |_| focused.set(false))}
            >
                <Input
                    r#type="email"
                    label="Email Address"
                    name="email"
                    placeholder="Enter your email address"
                    class="field"
                    field_class=""
                    label_class="sr-only"
                    input_class={props.input_style}
                    error_class="text-red-500 text-sm my-2"
                    error_message="Invalid email"
                    handle={email.clone()}
                    valid_handle={valid_handle}
                    validate_function={validate_email}
                    required=true
                    r#ref={email_ref}
                />
                <button
                    type="submit"
                    style={button_style}
                    aria-label="Subscribe to Ferris Cosmos Dev Community updates"
                    disabled={props.is_submitted || email.trim().is_empty()}
                >
                    <span class="sr-only">{ "Subscribe" }</span>
                    <div style={props.icon_style} aria-hidden="true" />
                </button>
            </div>
        </form>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct EventDetailsProps {
    #[prop_or("Pride Month Celebration")]
    pub title: &'static str,

    #[prop_or("")]
    pub description: &'static str,

    #[prop_or("position: relative; top: 50px; height: auto; padding: 20px;")]
    pub container_style: &'static str,

    #[prop_or("position: relative; top: 0; left: 0; text-align: center; margin: 0 auto;")]
    pub description_container_style: &'static str,

    #[prop_or(
        "display: flex; width: 297px; height: 241px; justify-content: flex-start; align-items: flex-start; font-size: 18px; font-weight: 400; line-height: 35.46px; color: black; text-align: left; margin: 0;"
    )]
    pub description_text_style: &'static str,

    #[prop_or("position: absolute; width: 100%; height: 100%;")]
    pub decorative_elements_style: &'static str,

    #[prop_or(
        "position: absolute; background-position: center; background-size: cover; background-repeat: no-repeat;"
    )]
    pub decorative_element_base_style: &'static str,

    #[prop_or(
        "width: 10px; height: 10px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/iktrqqiphxv3de0iliff.png\"); background-size: 100% 100%; top: 57.46%; left: 7.71%; z-index: 32;"
    )]
    pub star_1_style: &'static str,

    #[prop_or(
        "width: 15px; height: 15px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/r1tw21kq3qn0o3f799k5.png\"); background-size: 100% 100%; top: 63.02%; left: 6.79%; z-index: 31;"
    )]
    pub star_2_style: &'static str,

    #[prop_or(
        "width: 29px; height: 29px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/l1c43wpvhq9f09w3wxdx.png\"); background-size: 100% 100%; top: 68.59%; left: 3.43%; z-index: 34;"
    )]
    pub star_3_style: &'static str,

    #[prop_or(
        "width: 27px; height: 27px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/y0owpn1u84hc00c35of7.png\"); background-size: 100% 100%; top: 79.42%; left: 82.86%; z-index: 41;"
    )]
    pub misc_1_style: &'static str,

    #[prop_or(
        "width: 15px; height: 15px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/uzbqeqe5755b3wipuruj.png\"); background-size: 100% 100%; top: 79.62%; left: 3.07%; z-index: 19;"
    )]
    pub misc_2_style: &'static str,
}

#[function_component(EventDetails)]
pub fn event_details(props: &EventDetailsProps) -> Html {
    html! {
        <section style={props.container_style} aria-labelledby="event-details-title">
            <h2 id="event-details-title" class="sr-only">{ props.title }</h2>
            <div class="event-details-container">
                <div style={props.description_container_style}>
                    <p style={props.description_text_style}>{ props.description }</p>
                </div>
            </div>
            <div style={props.decorative_elements_style} aria-hidden="true">
                <div style={props.star_1_style} />
                <div style={props.star_2_style} />
                <div style={props.star_3_style} />
                <div style={props.misc_1_style} />
                <div style={props.misc_2_style} />
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct EventImageProps {
    #[prop_or("event-image-container")]
    pub container_class: &'static str,
    #[prop_or("position: relative; width: 100%; height: 620px; top: 295px; left: 259px;")]
    pub container_style: &'static str,

    #[prop_or("main-event-image")]
    pub main_image_class: &'static str,
    #[prop_or(
        r#"
        width: 767px;
        height: 620px;
        position: absolute;
        top: 0;
        left: 0;
        z-index: 75;
    "#
    )]
    pub main_image_style: &'static str,

    #[prop_or("curved-text-container")]
    pub curved_text_class: &'static str,
    #[prop_or("position: relative; width: 100%; height: 100%;")]
    pub curved_text_style: &'static str,

    #[prop_or("curved-letter curved-s")]
    pub curved_s_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 30.345px;
        top: 117px;
        left: 1.788px;
        transform: rotate(6.86deg);
        z-index: 99;
    "#
    )]
    pub curved_s_style: &'static str,

    #[prop_or("curved-letter curved-l")]
    pub curved_l_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 29.471px;
        top: 120.693px;
        left: 17.982px;
        transform: rotate(17.33deg);
        z-index: 100;
    "#
    )]
    pub curved_l_style: &'static str,

    #[prop_or("curved-love-container")]
    pub curved_love_container_class: &'static str,
    #[prop_or(
        "width: 8.29%; height: 60.89%; position: absolute; top: 20.25%; left: 3%; z-index: 103;"
    )]
    pub curved_love_container_style: &'static str,

    #[prop_or("curved-letter curved-o")]
    pub curved_o_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 32.643px;
        top: 0;
        left: 3.946px;
        transform: rotate(28.53deg);
        z-index: 101;
    "#
    )]
    pub curved_o_style: &'static str,

    #[prop_or("curved-letter curved-v")]
    pub curved_v_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 30.821px;
        top: 13.052px;
        left: 19.471px;
        transform: rotate(42.61deg);
        z-index: 102;
    "#
    )]
    pub curved_v_style: &'static str,

    #[prop_or("curved-letter curved-e")]
    pub curved_e_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 27.728px;
        color: #7bcef2;
        height: 27.728px;
        top: 30.059px;
        left: 31.708px;
        transform: rotate(56.33deg);
        z-index: 103;
    "#
    )]
    pub curved_e_style: &'static str,

    #[prop_or("secondary-event-image")]
    pub secondary_image_class: &'static str,
    #[prop_or(
        r#"
        width: 710.437px;
        height: 533.422px;
        position: absolute;
        top: 76px;
        left: 76.781px;
        z-index: 74;
    "#
    )]
    pub secondary_image_style: &'static str,

    #[prop_or("circular-profile-image")]
    pub profile_image_class: &'static str,
    #[prop_or(
        r#"
        width: 190px;
        height: 190px;
        border-radius: 50%;
        position: absolute;
        top: 109px;
        left: -95px;
        z-index: 90;
    "#
    )]
    pub profile_image_style: &'static str,

    #[prop_or("love-text-container")]
    pub love_text_class: &'static str,
    #[prop_or("position: absolute; top: 117px; left: -95px;")]
    pub love_text_style: &'static str,

    #[prop_or("love-letter love-i")]
    pub love_i_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 29.32px;
        top: 0.072px;
        left: 90.533px;
        transform: rotate(-3.61deg);
        z-index: 98;
    "#
    )]
    pub love_i_style: &'static str,

    #[prop_or("love-letter love-e")]
    pub love_e_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 31.575px;
        top: 1.466px;
        left: 68.172px;
        transform: rotate(-14.44deg);
        z-index: 97;
    "#
    )]
    pub love_e_style: &'static str,

    #[prop_or("love-letter love-v")]
    pub love_v_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 32.174px;
        top: 8.558px;
        left: 46.602px;
        transform: rotate(-28.17deg);
        z-index: 96;
    "#
    )]
    pub love_v_style: &'static str,

    #[prop_or("love-letter love-o")]
    pub love_o_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 29.045px;
        color: #7bcef2;
        height: 31.552px;
        top: 20.903px;
        left: 27.262px;
        transform: rotate(-42.25deg);
        z-index: 95;
    "#
    )]
    pub love_o_style: &'static str,

    #[prop_or("love-letter love-l")]
    pub love_l_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        font-size: 24px;
        font-weight: 400;
        line-height: 27.728px;
        color: #7bcef2;
        height: 27.728px;
        top: 38.637px;
        left: 13px;
        transform: rotate(-56.33deg);
        z-index: 94;
    "#
    )]
    pub love_l_style: &'static str,

    #[prop_or("decorative-elements")]
    pub decorative_elements_class: &'static str,
    #[prop_or("position: absolute; width: 100%; height: 100%;")]
    pub decorative_elements_style: &'static str,

    #[prop_or("decorative-element element-1")]
    pub element1_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        width: 37.276px;
        height: 193.82px;
        top: 157px;
        left: -220px;
        z-index: 105;
    "#
    )]
    pub element1_style: &'static str,

    #[prop_or("decorative-element element-2")]
    pub element2_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        width: 45.031px;
        height: 39.835px;
        top: 183.696px;
        left: -22px;
        z-index: 92;
    "#
    )]
    pub element2_style: &'static str,

    #[prop_or("decorative-element element-3")]
    pub element3_class: &'static str,
    #[prop_or(
        r#"
        position: absolute;
        width: 173.605px;
        height: 66.366px;
        background-image: url("");
        top: 224.331px;
        left: -86.605px;
        z-index: 104;
    "#
    )]
    pub element3_style: &'static str,
}

#[function_component(EventImage)]
pub fn event_image(props: &EventImageProps) -> Html {
    use_effect_with((), |_| {
        if let Some(document) = window().and_then(|w| w.document()) {
            if document.get_element_by_id("event-image").is_none() {
                let style: HtmlStyleElement = document
                    .create_element("style")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                style.set_id("event-image");
                style.set_inner_text(include_str!("../css/pride/event-image.css"));
                document.head().unwrap().append_child(&style).unwrap();
            }
        }
    });

    html! {
        <div class={props.container_class} style={props.container_style}>
            <div
                class={props.main_image_class}
                style={props.main_image_style}
                role="img"
                aria-label="Pride celebration event illustration with colorful decorative elements"
            >
                <div
                    class={props.curved_text_class}
                    style={props.curved_text_style}
                    aria-hidden="true"
                >
                    <span class={props.curved_s_class} style={props.curved_s_style}>{ "s" }</span>
                    <span class={props.curved_l_class} style={props.curved_l_style}>{ "l" }</span>
                    <div
                        class={props.curved_love_container_class}
                        style={props.curved_love_container_style}
                    >
                        <span class={props.curved_o_class} style={props.curved_o_style}>
                            { "o" }
                        </span>
                        <span class={props.curved_v_class} style={props.curved_v_style}>
                            { "v" }
                        </span>
                        <span class={props.curved_e_class} style={props.curved_e_style}>
                            { "e" }
                        </span>
                    </div>
                </div>
            </div>
            <div
                class={props.secondary_image_class}
                style={props.secondary_image_style}
                role="img"
                aria-label="Additional pride celebration graphics"
            />
            <div
                class={props.profile_image_class}
                style={props.profile_image_style}
                role="img"
                aria-label="Profile image of event participant"
            />
            <div class={props.love_text_class} style={props.love_text_style} aria-hidden="true">
                <span class={props.love_i_class} style={props.love_i_style}>{ "i" }</span>
                <span class={props.love_e_class} style={props.love_e_style}>{ "e" }</span>
                <span class={props.love_v_class} style={props.love_v_style}>{ "v" }</span>
                <span class={props.love_o_class} style={props.love_o_style}>{ "o" }</span>
                <span class={props.love_l_class} style={props.love_l_style}>{ "L" }</span>
            </div>
            <div
                class={props.decorative_elements_class}
                style={props.decorative_elements_style}
                aria-hidden="true"
            >
                <div class={props.element1_class} style={props.element1_style} />
                <div class={props.element2_class} style={props.element2_style} />
                <div class={props.element3_class} style={props.element3_style} />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct LoginButtonProps {
    #[prop_or("login-button")]
    pub button_class: &'static str,

    #[prop_or(
        r#"
        display: flex;
        height: 32px;
        justify-content: flex-start;
        align-items: flex-start;
        font-size: 24px;
        font-weight: 500;
        line-height: 31.68px;
        color: #000000;
        letter-spacing: 0.24px;
        position: absolute;
        top: 53px;
        left: 1241px;
        text-align: left;
        text-transform: capitalize;
        white-space: nowrap;
        z-index: 56;
        background: none;
        border: none;
        cursor: pointer;
        transition: all 0.2s ease;
    "#
    )]
    pub button_style: &'static str,
}

#[function_component(LoginButton)]
pub fn login_button(props: &LoginButtonProps) -> Html {
    let is_pressed = use_state(|| false);

    let on_click = {
        Callback::from(move |_| {
            web_sys::console::log_1(&"Login clicked".into());
        })
    };

    let on_keydown = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" || e.key() == " " {
                e.prevent_default();
                is_pressed.set(true);
                web_sys::console::log_1(&"Login clicked".into());
            }
        })
    };

    let on_keyup = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" || e.key() == " " {
                is_pressed.set(false);
            }
        })
    };

    let on_mousedown = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |_| is_pressed.set(true))
    };

    let on_mouseup = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |_| is_pressed.set(false))
    };

    let on_mouseleave = {
        let is_pressed = is_pressed.clone();
        Callback::from(move |_| is_pressed.set(false))
    };

    let class = if *is_pressed {
        format!("{} pressed", props.button_class)
    } else {
        props.button_class.to_string()
    };

    use_effect_with((), |_| {
        if let Some(document) = web_sys::window().unwrap().document() {
            if document.get_element_by_id("login-button").is_none() {
                let style = document.create_element("style").unwrap();
                style.set_id("login-button");
                style.set_inner_html(include_str!("../css/pride/login-button.css"));
                document.head().unwrap().append_child(&style).unwrap();
            }
        }
        || ()
    });

    html! {
        <button
            class={class}
            style={props.button_style}
            type="button"
            aria-label="Login to your account"
            onclick={on_click}
            onkeydown={on_keydown}
            onkeyup={on_keyup}
            onmousedown={on_mousedown}
            onmouseup={on_mouseup}
            onmouseleave={on_mouseleave}
        >
            { "LOGIN" }
        </button>
    }
}

#[derive(Clone, PartialEq)]
pub struct NavigationItem {
    pub id: String,
    pub label: String,
    pub href: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NavigationProps {
    #[prop_or("navigation")]
    pub container_class: &'static str,

    #[prop_or(
        r#"
        position: absolute;
        top: 55px;
        left: 80px;
        z-index: 6;
    "#
    )]
    pub container_style: &'static str,

    #[prop_or("navigation-list")]
    pub list_class: &'static str,

    #[prop_or(
        r#"
        display: flex;
        width: 388px;
        height: 26px;
        gap: 20px;
        align-items: center;
        flex-wrap: nowrap;
        list-style: none;
        margin: 0;
        padding: 0;
    "#
    )]
    pub list_style: &'static str,

    #[prop_or("navigation-item")]
    pub item_class: &'static str,

    #[prop_or(
        r#"
        display: flex;
        align-items: center;
        gap: 20px;
    "#
    )]
    pub item_style: &'static str,

    #[prop_or("navigation-link")]
    pub link_class: &'static str,

    #[prop_or(
        r#"
        height: 26px;
        flex-shrink: 0;
        flex-basis: auto;
        font-size: 20px;
        font-weight: 500;
        line-height: 26px;
        color: #000000;
        text-align: left;
        text-transform: capitalize;
        white-space: nowrap;
        text-decoration: none;
        cursor: pointer;
        transition: color 0.2s ease;
    "#
    )]
    pub link_style: &'static str,

    #[prop_or("navigation-link active")]
    pub active_link_class: &'static str,

    #[prop_or(
        r#"
        color: #7bcef2;
        font-weight: 600;
    "#
    )]
    pub active_link_style: &'static str,

    #[prop_or("navigation-separator")]
    pub separator_class: &'static str,

    #[prop_or(r#"
        width: 10px;
        height: 10px;
        flex-shrink: 0;
        background-position: center;
        background-image: url("https://dev-to-uploads.s3.amazonaws.com/uploads/articles/a4vtz4puc3plzgawwe4l.png");
        background-size: cover;
        background-repeat: no-repeat;
        border-radius: 50%;
    "#)]
    pub separator_style: &'static str,
}

#[function_component(Navigation)]
pub fn navigation(props: &NavigationProps) -> Html {
    let active_item = use_state(|| "home".to_string());
    let live_region_id = "nav-announcements";

    let navigation_items = vec![
        NavigationItem {
            id: "home".to_string(),
            label: "HOME".to_string(),
            href: "#home".to_string(),
        },
        NavigationItem {
            id: "features".to_string(),
            label: "FEATURES".to_string(),
            href: "#features".to_string(),
        },
        NavigationItem {
            id: "contact".to_string(),
            label: "CONTACT".to_string(),
            href: "#contact".to_string(),
        },
    ];

    {
        use_effect_with((), |_| {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if document.get_element_by_id("navigation").is_none() {
                    let style = document.create_element("style").unwrap();
                    style.set_id("navigation");
                    style.set_inner_html(include_str!("../css/pride/navigation.css"));
                    document.head().unwrap().append_child(&style).unwrap();
                }
            }
            || ()
        });
    }

    let handle_navigation = {
        let active_item = active_item.clone();
        Callback::from(move |item: NavigationItem| {
            active_item.set(item.id.to_string());

            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(live_region) = doc.get_element_by_id(live_region_id) {
                    live_region.set_text_content(Some(&format!("Navigated to {}", item.label)));
                }
            }
        })
    };
    let nav_items: Vec<Html> = navigation_items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let item = item.clone();

            let is_active = *active_item == item.id;

            let onclick = {
                let item = item.clone();
                let handle_navigation = handle_navigation.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    handle_navigation.emit(item.clone());
                })
            };

            let onkeydown = {
                let item = item.clone();
                let handle_navigation = handle_navigation.clone();
                Callback::from(move |e: KeyboardEvent| {
                    if e.key() == "Enter" || e.key() == " " {
                        e.prevent_default();
                        handle_navigation.emit(item.clone());
                    }
                })
            };

            html! {
                <li class={props.item_class} style={props.item_style} role="none">
                    <a
                        href={item.href.clone()}
                        class={if is_active { props.active_link_class } else { props.link_class }}
                        style={if is_active { props.active_link_style } else { props.link_style }}
                        role="menuitem"
                        tabindex="0"
                        aria-current={if is_active { Some("page") } else { None }}
                        onclick={onclick}
                        onkeydown={onkeydown}
                    >
                        { item.label.clone() }
                    </a>
                    { if index < navigation_items.len() - 1 {
                            html! {
                                <div
                                    class={props.separator_class}
                                    style={props.separator_style}
                                    role="presentation"
                                    aria-hidden="true"
                                />
                            }
                        } else {
                            html! {}
                        } }
                </li>
            }
        })
        .collect();

    html! {
        <nav
            class={props.container_class}
            style={props.container_style}
            role="navigation"
            aria-label="Main navigation"
        >
            <div id={live_region_id} class="sr-only" aria-live="polite" aria-atomic="true" />
            <ul class={props.list_class} style={props.list_style} role="menubar">
                { for nav_items }
            </ul>
        </nav>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeaderProps {
    #[prop_or("opensass.org")]
    pub website_url: &'static str,

    #[prop_or("header")]
    pub container_class: &'static str,

    #[prop_or(
        r#"
        position: relative;
        width: 100%;
        height: 120px;
    "#
    )]
    pub container_style: &'static str,

    #[prop_or("header-brand")]
    pub brand_class: &'static str,

    #[prop_or(
        r#"
        position: absolute;
        top: 1px;
        left: 154px;
        z-index: 88;
    "#
    )]
    pub brand_style: &'static str,

    #[prop_or("header-website")]
    pub website_class: &'static str,

    #[prop_or(
        r#"
        display: flex;
        width: 268px;
        height: 32px;
        justify-content: center;
        align-items: flex-start;
        font-size: 24px;
        font-weight: 400;
        line-height: 31.68px;
        color: #000000;
        letter-spacing: 1.44px;
        text-align: center;
        text-transform: uppercase;
        white-space: nowrap;
    "#
    )]
    pub website_style: &'static str,

    #[prop_or("header-content")]
    pub content_class: &'static str,

    #[prop_or(
        r#"
        position: relative;
        width: 100%;
        height: 100%;
    "#
    )]
    pub content_style: &'static str,

    #[prop_or("header-inner")]
    pub inner_class: &'static str,

    #[prop_or(
        r#"
        width: 1473px;
        height: 1075px;
        background-color: #ffffff;
        border-radius: 60px;
        border: 3px solid #000000;
        position: absolute;
        top: 32px;
        left: 36px;
    "#
    )]
    pub inner_style: &'static str,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    use_effect_with((), |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if document.get_element_by_id("header").is_none() {
                let style = document.create_element("style").unwrap();
                style.set_id("header");
                style.set_inner_html(include_str!("../css/pride/header.css"));
                document.head().unwrap().append_child(&style).unwrap();
            }
        }
        || ()
    });

    html! {
        <header class={props.container_class} style={props.container_style} role="banner">
            <div class={props.brand_class} style={props.brand_style}>
                <span class={props.website_class} style={props.website_style}>
                    { props.website_url }
                </span>
            </div>
            <div class={props.content_class} style={props.content_style}>
                <div class={props.inner_class} style={props.inner_style}>
                    <Navigation />
                    <LoginButton />
                </div>
                <BrandLogo />
            </div>
        </header>
    }
}

#[derive(Properties, PartialEq)]
pub struct TitleLetterProps {
    #[prop_or("title-letter title-letter--animated")]
    pub class: &'static str,

    #[prop_or_default]
    pub style: String,

    pub letter: &'static str,
}

#[derive(PartialEq, Clone)]
pub struct TitleLetter {
    pub letter: &'static str,
    pub color: &'static str,
    pub width: &'static str,
}

#[function_component(LetterSpan)]
fn letter_span(props: &TitleLetterProps) -> Html {
    html! {
        <span class={props.class} style={props.style.clone()} aria-hidden="true">
            { props.letter }
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct PrideTitleProps {
    #[prop_or("pride-title")]
    pub container_class: &'static str,

    #[prop_or(
        r#"
        position: absolute;
        top: 168.5px;
        left: 0;
        z-index: 42;
    "#
    )]
    pub container_style: &'static str,

    #[prop_or("title-row pride-row")]
    pub pride_row_class: &'static str,

    #[prop_or(
        r#"
        display: flex;
        align-items: flex-start;
        flex-wrap: nowrap;
        margin-bottom: 0;
        width: 570px;
        gap: 9px;
    "#
    )]
    pub pride_row_style: &'static str,

    #[prop_or("title-row month-row")]
    pub month_row_class: &'static str,

    #[prop_or(
        r#"
        display: flex;
        align-items: flex-start;
        flex-wrap: nowrap;
        margin-bottom: 0;
        width: 680px;
        gap: 7px;
        position: absolute;
        top: 0;
        left: 830px;
    "#
    )]
    pub month_row_style: &'static str,

    #[prop_or(
        r#"
        width: {width};
        text-shadow: 0 4px 0 {color};
        display: flex;
        height: 236px;
        justify-content: center;
        align-items: flex-start;
        flex-shrink: 0;
        flex-basis: auto;
        font-size: 140px;
        font-weight: 400;
        line-height: 236px;
        letter-spacing: 8.4px;
        text-align: center;
        text-transform: uppercase;
        white-space: nowrap;
        -webkit-text-stroke: 3px #000000;
    "#
    )]
    pub letter_style_template: &'static str,
}

#[function_component(PrideTitle)]
pub fn pride_title(props: &PrideTitleProps) -> Html {
    use_effect_with((), |_| {
        if let Some(document) = window().and_then(|w| w.document()) {
            if document.get_element_by_id("pride-title").is_none() {
                let style: HtmlStyleElement = document
                    .create_element("style")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                style.set_id("pride-title");
                style.set_inner_text(include_str!("../css/pride/pride-title.css"));
                document.head().unwrap().append_child(&style).unwrap();
            }
        }
    });

    let pride_letters = vec![
        TitleLetter {
            letter: "P",
            color: "#f64e4e",
            width: "120px",
        },
        TitleLetter {
            letter: "R",
            color: "#ffbd11",
            width: "132px",
        },
        TitleLetter {
            letter: "i",
            color: "#fff34a",
            width: "60px",
        },
        TitleLetter {
            letter: "D",
            color: "#0fab3d",
            width: "124px",
        },
        TitleLetter {
            letter: "E",
            color: "#0e6aff",
            width: "98px",
        },
    ];

    let month_letters = vec![
        TitleLetter {
            letter: "M",
            color: "#9c2dad",
            width: "155px",
        },
        TitleLetter {
            letter: "o",
            color: "#ffbd11",
            width: "133px",
        },
        TitleLetter {
            letter: "N",
            color: "#fff34a",
            width: "127px",
        },
        TitleLetter {
            letter: "T",
            color: "#0fab3d",
            width: "112px",
        },
        TitleLetter {
            letter: "H",
            color: "#0e6aff",
            width: "125px",
        },
    ];

    html! {
        <div
            class={props.container_class}
            style={props.container_style}
            role="img"
            aria-label="Pride Month in colorful letters"
        >
            <div class={props.pride_row_class} style={props.pride_row_style}>
                { pride_letters.iter().enumerate().map(|(i, l)| html! {
                    <LetterSpan
                        key={i.to_string()}
                        letter={l.letter}
                        style={props.letter_style_template
                            .replace("{width}", l.width)
                            .replace("{color}", l.color)
                        }
                    />
                }).collect::<Html>() }
            </div>
            <div class={props.month_row_class} style={props.month_row_style}>
                { month_letters.iter().enumerate().map(|(i, l)| html! {
                    <LetterSpan
                        key={i.to_string()}
                        letter={l.letter}
                        style={props.letter_style_template
                            .replace("{width}", l.width)
                            .replace("{color}", l.color)
                        }
                    />
                }).collect::<Html>() }
            </div>
        </div>
    }
}

fn inject_styles() {
    if let Some(win) = window() {
        if let Some(doc) = win.document() {
            if doc.get_element_by_id("newsletter").is_none() {
                let style = doc
                    .create_element("style")
                    .unwrap()
                    .dyn_into::<HtmlStyleElement>()
                    .unwrap();
                style.set_id("newsletter");
                style.set_inner_text(include_str!("../css/pride/newsletter.css"));
                doc.head().unwrap().append_child(&style).unwrap();
            }
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct NewsletterSignupProps {
    #[prop_or("June 25, 2025".into())]
    pub event_date: String,
    #[prop_or("2:00 PM - 8:00 PM".into())]
    pub event_time: String,
    #[prop_or("Ferris Cosmos Dev Community".into())]
    pub location: String,
    #[prop_or(100_000)]
    pub expected_attendees: u32,

    #[prop_or("position: relative; top: -375px; height: auto; padding: 20px;")]
    pub section_style: &'static str,
    #[prop_or(
        "position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border: 0;"
    )]
    pub sr_only_style: &'static str,
    #[prop_or(
        "position: relative;
        width: 100%;
        max-width: 600px;
        height: auto;
        margin: 0 auto;
        border-radius: 20px;"
    )]
    pub signup_container_style: &'static str,
    #[prop_or(
        "position: absolute;
        width: 100%;
        height: 100%;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 20px;"
    )]
    pub signup_overlay_style: &'static str,
    #[prop_or(
        "position: relative;
        top: 50px;
        left: 0;
        width: 90%;
        margin: 0 auto;"
    )]
    pub signup_content_style: &'static str,
    #[prop_or(
        "justify-content: center;
        display: flex;
        gap: 8px;
        align-items: center;
        min-height: 48px;"
    )]
    pub flags_container_style: &'static str,
    #[prop_or(
        "display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
        position: relative;
        z-index: 10;"
    )]
    pub event_grid_style: &'static str,
    #[prop_or(
        "background: rgba(255,255,255,0.95);
        border: 2px solid #000;
        border-radius: var(--radius-xl);
        padding: 1.5rem;
        text-align: center;
        transition: all 0.3s;
        backdrop-filter: blur(10px);"
    )]
    pub card_style: &'static str,
    #[prop_or(
        "display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 1rem;"
    )]
    pub card_header_style: &'static str,
    #[prop_or(
        "font-size: 2rem;
        #7bcef2;
        transition: all 0.3s;"
    )]
    pub card_icon_style: &'static str,
    #[prop_or(
        "font-size: clamp(0.9rem, 2vw, 1.1rem);
        font-weight: 600;
        color: #000;
        margin: 0;
        text-transform: uppercase;
        letter-spacing: 0.05em;"
    )]
    pub card_title_style: &'static str,
    #[prop_or("color: var(--text-dark);")]
    pub card_content_style: &'static str,
    #[prop_or(
        "display: block;
        font-size: clamp(0.9rem, 2vw, 1rem);
        font-weight: 500;
        margin-bottom: 0.25rem;"
    )]
    pub time_style: &'static str,
    #[prop_or("font-size: clamp(0.8rem, 1.8vw, 0.9rem); opacity: 0.8;")]
    pub time_secondary_style: &'static str,
    #[prop_or(
        "font-style: normal;
        font-size: clamp(0.9rem, 2vw, 1rem);
        font-weight: 500;
        line-height: 1.4;"
    )]
    pub location_style: &'static str,
    #[prop_or(
        "font-size: clamp(0.9rem, 2vw, 1rem);
        font-weight: 600;
        color: var(--pride-green);"
    )]
    pub attendee_count_style: &'static str,
    #[prop_or(
        "position: absolute;
        width: 100%;
        height: 100%;"
    )]
    pub decorations_container_style: &'static str,
    #[prop_or(
        "position: absolute; width: 23px; height: 23px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/wbg35vijy00ayf3f6lc9.png\");"
    )]
    pub decoration1_style: &'static str,
    #[prop_or(
        "position: absolute; width: 20px; height: 20px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/wbg35vijy00ayf3f6lc9.png\");"
    )]
    pub decoration2_style: &'static str,
    #[prop_or(
        "position: absolute; width: 16px; height: 16px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/wbg35vijy00ayf3f6lc9.png\");"
    )]
    pub decoration3_style: &'static str,
    #[prop_or(
        "position: absolute; width: 23px; height: 23px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/wbg35vijy00ayf3f6lc9.png\");"
    )]
    pub decoration4_style: &'static str,
    #[prop_or(
        "position: absolute; width: 15px; height: 15px; background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/wbg35vijy00ayf3f6lc9.png\");"
    )]
    pub decoration5_style: &'static str,
}

#[function_component(NewsletterSignup)]
pub fn newsletter_signup(props: &NewsletterSignupProps) -> Html {
    inject_styles();

    let email = use_state(|| "".to_string());
    let is_submitted = use_state(|| false);
    let error = use_state(|| "".to_string());

    let on_submit = {
        let email = email.clone();
        let is_submitted = is_submitted.clone();
        let error = error.clone();
        Callback::from(move |value: String| {
            if !value.contains('@') {
                error.set("Please enter a valid email address".into());
                return;
            }
            email.set(value.clone());
            is_submitted.set(true);
            error.set("".into());
            if let Some(doc) = window().and_then(|w| w.document()) {
                if let Some(el) = doc.get_element_by_id("signup-announcements") {
                    el.set_text_content(Some(&format!(
                        "Successfully subscribed with email {}",
                        value
                    )));
                }
            }
        })
    };

    html! {
        <section style={props.section_style} aria-labelledby="newsletter-title">
            <h2 id="newsletter-title" style={props.sr_only_style}>
                { "Newsletter Signup and Social Sharing" }
            </h2>
            <div
                id="signup-announcements"
                aria-live="polite"
                aria-atomic="true"
                style={props.sr_only_style}
            />
            <div class="signup-container" style={props.signup_container_style}>
                <div style={props.signup_overlay_style} aria-hidden="true" />
                <div class="signup-content" style={props.signup_content_style}>
                    <CalendarButton />
                    <FlagSection
                        id="flags"
                        title=""
                        container_style={props.flags_container_style}
                        flags={vec![
                            Type::Rainbow, Type::Transgender, Type::Bisexual, Type::Lesbian,
                            Type::Pansexual, Type::Asexual, Type::NonBinary, Type::Aromantic,
                            Type::Demisexual, Type::Genderfluid, Type::Agender, Type::Polysexual,
                            Type::Omnisexual, Type::Demiromantic, Type::Graysexual,
                        ]}
                    />
                    <div class="event-info-grid" style={props.event_grid_style}>
                        <article
                            role="region"
                            aria-labelledby="datetime-title"
                            style={props.card_style}
                        >
                            <header class="card-header" style={props.card_header_style}>
                                <span class="card-icon" style={props.card_icon_style}>
                                    <i class="fa-solid fa-clock" aria-hidden="true" />
                                </span>
                                <h3
                                    id="datetime-title"
                                    class="card-title"
                                    style={props.card_title_style}
                                >
                                    { "When" }
                                </h3>
                            </header>
                            <div class="card-content" style={props.card_content_style}>
                                <time datetime="2025-06-25T14:00" style={props.time_style}>
                                    { &props.event_date }
                                </time>
                                <div style={props.time_secondary_style}>{ &props.event_time }</div>
                            </div>
                        </article>
                        <article
                            role="region"
                            aria-labelledby="location-title"
                            style={props.card_style}
                        >
                            <header class="card-header" style={props.card_header_style}>
                                <span class="card-icon" style={props.card_icon_style}>
                                    <i class="fa-solid fa-location-dot" aria-hidden="true" />
                                </span>
                                <h3
                                    id="location-title"
                                    class="card-title"
                                    style={props.card_title_style}
                                >
                                    { "Where" }
                                </h3>
                            </header>
                            <div class="card-content" style={props.card_content_style}>
                                <address class="event-location" style={props.location_style}>
                                    { &props.location }
                                </address>
                            </div>
                        </article>
                        <article
                            role="region"
                            aria-labelledby="attendees-title"
                            style={props.card_style}
                        >
                            <header class="card-header" style={props.card_header_style}>
                                <span class="card-icon" style={props.card_icon_style}>
                                    <i class="fa-solid fa-users" aria-hidden="true" />
                                </span>
                                <h3
                                    id="attendees-title"
                                    class="card-title"
                                    style={props.card_title_style}
                                >
                                    { "Expected" }
                                </h3>
                            </header>
                            <div class="card-content" style={props.card_content_style}>
                                <div class="attendee-count" style={props.attendee_count_style}>
                                    { format!("{}+ people", props.expected_attendees) }
                                </div>
                            </div>
                        </article>
                    </div>
                    <EmailInput on_submit={on_submit} />
                </div>
                <div
                    class="hero-decorations"
                    style={props.decorations_container_style}
                    aria-hidden="true"
                >
                    <div style={props.decoration1_style} />
                    <div style={props.decoration2_style} />
                    <div style={props.decoration3_style} />
                    <div style={props.decoration4_style} />
                    <div style={props.decoration5_style} />
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct HeroSectionProps {
    #[prop_or("hero-section")]
    pub section_class: &'static str,
    #[prop_or(
        "position: relative;
        width: 100%;
        height: 500px;
        margin-top: 100px;"
    )]
    pub section_style: &'static str,

    #[prop_or("hero-decorations")]
    pub decorations_class: &'static str,
    #[prop_or(
        "position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        pointer-events: none;"
    )]
    pub decorations_style: &'static str,

    #[prop_or("decoration-element decoration-2")]
    pub decoration2_class: &'static str,
    #[prop_or(
        "position: absolute;
        width: 1.87%;
        height: 3.83%;
        top: 8.93%;
        left: 5.84%;
        background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/qb8v8jta0adpctg1bgdr.png\");
        background-size: 100% 100%;
        background-repeat: no-repeat;
        background-position: center;
        z-index: 18;
        animation: float 4s ease-in-out infinite 0.5s;"
    )]
    pub decoration2_style: &'static str,

    #[prop_or("decoration-element decoration-3")]
    pub decoration3_class: &'static str,
    #[prop_or(
        "position: absolute;
        width: 3.61%;
        height: 7.4%;
        top: 23.23%;
        left: 0;
        background-image: url(\"https://dev-to-uploads.s3.amazonaws.com/uploads/articles/qb8v8jta0adpctg1bgdr.png\");
        background-size: 100% 100%;
        background-repeat: no-repeat;
        background-position: center;
        z-index: 21;
        animation: float 5s ease-in-out infinite 1s;"
    )]
    pub decoration3_style: &'static str,

    #[prop_or("sr-only")]
    pub sr_only_class: &'static str,
    #[prop_or(
        "position: absolute;
        width: 1px;
        height: 1px;
        padding: 0;
        margin: -1px;
        overflow: hidden;
        clip: rect(0, 0, 0, 0);
        white-space: nowrap;
        border: 0;"
    )]
    pub sr_only_style: &'static str,
}

#[function_component(HeroSection)]
pub fn hero_section(props: &HeroSectionProps) -> Html {
    use_effect_with((), |_| {
        if let Some(document) = window().and_then(|w| w.document()) {
            if document.get_element_by_id("hero-section").is_none() {
                let style: HtmlStyleElement = document
                    .create_element("style")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                style.set_id("hero-section");
                style.set_inner_text(include_str!("../css/pride/hero-section.css"));
                document.head().unwrap().append_child(&style).unwrap();
            }
        }
    });

    html! {
        <section
            class={props.section_class}
            style={props.section_style}
            aria-labelledby="hero-title"
        >
            <div
                class={props.decorations_class}
                style={props.decorations_style}
                aria-hidden="true"
            >
                <div class={props.decoration2_class} style={props.decoration2_style} />
                <div class={props.decoration3_class} style={props.decoration3_style} />
            </div>
            <h1 id="hero-title" class={props.sr_only_class} style={props.sr_only_style}>
                { "Pride Month Celebration Event" }
            </h1>
            <PrideTitle />
        </section>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PridePageProps {
    #[prop_or("pride-page")]
    pub container_class: &'static str,
    #[prop_or(
        "width: 100%;
        min-height: 100vh;
        position: relative;
        overflow-x: auto;"
    )]
    pub container_style: &'static str,

    #[prop_or("page-container")]
    pub page_container_class: &'static str,
    #[prop_or(
        "width: 1600px;
        height: 1200px;
        position: relative;
        overflow: hidden;
        margin: 0 auto;"
    )]
    pub page_container_style: &'static str,

    #[prop_or("content-wrapper")]
    pub content_wrapper_class: &'static str,
    #[prop_or(
        "width: 1544px;
        height: 1149px;
        background-color: #7bcef2;
        border-radius: 88px;
        border: 3px solid #000000;
        position: absolute;
        top: 30px;
        left: 50%;
        transform: translate(-49.94%, 0);"
    )]
    pub content_wrapper_style: &'static str,

    #[prop_or("main-content")]
    pub main_content_class: &'static str,
    #[prop_or(
        "width: 1473px;
        height: 1075px;
        border-radius: 76px;
        border: 3px solid #000000;
        position: absolute;
        top: 35px;
        left: 50%;
        transform: translate(-50.1%, 0);"
    )]
    pub main_content_style: &'static str,

    #[prop_or("skip-link")]
    pub skip_link_class: &'static str,
    #[prop_or(
        "position: absolute;
        top: 0;
        left: 0;
        padding: 8px;"
    )]
    pub skip_link_style: &'static str,
}

#[function_component(Hero)]
pub fn hero(props: &PridePageProps) -> Html {
    use_effect_with((), move |_| {
        if let Some(doc) = window().and_then(|w| w.document()) {
            if doc.get_element_by_id("pride-page").is_none() {
                let style: HtmlStyleElement =
                    doc.create_element("style").unwrap().dyn_into().unwrap();

                style.set_id("pride-page");
                style.set_inner_text(include_str!("../css/pride/pride-page.css"));

                doc.head().unwrap().append_child(&style).unwrap();
            }
        }
    });

    html! {
        <div class={props.container_class} style={props.container_style}>
            <div
                class={props.page_container_class}
                style={props.page_container_style}
                role="main"
                id="main-content"
            >
                <div class={props.content_wrapper_class} style={props.content_wrapper_style}>
                    <Header />
                    <main class={props.main_content_class} style={props.main_content_style}>
                        <HeroSection />
                        <EventDetails />
                        <NewsletterSignup />
                    </main>
                </div>
            </div>
        </div>
    }
}
