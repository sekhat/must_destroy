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
use std::marker::PhantomData;
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
pub struct MustDestroy<T: Destroy<Args>, Args> {
    wrapped: T,
    _args_marker: PhantomData<Args>,
}

impl<Args, T: Destroy<Args>> MustDestroy<T, Args> {
    /// Create a new `MustDestroy` for the given item
    pub fn new(item: T) -> Self {
        MustDestroy {
            wrapped: item,
            _args_marker: PhantomData,
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

impl<Args, T: Destroy<Args>> Destroy<Args> for MustDestroy<T, Args> {
    fn destroy(self, args: Args) {
        self.into_inner().destroy(args);
    }
}

impl<T: Destroy<()>> MustDestroy<T, ()> {
    fn destroy(self) {
        Destroy::destroy(self, ())
    }
}

impl<A1, A2, A3, A4, T: Destroy<(A1, A2, A3, A4)>> MustDestroy<T, (A1, A2, A3, A4)> {
    fn destroy(self, arg1: A1, arg2: A2, arg3: A3, arg4: A4) {
        Destroy::destroy(self, (arg1, arg2, arg3, arg4))
    }
}

impl<A1, A2, A3, T: Destroy<(A1, A2, A3)>> MustDestroy<T, (A1, A2, A3)> {
    fn destroy(self, arg1: A1, arg2: A2, arg3: A3) {
        Destroy::destroy(self, (arg1, arg2, arg3))
    }
}

impl<A1, A2, T: Destroy<(A1, A2)>> MustDestroy<T, (A1, A2)> {
    fn destroy(self, arg1: A1, arg2: A2) {
        Destroy::destroy(self, (arg1, arg2))
    }
}

impl<A1, T: Destroy<(A1,)>> MustDestroy<T, (A1,)> {
    fn destroy(self, arg1: A1) {
        Destroy::destroy(self, (arg1,))
    }
}

impl<Args, T: Destroy<Args>> Drop for MustDestroy<T, Args> {
    fn drop(&mut self) {
        panic!("Can not drop, must call destroy.");
    }
}

impl<Args, T: Destroy<Args>> Deref for MustDestroy<T, Args> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

impl<Args, T: Destroy<Args>> DerefMut for MustDestroy<T, Args> {
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
        destroy_me.destroy("Test String", 12);
    }
}
