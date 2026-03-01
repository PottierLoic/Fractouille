# Fractouille

![](example.png)

Fractouille is a simple fractal explorer running in your terminal.

[![Rust](https://img.shields.io/badge/rust-1.86.0-orange.svg)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- Multiple fractal sets (Mandelbrot, Julia, Burning Ship)
- High-quality screenshot capture with smooth coloring
- Zoom videos recording
- Interactive navigation
- Command mode for advanced usage

## Requirements

- Rust
- ffmpeg (for video recording)

## Usage

```bash
# clone the repository
git clone https://github.com/pottierloic/fractouille
cd fractouille

# run directly
cargo run

# or install and run from anywhere
cargo install --path .
fractouille
```

```bash
# sixel mode to have pure terminal graphics
fractouille --sixel
```

```bash
# original interactive explorer (manual controls)
fractouille
```

```bash
# infinite animate mode (Ctrl+C to stop)
fractouille --animate
```

```bash
# faster animate mode
fractouille --animate --animate-fps 24 --animate-zoom-factor 1.03
```

## Keybinds

- `wasd`: Move around
- `r/f`: adjust max iterations
- `-/+`: adjust zoom level
- `p`: toggle animate pause/resume
- `space`: cycle color palette
- `enter`: switch fractal set
- `q`: quit fractouille

## Command mode

Just like in Vim, press `:` to enter command mode. 
A list of all available commands can be found by in the `COMMANDS.md` file.

# Screenshots or records

Screenshots and records are automatically saved to your system's Pictures folder inside of `fractouille`.
Recording need `ffmpeg` to be installed on your system, more information can be found in the `RECORD.md` file.

## Roadmap

- [x] Basic fractal sets implementation
    - [x] Mandelbrot Set
    - [x] Julia Set
    - [x] Burning Ship
- [x] Screenshot functionality
    - [x] Smooth coloring
    - [x] Auto-save
- [x] Variable power parameter

- [ ] Border tracing method
- [ ] Arbitray precision
- [x] Customizable color palette

- [x] Phoenix Set implementation
- [ ] Newton fractals
- [ ] Lyapunov fractals

## Museum

Some really cool pictures I took can be seen in the `museum` folder.

![](museum/mandelbrot_1765719606_x-1.0112261337344692_y-0.3141187591728309_z84280.97165257359_p2.png)
![](museum/mandelbrot_1765719666_x-1.2112520476343478_y-0.318445881949524_z305844346.7923433_p2.png)
![](museum/mandelbrot_1765719547_x0.4147993498579829_y-0.14790285832920558_z1787453723235.0679_p2.png)
