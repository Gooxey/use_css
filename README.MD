# use_css

Instead of using the `use_css!` macro, one can also just use the [stylist](https://crates.io/crates/stylist) crate. But the problem you will face is that VSCode won't be able to provide its full feature set regarding CSS code. Therefore, the best solution is to use CSS files. The only question now is: How do you use CSS code inside your [Yew](https://yew.rs) project? Well, you need to generate Rust code based on these files. And to do this, you can use this macro.

## Example

Let's say this is your project structure:

```text
│
├── frontend
│  ├── src
│  │  ├── title_bar
│  │  │  ├── mod.rs
│  │  │  └── style.css // This file has to be call like this
│  │  └── main.rs
│  ├── Cargo.toml
│  ├── Cargo.lock
│  └── index.html
```

With `style.css` looking like:

```css
/* The name of titlebar should be unique to this file */
titlebar {
    display: flex;

    height: 10px;
    width: 10px;
}

/* This macro also accepts `.` and `#` before the name */
/* And even type errors like shown below */

.global{
    --background-color: #000000;
    --font-color: #FFFFFF;
}
/* And we can use nested styles */
#global_document     {
    html, body {
        background-color: var(--background-color);
        color: var(--font-color);
    }
}
```

To use this style in `mod.rs`, you can write something like:

```rust
use yew::prelude::*;
use use_css::use_css;

// The string given should be the path, starting from `scr/`, to the
// folder where the desired `style.css` can be found.
use_css!("title_bar");

// here we first apply our global styles
// and then we apply styles for actual elements
#[function_component]
pub fn TitleBar() -> Html {
    html! {
        <>
            <Global css={style::global()}/>
            <Global css={style::global_document()}/>

            <div class={style::titlebar()}>
                <span>{"Hello world"}</span>
            </div>
        </>
    }
}
```
