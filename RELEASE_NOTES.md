# Pushrod Releases

## 0.1.4

- Run loop optimization:
  - Optimized mouse movement - repetitive points are redundant.
  - Added invalidation to set origin, size, and color.
  - Added clear_invalidation flag to draw.
  - Getting some REALLY bizarre screen flickering when trying to draw invalidated objects, skipping that logic for now.
- Added tests for Points.
- Updated callbacks to use widget_id when calling mouse enter, exit, scroll.
- Removed context reset from trait object default draw method.
- Renamed simple example to "simple"
- Adjusted Cargo.toml to include keywords and README.
- Got Travis.CI building the application.
- Shortened README.

## 0.1.3

- Got some assistance from @andygrove for documentation and testing.
- Completed documentation in the application
- Moved example away from the `src` directory, and into its own `examples` directory.

## 0.1.2

- Adjustments for Crates.

## 0.1.1

- Updated Cargo Crates to use latest piston library and associated graphics libraries.
- Fixed bug found with configs: now returns default values in default trait if not found.
- Added convenience method to create a width and height of 0x0

## 0.1.0

- First release
