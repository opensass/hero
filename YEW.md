# Hero Yew Usage

Adding Hero to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Hero component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add hero --features=yew
   ```

1. Import a `Hero` component into your Yew component and start using it in your app.

## 🛠️ Usage

At the current release, `hero` comes with 4 built-in, highly customizable Hero sections designed for various layout needs and visual styles. Each Hero variant is modular, accessible, and easily styled through a comprehensive set of props. Follow these steps to integrate a `Hero` component into your Yew application:

### Available Components

```rust
use hero::yew::hero1::Hero as Hero1;
use hero::yew::hero2::Hero as Hero2;
use hero::yew::hero3::Hero as Hero3;
use hero::yew::hero4::Hero as Hero4;
```

### Basic Usage Example (`hero1`)

```rust
use yew::prelude::*;
use hero::yew::hero1::Hero;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Hero
            heading="Build Fast in Rust"
            description="Drop-in hero sections for Yew, Leptos, and Dioxus."
            title_style="font-size: 3rem; font-weight: bold; color: #4F46E5;"
            description_style="font-size: 1.25rem; color: #6B7280;"
            cta_style="padding: 0.75rem 1.5rem; background-color: #4F46E5; color: white; border-radius: 0.5rem;"
        />
    }
}
```

### Styling with Tailwind (or CSS Classes)

Each component exposes full control over styles and class names. For example, with `hero1`:

```rust
use yew::prelude::*;
use hero::yew::hero1::Hero;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Hero
            heading="Launch Ultra-Fast Apps"
            description="Style your hero section with Tailwind, inline styles, or any CSS framework."
            container_class="max-w-6xl mx-auto px-4 py-24"
            title_class="text-5xl font-extrabold text-center text-white"
            description_class="mt-4 text-xl text-center text-gray-300"
            cta_class="mt-6 bg-white text-black px-6 py-3 rounded-full shadow-lg hover:bg-gray-100"
        />
    }
}
```

### Customization Options

Each `Hero` component supports:

- **Content props**: `heading`, `description`, `tabs`, `cta_style`, etc.
- **Style props**: `title_style`, `description_style`, `container_style`, etc.
- **Class props**: `container_class`, `title_class`, etc.
- **Semantic options**: `heading_tag`, `aria_label`, etc.

You can combine inline styles and utility classes for full control over design.

## 💡 Notes

- **Framework-Agnostic**: Can be used in Yew, Leptos, or Dioxus.
- **Accessible**: Semantic HTML and keyboard navigable by default.
- **Dark Mode Friendly**: Works well with Tailwind's dark variants or your custom themes.
- **Completely Customizable**: Override layout, inject your own components, or style everything.
