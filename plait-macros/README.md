# plait-macros

Procedural macros for the `plait` HTML templating library.

This crate provides three macros for generating HTML:

- `html!` - Creates an `Html` value from a template
- `render!` - Renders content to an existing `HtmlFormatter`
- `attrs!` - Creates an `Attributes` collection

These macros are re-exported by the main `plait` crate and should typically be used from there rather than
directly from this crate.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../LICENSE-MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
