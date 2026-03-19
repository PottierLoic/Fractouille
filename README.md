# Fractouille

![](example.png)

A fractal exploration and rendering engine written in Rust.  
Navigate Mandelbrot, Julia, Burning Ship and Phoenix sets in real-time and export high-resolution screenshots and zoom videos.  

[![Rust](https://img.shields.io/badge/rust-1.86.0-orange.svg)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Getting started

```bash
git clone https://github.com/pottierloic/fractouille
cd fractouille
cargo run
```

Sixel mode for true pixel rendering in compatible terminals:

```bash
fractouille --sixel
```

`ffmpeg` is required for video recording, see [`RECORD.md`](RECORD.md) for details.

## Usage

Press `:` to enter command mode (Vim-style). Full command reference in [`COMMANDS.md`](COMMANDS.md).

| Key | Action |
|-----|--------|
| `wasd` | Move |
| `-` / `+` | Zoom |
| `r` / `f` | Adjust iterations |
| `space` | Cycle palette |
| `enter` | Switch fractal set |
| `q` | Quit |

Screenshots and records are automatically saved to your system's Pictures folder. See [`RECORD.md`](RECORD.md).

## Roadmap & Contributing

Fractouille aims to evolve into a multi-frontend fractal engine with GPU support, arbitrary precision rendering and multiple CPU render backends.  
See [`ROADMAP.md`](ROADMAP.md) for the full picture.

Contributions are welcome, there is a lot of ground to cover.

## Museum

Some really cool pictures I took can be seen in the `museum` folder.

![](museum/mandelbrot_1765719606_x-1.0112261337344692_y-0.3141187591728309_z84280.97165257359_p2.png)
![](museum/mandelbrot_1765719666_x-1.2112520476343478_y-0.318445881949524_z305844346.7923433_p2.png)
![](museum/mandelbrot_1765719547_x0.4147993498579829_y-0.14790285832920558_z1787453723235.0679_p2.png)
