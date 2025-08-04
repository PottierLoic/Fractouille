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

- `phoenix <real> <imaginary>`
  Sets the complex constant for the Phoenix set (real + imaginary*i).

- `power <value>`
  Sets the power value for the fractal equation (e.g., z^power for Mandelbrot/Julia). Affects the fractal's shape and computation speed.

- `iterations <count>`
  Set the maximum number of iterations for fractal computation. Higher values increase detail but may slow rendering.

- `zoom <factor>`
  Sets the zoom factor for the fractal view.

- `set <type>`
  Sets the fractal set type. Available types: mandelbrot, julia, burningship