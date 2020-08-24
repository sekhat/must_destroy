//! # must_destroy
//!
//! Must destroy is used to create a paramterized destructor for a type
//! that must be explicitly called.
//!
//! `MustDestroy<T, Args>` acts as a guard for a wrapped type that implements the `Destroy`
//! trait, that causes a `panic` if the guard is dropped.
//!
//! However, calling destroy upon the guard, will call destroy on wrapped child, and will
//! be consumed safely.
use std::mem::forget;
use std::ops::{Deref, DerefMut, Drop};

/// Trait applied to items that can be destroyed.
///
/// `Args` represents the type to act as an arguments to the destructor. For multiple
/// arguments you can use a `tuple`
pub trait Destroy<Args> {
    /// destroys the item being called upon.
    fn destroy(self, args: Args);
}

/// The value contained is an item that can't be dropped and must be
/// destroyed via calling it's `Destroy::destroy` method.
pub struct MustDestroy<T> {
    wrapped: T
}

impl<T> MustDestroy<T> {
    /// Create a new `MustDestroy` for the given item
    pub fn new(item: T) -> Self {
        MustDestroy {
            wrapped: item,
        }
    }

    /// Removes the contained item from the MustDestroy guard
    pub fn into_inner(mut self) -> T {
        // Safe because we never actually use the zeroed value, we just need to get the original
        // value out of the struct, without dropping it.
        let wrapped = std::mem::replace(&mut self.wrapped, unsafe { std::mem::zeroed() });
        // as self is consumed by the function, and without the wrapped value there is
        // nothing else to do, we can just forget ourselves.
        forget(self);
        wrapped
    }
}

impl<Args, T: Destroy<Args>> Destroy<Args> for MustDestroy<T> {
    fn destroy(self, args: Args) {
        self.into_inner().destroy(args);
    }
}

impl<T: Destroy<()>> MustDestroy<T> {
    pub fn destroy(self) {
        Destroy::destroy(self, ())
    }
}

impl<T> Drop for MustDestroy<T> {
    fn drop(&mut self) {
        panic!("Can not drop, must call destroy.");
    }
}

impl<T> Deref for MustDestroy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

impl<T> DerefMut for MustDestroy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wrapped
    }
}

#[cfg(test)]
mod tests {
    use crate::{Destroy, MustDestroy};
    #[test]
    fn test_readme() {
        struct MyDestroyableItem;

        impl Destroy<(&'_ str, i32)> for MyDestroyableItem {
            fn destroy(self, args: (&str, i32)) {
                // Do things to destroy item
                assert_eq!("Test String", args.0);
                assert_eq!(12, args.1);
            }
        }

        let destroy_me = MustDestroy::new(MyDestroyableItem);

        // Dropping the item here would cause a panic at runtime
        // drop(destroy_me)

        // However calling destroy will consume the item, and not cause
        // a panic.
        destroy_me.destroy(("Test String", 12));
    }
}
