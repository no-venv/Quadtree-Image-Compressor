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


### Compression Algorithm 
---

- We work with Images work with YUV format
  - This algorithm uses the YUV422 planar variant 

- First split the images into a specified pixel group (take 8x8 for example)
- Then, get the minimum and maximum values within the pixel group
- Subtract the minimum and maximum values, which would be the differences 
- Check to see if the difference is within the allowed threshold 
  - Difference < Threshold?
- If the distance is within the allowed threshold 
  - Get the average of the pixel group 
- If not,
  - Split the pixel group into 4 and rerun the algorithm again
- There is extra steps taken to further compress
  - Bit packing the entire image data
  - Reusing last average values, if determined to be similar 

