# Recording

The `record` command streams frames directly to `ffmpeg` and saves the resulting video to the `fractouille` folder in your system's Pictures directory. `ffmpeg` is required, the command will not work without it.

## Speed parameter

The `speed` parameter defines how fast the zoom progresses in logarithmic space.  
Because fractal zooming is exponential, constant-speed zoom is obtained by changing the scale by a fixed multiplier per second rather than by a fixed amount.  
In practice, the most useful and visually readable choice is to make the zoom level **double every second** (in my opinion).

This corresponds to a `speed` value of `ln(2) ≈ 0.693147`, meaning that the zoom grows by a factor of 2 per second, independently of the framerate or output resolution.

This setting produces a smooth and natural zoom progression and is the zoom speed used by the current [deepest Mandelbrot video!](https://www.youtube.com/watch?v=CfqHAOOM8Tw&list=PLKHNByHfHxT4Gl01oxGmuK5UGPkbUTjll)
