`NonNull<T>`, but `T` is invariant like `*mut T`

Like [`NonNull<T>`], but `T` is invariant like `*mut T`

# Examples

```rust
# use std::mem::{size_of, align_of};
use nonnull_mut::NonNullMut;

assert_eq!(size_of::<NonNullMut<i16>>(), size_of::<Option<NonNullMut<i16>>>());
assert_eq!(align_of::<NonNullMut<i16>>(), align_of::<Option<NonNullMut<i16>>>());

assert_eq!(size_of::<NonNullMut<str>>(), size_of::<Option<NonNullMut<str>>>());
assert_eq!(align_of::<NonNullMut<str>>(), align_of::<Option<NonNullMut<str>>>());
```
