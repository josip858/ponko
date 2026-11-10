### Added
- `GpuUploads::reallocations`: the buffers and textures a frame replaced
  because they were outgrown (they grow to the next power of two, so a
  growing world replaces them a logarithmic number of times and a steady
  one none). In
  benchmark reports, `perf_report` and `perf.get`'s `render.uploads`.
- C and C++: `ponko_ctx_gpu_uploads` with `PonkoGpuUploads`, and
  `ctx.gpu_uploads()` in C++: the bytes the renderer uploaded last frame
  by kind and the buffers it made.

### Changed
- The renderer uploads only the parts of its per-frame buffers that
  changed since the last frame: the frame settings of the camera, target
  cameras, probe views and every shadow view (each a copy before),
  post-processing and effect settings, motion records, skin matrices and
  morph weights, instance batch views, particles, ribbons, lines, 3D text,
  and 2D sprites and cameras. Measured offscreen (1,024 shadowed models,
  clock running): a still scene went from 15,616 to 80 bytes a frame,
  one with 1% of its models moving under temporal antialiasing from
  20,640 to 4,331, a panning camera from 17,498 to 4,994, and 2,000 still
  particles with 8 lines from 113,440 to 80. See the guide's "What a
  frame sends the GPU".
