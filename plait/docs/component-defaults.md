## Author-declared defaults (0.9)

Declare a default with `prop: Type = expression`. Props without an `=` remain
required, including `Option<T>` props and types that implement `Default`.
Callers keep the ordinary template syntax; construction helpers are internal.

```rust
use plait::{component, html, ToHtml};

component! {
    pub fn Button(kind: &str = "button", disabled: bool = false) {
        button(type: kind, disabled?: disabled, #attrs) { #children }
    }
}

assert_eq!(html! { @Button { "Cancel" } }.to_html(),
    "<button type=\"button\">Cancel</button>");
assert_eq!(html! { @Button(kind: "submit"; id: "save") { "Save" } }.to_html(),
    "<button type=\"submit\" id=\"save\">Save</button>");
assert_eq!(html! { @Button(disabled: true) {} }.to_html(),
    "<button type=\"button\" disabled></button>");
```

Each omitted default is evaluated once when that component invocation renders,
not when its lazy `html!` fragment is created. Rendering the fragment again
reevaluates its defaults. Supplying a prop prevents evaluation of its default;
explicit `None` overrides a default such as `Some("help")`.

Defaults are expressions in the component's declaration scope, type-checked
against the declared prop type. They do not refer to other props or the caller's
local variables. They need not be constants. A generic default must be valid for
every permitted instantiation: for example, `value: T = T::default()` with an
explicit `T: Default` bound. This does not impose `Default` on other prop types.
An anonymous `impl Trait` is still a generic input, not automatic conversion or
inference of a concrete type from a default expression.

### Required inputs and borrowed optional text

```rust
use plait::{component, html, ToHtml};

component! {
    fn Field(id: &str, value: &str = "", error: Option<&str> = None) {
        input(id: id, value: value, #attrs);
        if let Some(error) = error { p { (error) } }
    }
}

let value_owner = String::from("<invalid>");
let error_owner = Some(String::from("Try again"));
let value = value_owner.as_str();
let error = error_owner.as_deref();
let page = html! { @Field(id: "email", value, error; required) {} };
assert_eq!(page.to_html(),
    "<input id=\"email\" value=\"&lt;invalid&gt;\" required><p>Try again</p>");
```

No cloning, allocation for prop conversion, or caller generic annotation is
needed. `html!` retains its existing move-closure capture behavior: borrow owners
before constructing a template when they must remain reusable, especially for
nested fragments. These borrows must outlive the fragment that uses them.

Missing required props fail at compile time with the prop and component names:

```compile_fail
use plait::{component, html};
component! { fn Field(id: &str, error: Option<&str> = None) { (id) (error) } }
let _ = html! { @Field(error: None) {} }; // missing required prop `id`
```

A prop whose type is `Option` is still required unless its author declares a default:

```compile_fail
use plait::{component, html};
component! { fn Field(error: Option<&str>) { (error) } }
let _ = html! { @Field {} }; // missing required prop `error`
```

Unknown props, duplicate props (including shorthand duplicates), mismatched
prop types, and mismatched default expression types also fail compilation.

### Optional fragment regions without inference boilerplate

For independently omissible borrowed fragments, use the concrete type
`Option<&dyn PartialHtml> = None`. Required fragments may continue using
`impl PartialHtml`. This uses borrowed trait objects, not boxing or allocation.

```rust
use plait::{component, html, PartialHtml, ToHtml};

component! {
    fn Panel(
        title: impl PartialHtml,
        description: Option<&dyn PartialHtml> = None,
        actions: Option<&dyn PartialHtml> = None,
    ) {
        section(#attrs) {
            h2 { (title) }
            if let Some(description) = description { div { (description) } }
            main { #children }
            if let Some(actions) = actions { footer { (actions) } }
        }
    }
}

assert_eq!(html! { @Panel(title: html! { "Edit" }) { "Body" } }.to_html(),
    "<section><h2>Edit</h2><main>Body</main></section>");

let help_owner = String::from("Help");
let help = help_owner.as_str();
let description = html! { p { (help) } };
let actions = html! { button(type: "submit") { "Save" } };
let page = html! {
    @Panel(title: html! { strong { "Edit" } },
        description: Some(&description), actions: Some(&actions)) { "Body" }
};
assert_eq!(page.to_html(),
    "<section><h2><strong>Edit</strong></h2><div><p>Help</p></div><main>Body</main><footer><button type=\"submit\">Save</button></footer></section>");
```

Either optional prop, or both, may be omitted. Explicit `None` and temporary
fragment borrows such as `Some(&html! { p { "Help" } })` also work. Callers do
not need empty fragments, typed `None`, or generic arguments. Authors remain
responsible for rendering optional nodes and accessible associations only when
those regions exist; the macro does not coordinate UI semantics.

### Compatibility and handwritten component migration

This is a **breaking 0.9 construction change**, not a 0.8 patch. Existing
`component!`-generated calls keep their syntax, shorthand, props, children,
attribute forwarding, escaping, and generic/lifetime support. Rebuild component
libraries against the same Plait 0.9 line; components generated by 0.8 macros do
not provide the new construction protocol. The runtime pins its matching macro
version. Qualified imports and component re-exports remain supported.

The generated component struct still has public fields. Direct struct literals
still require all fields and do not apply macro defaults. The `Component` trait
and `render_component` method are unchanged, so handwritten implementations
remain supported for direct rendering. However, `@Handwritten(...)` no longer
constructs arbitrary structs: `@` calls use the protocol generated by
`component!`. Do not implement or call the hidden `__plait_*` helpers yourself;
those are not a supported public builder API.

To retain `@` syntax, express the component using `component!`, for example:

```rust
use plait::{component, html, ToHtml};

component! {
    pub fn CustomButton(text: &str, disabled: bool = false) {
        button(disabled?: disabled, #attrs) { (text) #children }
    }
}
assert_eq!(html! { @CustomButton(text: "Save"; data_test: "custom") { "!" } }.to_html(),
    "<button data-test=\"custom\">Save!</button>");
```

For renderers that must remain handwritten, continue constructing the struct
and invoking `Component::render_component`, providing the attribute and children
writer closures explicitly. This preserves that escape hatch without depending
on hidden builder internals. `tests/component_migration_tests.rs` verifies direct
rendering and the equivalent macro-defined component, including escaping and
attribute/children destinations.

### Verification

From the Plait repository root:

```sh
cargo test --locked --workspace --no-default-features
cargo test --locked --workspace --all-features
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
```

`component_defaults_tests` covers compile-pass/render contracts P01–P10 and
additional lazy/default-expression, raw-prop, and generic cases.
`component_compile_tests` checks required-prop diagnostics F01–F08 and invalid
calls using pinned trybuild, plus a separate components/consumer workspace to
verify cross-crate imports and re-exports. Its subprocess uses a separate target
directory to avoid Cargo lock contention.

The recorded diagnostic baseline is Rust 1.98.1 / Cargo 1.98.1 with trybuild
1.0.116. Review diagnostic changes before updating `.stderr` fixtures on another
compiler; a compiler error unrelated to the missing prop is not a passing
contract test. These are unstyled macro fixtures, not a UI kit or browser tests.
