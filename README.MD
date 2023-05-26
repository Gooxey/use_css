# use_css

Instead of using the `use_css!` macro, one can also just use the [stylist](https://crates.io/crates/stylist) crate. But the problem you will face is that VSCode won't be able to provide its full feature set regarding CSS code. Therefore, the best solution is to use CSS files. The only question now is: How do you use CSS code inside your [Yew](https://yew.rs) project? Well, you need to generate Rust code based on these files. And to do this, you can use this macro.

## Disclaimer

This library is currently not under active development, but features will be added once they are needed in my other projects.

## Requirements

To use the CSS code in your [Yew](https://yew.rs) project, you will need to add [stylist](https://crates.io/crates/stylist) as a dependency:

```bash
cargo add stylist
```

## Example

Let's say this is your project structure:

```text
|
| -frontend
|  | -src
|  |  | -title_bar
|  |  |  | -mod.rs
|  |  |  | -style.css // This file has to be call like this
|  |  | -main.rs
|  | -Cargo.toml
|  | -Cargo.lock
|  | -index.html
```

With `style.css` looking like:

```css
titlebar { /* The name of titlebar should be unique to this file and not be surrounded by anything. */
    display: flex;

    height: 10px;
    width: 10px;
}
```

To use this style in `mod.rs`, you can write something like:

```rust
use yew::prelude::*;
use use_css::use_css;

use_css!("title_bar"); // The string given should be the path, starting from `scr/`, to the folder where the desired `style.css` can be found.

#[function_component]
pub fn TitleBar() -> Html {
    html! {
        <>
            <div class={style::titlebar()}>
                <span>{"Hello world"}</span>
            </div>
        </>
    }
}
```

If one decided to write the generated code manually, it would look like this:

```rust
use yew::prelude::*;

mod style {
    use stylist::{
        css,
        StyleSource,
    };

    pub fn titlebar() -> StyleSource {
        css!("
            display: flex;

            height: 10px;
            width: 10px;
        ")
    }
}

// Implementation of the TitleBar component...
```
