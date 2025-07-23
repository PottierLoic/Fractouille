# Fractouille

![](example.png)

Fractouille is a simple fractal explorer running in your terminal.

It is better to use cargo build --release to build it, as it is a bit slow otherwise.

## Keybinds

- `wasd`: Move around
- `q`: quit fractouille
- `r/f`: increase/decrease max iterations
- `-/`: decrease/increase zoom level
- `space`: change color palette
- `enter`: change set
- `:`: enter command mode
- `esc`: exit command mode

## Command mode

Pressing `:` enters command mode. A list of all available commands can be found by in the `COMMANDS.md` file.

## TODO:
- [x] Mandelbrot
- [x] Julia
- [x] Burning Ship
- [x] Tanking screenshots
- [x] Smooth coloring on screenshots
- [ ] Have deep zoom
- [ ] Saving location
- [x] Auto move / zoom to saved locations
- [ ] Make more and better looking color palettes
- [x] Being able to change the power, not only quadratic

Here are some screenshots:
![](museum/mandelbrot.png)
![](museum/julia_1752786293_x0.025686664181739313_y0_z3.797498335832415_p6.png)
![](museum/base_ship.png)
