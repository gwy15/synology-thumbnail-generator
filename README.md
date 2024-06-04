# Synology Photos Thumbnail Generator

This program generates Synology Photos thumbnails which by default takes a thousand years to generate.

It is only compatible with DSM 7.

It is only compatible with images with ratio 3:2 (portrait or landscape).

```
SYNOPHOTO_THUMB_SM.jpg: 360x240 (3:2)
SYNOPHOTO_THUMB_M.jpg: 480x320 (3:2)
SYNOPHOTO_THUMB_XL.jpg: 1920x1280 (3:2)
```

## Build on Linux
```
apt-get update
apt-get install -y clang libclang-dev libopencv-dev
```

## Run on Linux
```
apt-get update
apt-get install -y libopencv-core libopencv-imgproc libopencv-imgcodecs
```
