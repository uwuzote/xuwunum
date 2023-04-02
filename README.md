# XUwUNum
Type-level natural numbers and operations on them.

```rust
#![recursion_limit = "500"]

# use xuwunum::{
#     nat::{nums::*, NAdd, NMul, NZSub, NDivRem, NDivQuot},
#     runtime::rt
# };
# fn main() {
assert_eq!(
    rt::<NAdd<N26, N43>, u8>(),
    69
);

assert_eq!(
    rt::<NMul<N21, N20>, u32>(),
    420
);

assert_eq!(
    rt::<NZSub<N420, N363>, i8>(),
    57
);

assert_eq!(
    rt::<NDivQuot<N500, N3>, u8>(),
    166
);

assert_eq!(
    rt::<NDivRem<N500, N3>, u8>(),
    2
)
# }
```