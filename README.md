# pbrt-rs

WORK IN PROGRESS

> Physically Based Raytracing Renderer written in Rust

Based on:

- https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/
- https://bheisler.github.io/post/writing-gpu-accelerated-path-tracer-part-1/
- http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html?m=1
- https://www.scratchapixel.com/index.php?redirect
- http://pbrt.org/
- http://www.pbr-book.org/3ed-2018/contents.html
- https://pharr.org/matt/blog/2018/07/16/moana-island-pbrt-all.html

### Current progress

![presentation](https://raw.githubusercontent.com/baransu/pbrt-rs/master/test.png)

### TODO

- global illumination / ambient light
- better code structure
- obj models support
- depth of field
- improved parallelization (not per pixel but per tile, will minimize load balancing overhead)
