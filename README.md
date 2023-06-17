![alt text](https://raw.githubusercontent.com/no-venv/Quadtree-Image-Compressor/main/Logo.svg)

---
### Quad Tree Image Format | image compression based on quadtrees

Main focus:
- Compressing images while being pleasant to look at 
- Compressess similar values across YCbCr channels 
- Simplicity in encoding and decoding

Included in this repository:
- Encoder
- Decoder

Encoder output file is just.. "output"

### Build
---
FFMPEG is required to convert the images into YCbCr422 during compression and back when decoding

You can get the appropriate FFMPEG binaries for your platform by running the python script, `download_ffmpeg.py`

Linux
``sh build.sh``

Windows
``build.bat``

### Arguments 
---
```
  <FILENAME>                 image file
  <LUM_BLOCK>                luminance (y) pixel block size (must be within the powers of 2) a pixel size of 8 is 8x8
  <COLOR_BLOCK>              chrominance (u,v) pixel block size (must be within the powers of 2) a pixel size of 8 is 8x8
  <LUM_DIST>                 luminance (y) allowed distance
  <COLOR_DIST>               chrominance (u,v) allowed distance
  <DEPTH_COMPRESSION_LUM>    luminance (y) depth compression intensity
  <DEPTH_COMPRESSION_COLOR>  chrominance (u,v) depth compression intensity

Options:
  -r, --resize <RESIZE> <RESIZE>  resize image by width x height, optional

```
