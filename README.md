# huepatitis

![Icon](icon.png)

## What is?

A tool that converts images to reduced palletes.

![Screenshot](before_after.png)

## How do?

```bash
huepatitis --image test_images/bigimage_mountain.jpg --palette palletes/a.p -o 4kpo.png
```

## Options
Theres a no-transparency option if you want to convert transparent pixels to the nearest color in the palette.

```bash 
-n or --notransparency
```

## Notes:
- takes up to 2 seconds for 8k images, mostly for compression, but otherwise is fast
- great place to get palettes: https://lospec.com