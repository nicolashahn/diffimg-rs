# diffimg-rs

A Rust implementation of [diffimg](https://github.com/nicolashahn/diffimg). Also see
[diffimg-go](https://github.com/nicolashahn/diffimg-go).

Measures the per-pixel difference of two images with identical dimensions as a ratio, or
outputs a difference image showing where the two images differ. Currently supports `jpg`
and `png`.

### Usage

To get a ratio:
```
diffimg image1 image2
```

To generate a diff image (`diff_image` should have `jpg` or `png` extension):
```
diffimg image1 image2 -f diff_image
```
