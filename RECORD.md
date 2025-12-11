# Recording

The `record` command will save frames and then try to use `ffmpeg` to create a video.
If you don't have `ffmpeg` installed, you will only get the frames.
Everything is saved in the `pictures` folder of your OS.

# Speed parameter

The `speed` parameter defines how fast the zoom progresses in logarithmic space.  
Because fractal zooming is exponential, constant-speed zoom is obtained by changing the scale by a fixed multiplier per second rather than by a fixed amount. In practice, the most useful and visually readable choice is to make the zoom level **double every second** (in my opinion).

This corresponds to a `zoom_speed` value of `ln(2) ≈ 0.693147`, meaning that the zoom grows by a factor of 2 per second, independently of the framerate or output resolution.

This setting produces a smooth and natural zoom progression and is the zoom speed used by the current [deepest Mandelbrot video!](https://www.youtube.com/watch?v=CfqHAOOM8Tw&list=PLKHNByHfHxT4Gl01oxGmuK5UGPkbUTjll)
