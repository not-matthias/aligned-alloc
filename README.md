# aligned-alloc

## Example

```rust
let memory: Box<[u8], _> =
            unsafe { Box::new_zeroed_slice_in(100, AlignedAlloc::<4096>).assume_init() };
```