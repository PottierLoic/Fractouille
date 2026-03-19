## Architecture

The project is being restructured into four crates:

- **`fractouille-core`** - pure computation: fractal math, render backends, palettes. No UI dependency.
- **`fractouille-tui`** - the current Ratatui terminal frontend.
- **`fractouille-gui`** - a graphical frontend (in progress).
- **`fractouille-cli`** - a headless CLI engine for batch frame/video generation.

This is the foundation everything else builds on and the current priority before new features land.

- [ ] Workspace migration
    - [ ] Extract `fractouille-core`
    - [ ] Migrate TUI to `fractouille-tui`
    - [ ] Scaffold `fractouille-cli`
    - [ ] Scaffold `fractouille-gui`

---

## Fractal Sets

- [x] Mandelbrot Set
- [x] Julia Set
- [x] Burning Ship
- [x] Phoenix Set
- [ ] Newton fractals *(delayed)*
- [ ] Lyapunov fractals *(delayed)*

- [x] Variable power parameter *(needs rework)*

---

## Render Backends

- [x] CPU - single/multithreaded
- [ ] GPU - wgpu compute shaders

---

## CPU Render Methods

- [x] Brute force *(has been removed but will be reintroduced)*
- [x] Adaptive quadtree subdivision *(current default)*
- [ ] Border tracing
- [ ] Arbitrary precision *(brute force, extremely slow)*
- [ ] Arbitrary precision with perturbation theory and series approximation

---

## Optimizations

These can be toggled independently of the render method.

- [x] Cardioid / main bulb skip *(Mandelbrot only)*
- [ ] Cycle detection (Brent's algorithm)

---

## Graphic Features

- [x] Smooth coloring *(on specific render methods only)*

## Output & Export

- [x] Screenshot with auto-save
- [x] Zoom video recording
- [ ] Headless batch rendering via `fractouille-cli`

---

## Palettes

- [x] Built-in palettes with multiple interpolation modes

---

## Frontends

- [x] TUI (ratatui)
- [ ] GUI *(in progress)*