# [asterism.sh](https://asterism.sh/)

my personal website, featuring Unix terminal-style navigation. written in Rust via [Yew](https://yew.rs/) and WebAssembly!

## features

ordered by priority/likelihood of implementation:

- [x] personal introduction
- [x] Unix terminal-like navigation
- [x] "executable" files
- [ ] better 404 fallback
- [ ] personal project documentation
- [ ] mutable filesystem
    - [ ] persistent filesystem via localStorage
- [ ] text editor
- [ ] JS bindings for "standard library," accessible through user programs

## development

1. install tailwindcss:

`$ npm install tailwindcss @tailwindcss/cli`

2. install rust nightly:

`$ rustup toolchain install nightly`

3. install the Rust WebAssembly target:

`$ rustup target add wasm32-unknown-unknown`

4. install Trunk:

`$ cargo install --locked trunk`

5. start a development server:

`$ trunk serve`
