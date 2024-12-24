# the cloneable_errors crate

this is basically the core functionality of anyhow, but written from scratch, in safe rust.

this library was previously an internal `error_handling` crate in [DeArrow Browser](https://github.com/mini-bomba/DeArrowBrowser)
and was licensed together with the entire project under AGPL.
it was spun off into it's own public crate and is now licensed under MIT.

## why?
anyhow errors are not cloneable - caching them is annoying, especially if you want to add more context later on

this crate aims to be a simpler, hopefully less annoying alternative if you do the things I do
- errors use Arc<> internally
- there's a serializable variant that drops a lot of data, but makes it easy to send error info between workers or over the wire

## deps
- no required dependencies
- serde optionally required for serializing the serializable error variant (enable the `serde` feature)
- anyhow optionally required for turning anyhow errors into serializable errors (enable the `anyhow` feature)
