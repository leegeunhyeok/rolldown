#![allow(clippy::ignore_without_reason)]
#![allow(clippy::large_futures)]

mod esbuild;
mod rolldown_fixture;
#[path = "../rolldown/mod.rs"]
mod rolldown_tests;
mod rollup;
mod test262;
