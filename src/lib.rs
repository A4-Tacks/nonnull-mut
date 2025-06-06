#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::{cmp::Ordering, fmt, hash, marker::PhantomData, num::NonZeroUsize, ptr::NonNull};

/// Like [`NonNull<T>`], but `T` is invariant like `*mut T`
///
/// # Examples
///
/// ```
/// # use std::mem::{size_of, align_of};
/// use nonnull_mut::NonNullMut;
///
/// assert_eq!(size_of::<NonNullMut<i16>>(), size_of::<Option<NonNullMut<i16>>>());
/// assert_eq!(align_of::<NonNullMut<i16>>(), align_of::<Option<NonNullMut<i16>>>());
///
/// assert_eq!(size_of::<NonNullMut<str>>(), size_of::<Option<NonNullMut<str>>>());
/// assert_eq!(align_of::<NonNullMut<str>>(), align_of::<Option<NonNullMut<str>>>());
/// ```
#[repr(transparent)]
pub struct NonNullMut<T: ?Sized> {
    inner: NonNull<T>,
    _phantom: PhantomData<*mut T>,
}

impl<T: ?Sized> From<NonNull<T>> for NonNullMut<T> {
    fn from(inner: NonNull<T>) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> From<NonNullMut<T>> for NonNull<T> {
    fn from(value: NonNullMut<T>) -> Self {
        value.inner
    }
}

impl<T> NonNullMut<T> {
    /// Like [`NonNull::dangling`]
    ///
    /// # Examples
    ///
    /// ```
    /// use nonnull_mut::NonNullMut;
    ///
    /// let ptr = NonNullMut::<u32>::dangling();
    /// // Important: don't try to access the value of `ptr` without
    /// // initializing it first! The pointer is not null but isn't valid either!
    /// ```
    #[inline]
    #[must_use]
    pub const fn dangling() -> Self {
        let inner = NonNull::dangling();
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> NonNullMut<T> {
    /// Like [`NonNull::new`]
    ///
    /// # Examples
    ///
    /// ```
    /// use nonnull_mut::NonNullMut;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNullMut::<u32>::new(&mut x as *mut _).expect("ptr is null!");
    ///
    /// if let Some(ptr) = NonNullMut::<u32>::new(std::ptr::null_mut()) {
    ///     unreachable!();
    /// }
    /// ```
    #[inline]
    pub const fn new(ptr: *mut T) -> Option<Self> {
        match NonNull::new(ptr) {
            Some(inner) => Some(Self {
                inner,
                _phantom: PhantomData,
            }),
            None => None,
        }
    }

    /// Like [`NonNull::new_unchecked`]
    ///
    /// # Safety
    /// - Like [`NonNull::new_unchecked`]
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        let inner = unsafe { NonNull::new_unchecked(ptr) };
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    /// Create [`NonNullMut<T>`] from [`NonNull<T>`]
    pub const fn from_inner(inner: NonNull<T>) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    /// Like [`NonNull::addr`]
    #[inline]
    #[must_use]
    pub fn addr(self) -> NonZeroUsize {
        self.inner.addr()
    }

    /// Like [`NonNull::with_addr`]
    #[inline]
    #[must_use]
    pub fn with_addr(self, addr: NonZeroUsize) -> Self {
        self.inner.with_addr(addr).into()
    }

    /// Like [`NonNull::map_addr`]
    #[inline]
    #[must_use]
    pub fn map_addr(self, f: impl FnOnce(NonZeroUsize) -> NonZeroUsize) -> Self {
        self.inner.map_addr(f).into()
    }

    /// Like [`NonNull::as_ptr`]
    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(self) -> *mut T {
        self.inner.as_ptr()
    }

    /// Get inner [`NonNull<T>`]
    pub const fn as_inner(self) -> NonNull<T> {
        self.inner
    }

    /// Like [`NonNull::as_ref`]
    ///
    /// # Safety
    /// - Like [`NonNull::as_ref`]
    #[inline(always)]
    #[must_use]
    pub const unsafe fn as_ref<'a>(&self) -> &'a T {
        unsafe { self.inner.as_ref() }
    }

    /// Like [`NonNull::as_mut`]
    ///
    /// # Safety
    /// - Like [`NonNull::as_mut`]
    #[inline(always)]
    #[must_use]
    pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
        unsafe { self.inner.as_mut() }
    }

    /// Like [`NonNull::cast`]
    #[inline]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub const fn cast<U>(self) -> NonNull<U> {
        self.inner.cast()
    }

    /// Like [`NonNull::offset`]
    ///
    /// # Safety
    /// - Like [`NonNull::offset`]
    #[inline(always)]
    #[must_use = "returns a new pointer rather than modifying its argument"]
    pub const unsafe fn offset(self, count: isize) -> Self
    where
        T: Sized,
    {
        unsafe { Self::from_inner(self.inner.offset(count)) }
    }

    /// Like [`NonNull::byte_offset`]
    ///
    /// # Safety
    /// - Like [`NonNull::byte_offset`]
    #[inline(always)]
    #[must_use]
    pub const unsafe fn byte_offset(self, count: isize) -> Self {
        unsafe { Self::from_inner(self.inner.byte_offset(count)) }
    }

    /// Like [`NonNull::add`]
    ///
    /// # Safety
    /// - Like [`NonNull::add`]
    #[inline(always)]
    #[must_use = "returns a new pointer rather than modifying its argument"]
    pub const unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        unsafe { Self::from_inner(self.inner.add(count)) }
    }

    /// Like [`NonNull::byte_add`]
    ///
    /// # Safety
    /// - Like [`NonNull::byte_add`]
    #[inline(always)]
    #[must_use]
    pub const unsafe fn byte_add(self, count: usize) -> Self {
        unsafe { Self::from_inner(self.inner.byte_add(count)) }
    }

    /// Like [`NonNull::sub`]
    ///
    /// # Safety
    /// - Like [`NonNull::sub`]
    #[inline(always)]
    #[must_use = "returns a new pointer rather than modifying its argument"]
    pub const unsafe fn sub(self, count: usize) -> Self
    where
        T: Sized,
    {
        unsafe { Self::from_inner(self.inner.sub(count)) }
    }

    /// Like [`NonNull::byte_sub`]
    ///
    /// # Safety
    /// - Like [`NonNull::byte_sub`]
    #[inline(always)]
    #[must_use]
    pub const unsafe fn byte_sub(self, count: usize) -> Self {
        unsafe { Self::from_inner(self.inner.byte_sub(count)) }
    }

    /// Like [`NonNull::offset_from`]
    ///
    /// # Safety
    /// - Like [`NonNull::offset_from`]
    #[inline]
    pub const unsafe fn offset_from(self, origin: NonNull<T>) -> isize
    where
        T: Sized,
    {
        unsafe { self.inner.offset_from(origin) }
    }

    /// Like [`NonNull::byte_offset_from`]
    ///
    /// # Safety
    /// - Like [`NonNull::byte_offset_from`]
    #[inline(always)]
    pub const unsafe fn byte_offset_from<U: ?Sized>(self, origin: NonNull<U>) -> isize {
        unsafe { self.inner.byte_offset_from(origin) }
    }

    /// Like [`NonNull::read`]
    ///
    /// # Safety
    /// - Like [`NonNull::read`]
    #[inline]
    pub const unsafe fn read(self) -> T
    where
        T: Sized,
    {
        unsafe { self.inner.read() }
    }

    /// Like [`NonNull::read_volatile`]
    ///
    /// # Safety
    /// - Like [`NonNull::read_volatile`]
    #[inline]
    pub unsafe fn read_volatile(self) -> T
    where
        T: Sized,
    {
        unsafe { self.inner.read_volatile() }
    }

    /// Like [`NonNull::read_unaligned`]
    ///
    /// # Safety
    /// - Like [`NonNull::read_unaligned`]
    #[inline]
    pub const unsafe fn read_unaligned(self) -> T
    where
        T: Sized,
    {
        unsafe { self.inner.read_unaligned() }
    }

    /// Like [`NonNull::copy_to`]
    ///
    /// # Safety
    /// - Like [`NonNull::copy_to`]
    #[inline(always)]
    pub const unsafe fn copy_to(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        unsafe { self.inner.copy_to(dest, count) }
    }

    /// Like [`NonNull::copy_to_nonoverlapping`]
    ///
    /// # Safety
    /// - Like [`NonNull::copy_to_nonoverlapping`]
    #[inline(always)]
    pub const unsafe fn copy_to_nonoverlapping(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        unsafe { self.inner.copy_to_nonoverlapping(dest, count) }
    }

    /// Like [`NonNull::copy_from`]
    ///
    /// # Safety
    /// - Like [`NonNull::copy_from`]
    #[inline(always)]
    pub const unsafe fn copy_from(self, src: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        unsafe { self.inner.copy_from(src, count) }
    }

    /// Like [`NonNull::copy_from_nonoverlapping`]
    ///
    /// # Safety
    /// - Like [`NonNull::copy_from_nonoverlapping`]
    #[inline(always)]
    pub const unsafe fn copy_from_nonoverlapping(self, src: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        unsafe { self.inner.copy_from_nonoverlapping(src, count) }
    }

    /// Like [`NonNull::drop_in_place`]
    ///
    /// # Safety
    /// - Like [`NonNull::drop_in_place`]
    #[inline(always)]
    pub unsafe fn drop_in_place(self) {
        unsafe { self.inner.drop_in_place() }
    }

    /// Like [`NonNull::write`]
    ///
    /// # Safety
    /// - Like [`NonNull::write`]
    #[inline(always)]
    pub const unsafe fn write(self, val: T)
    where
        T: Sized,
    {
        unsafe { self.inner.write(val) }
    }

    /// Like [`NonNull::write_bytes`]
    ///
    /// # Safety
    /// - Like [`NonNull::write_bytes`]
    #[inline(always)]
    pub const unsafe fn write_bytes(self, val: u8, count: usize)
    where
        T: Sized,
    {
        unsafe { self.inner.write_bytes(val, count) }
    }

    /// Like [`NonNull::write_volatile`]
    ///
    /// # Safety
    /// - Like [`NonNull::write_volatile`]
    #[inline(always)]
    pub unsafe fn write_volatile(self, val: T)
    where
        T: Sized,
    {
        unsafe { self.inner.write_volatile(val) }
    }

    /// Like [`NonNull::write_unaligned`]
    ///
    /// # Safety
    /// - Like [`NonNull::write_unaligned`]
    #[inline(always)]
    pub const unsafe fn write_unaligned(self, val: T)
    where
        T: Sized,
    {
        unsafe { self.inner.write_unaligned(val) }
    }

    /// Like [`NonNull::replace`]
    ///
    /// # Safety
    /// - Like [`NonNull::replace`]
    #[inline(always)]
    pub unsafe fn replace(self, src: T) -> T
    where
        T: Sized,
    {
        unsafe { self.inner.replace(src) }
    }

    /// Like [`NonNull::swap`]
    ///
    /// # Safety
    /// - Like [`NonNull::swap`]
    #[inline(always)]
    pub const unsafe fn swap(self, with: NonNull<T>)
    where
        T: Sized,
    {
        unsafe { self.inner.swap(with) }
    }

    /// Like [`NonNull::align_offset`]
    #[inline]
    #[must_use]
    pub fn align_offset(self, align: usize) -> usize
    where
        T: Sized,
    {
        self.inner.align_offset(align)
    }

    /// Like [`NonNull::is_aligned`]
    #[inline]
    #[must_use]
    pub fn is_aligned(self) -> bool
    where
        T: Sized,
    {
        self.inner.is_aligned()
    }
}

impl<T> NonNullMut<[T]> {
    /// Like [`NonNull::slice_from_raw_parts`]
    #[inline]
    #[must_use]
    pub const fn slice_from_raw_parts(data: NonNull<T>, len: usize) -> Self {
        Self::from_inner(NonNull::slice_from_raw_parts(data, len))
    }

    /// Like [`NonNull::len`]
    #[inline]
    #[must_use]
    pub const fn len(self) -> usize {
        self.inner.len()
    }

    /// Like [`NonNull::is_empty`]
    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: ?Sized> Clone for NonNullMut<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: ?Sized> Copy for NonNullMut<T> {}

impl<T: ?Sized> fmt::Debug for NonNullMut<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> fmt::Pointer for NonNullMut<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

impl<T: ?Sized> Eq for NonNullMut<T> {}

#[allow(ambiguous_wide_pointer_comparisons)]
impl<T: ?Sized> PartialEq for NonNullMut<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

#[allow(ambiguous_wide_pointer_comparisons)]
impl<T: ?Sized> Ord for NonNullMut<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

#[allow(ambiguous_wide_pointer_comparisons)]
#[allow(clippy::non_canonical_partial_ord_impl)]
impl<T: ?Sized> PartialOrd for NonNullMut<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

#[allow(ambiguous_wide_pointer_comparisons)]
impl<T: ?Sized> hash::Hash for NonNullMut<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

impl<T: ?Sized> From<&mut T> for NonNullMut<T> {
    #[inline]
    fn from(r: &mut T) -> Self {
        NonNullMut::from_inner(r.into())
    }
}

impl<T: ?Sized> From<&T> for NonNullMut<T> {
    #[inline]
    fn from(r: &T) -> Self {
        NonNullMut::from_inner(r.into())
    }
}
