# Recording (WIP)

The `record` command will save frames so you can make a video. (It takes some time)
Then you can merge the frames using ffmpeg for example.

Working command:
```bash
cd fractouille_record/zoom_XXXXX
ffmpeg -framerate 30 -i frame_%04d.png -c:v libx264 -pix_fmt yuv420p -crf 18 video.mp4
```
