# BOREDSPLIT
[![Build](https://github.com/clowzed/boredsplit/actions/workflows/rust.yml/badge.svg)](https://github.com/clowzed/boredsplit/actions/workflows/rust.yml)
## See this. It's ugly!
```css
body {
    margin: 25px;
    background-color: rgb(240, 240, 240);
    font-family: arial, sans-serif;
    font-size: 14px;
}
```
#### Much better
```css
body {
    margin            :  25px;
    background-color  :  rgb(240, 240, 240);
    font-family       :  arial, sans-serif;
    font-size         :  14px;
}
```

## Installation
1) Clone repository
2) Run `cargo build --release`



## Params
| short | long       | default | description                          |
|-------|------------|---------|--------------------------------------|
| -f    | --file     | `*.css` | Sets glob pattern for file searching |
| -i    | --ident    | `4`     | Spaces before left part              |
| -l    | --lmargin | `2`     | Left spaces near splitter            |
| -r    | --rmargin | `2`     | Right spaces near splitter           |
