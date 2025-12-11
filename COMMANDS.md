# Commands

This is a list of all the available commands in Fractouille. They can be entered like Vim commands.

- `move <x> <y> [<zoom>]`
  Moves the fractal view to the specified x and y coordinates. Optionally sets the zoom level. If zoom is omitted, the current zoom level is unchanged.

- `reset`
  Resets the fractal to its default state, including center position, zoom, iterations, power, and set type.

- `screenshot [<width> <height>]`
  Saves a screenshot of the current fractal view as a PNG file. If width and height are provided, the screenshot is rendered at the specified resolution; otherwise, the default resolution (1920x1080) is used.

- `help`
  Toggles the extended menu, showing detailed fractal parameters and command mode instructions. Alias: h

- `quit`
  Exits the application. Alias: q

- `julia <real> <imaginary>`
  Sets the complex constant for the Julia set (real + imaginary*i).

- `phoenix_c <real>`
  Sets the real constant c for the Phoenix set.

- `phoenix_p <real> <imaginary>`
  Sets the complex constant p for the Phoenix set (real + imaginary*i).

- `power <value>`
  Sets the power value for the fractal equation (e.g., z^power for Mandelbrot/Julia). Affects the fractal's shape and computation speed.

- `iterations <count>`
  Set the maximum number of iterations for fractal computation. Higher values increase detail but may slow rendering.

- `zoom <zoom_level>`
  Sets the zoom level for the fractal view.

- `set <type>`
  Sets the fractal set type. Available types: mandelbrot, julia, burningship, phoenix

- `record <width> <height> <start> <end> <speed>`
  Record frames of specified size from `start` to `end` zoom level. Can then be used to make a video using external tools.

- `cycle_speed <n>`
  Set the `cycle_speed` value of current palette  to `n`, it controls the speed at which the palette will cycle through colors.

- `palette create <r0> <g0> <b0> ... <rn> <gn> <bn>`
  Creates a new custom palette from a list of RGB triplets. Each color is three integers (0–255). The palette is appended to the palette list and its index is returned.

- `palette use <index>`
  Sets the active palette to the palette at the given index.

- `palette delete <index>`
  Deletes the palette at the given index. The last remaining palette cannot be deleted.

- `palette mode <linear|cosine|hsv|hsv_cyclic>`
  Changes the interpolation mode of the currently active palette.