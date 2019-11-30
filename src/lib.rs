//! # must_destroy
//! 
//! Must destroy is used to create a paramterized deconstructor for a type
//! that must be explicitly called.
//! 
//! `MustDestroy<T, Args>` acts as a guard for a wrapped type that implements the `Destroy`
//! trait, that causes a `panic` if the guard is dropped.
//! 
//! However, calling destroy upon the guard, will call destroy on wrapped child, and will
//! be consumed safely.
use std::mem::forget;
use std::marker::PhantomData;
use std::ops::{Drop, Deref, DerefMut};

/// Trait applied to items that can be destroyed.
///
/// `Args` represents the type to act as an arguments to the destructor. For multiple
/// arguments you can use a `tuple`
pub trait Destroy<Args> {
    /// destroy is like an explicit deconstructor that may require additional
    /// arguments to perform it's ability to destroy.
    fn destroy(self, args: Args);
}

/// The value contained is an item that can't be dropped and must be
/// destroyed via calling it's `Destroy::destroy` method.
pub struct MustDestroy<T: Destroy<Args>, Args>
{
    wrapped: T,
    _args_marker: PhantomData<Args>
}

impl<T: Destroy<Args>, Args> MustDestroy<T, Args> {
    /// Create a new `MustDestroy` for the given item
    pub fn new(item: T) -> Self {
        MustDestroy {
            wrapped: item,
            _args_marker: PhantomData
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

impl<T: Destroy<Args>, Args> Destroy<Args> for MustDestroy<T, Args>
{
    fn destroy(self, args: Args) {
        self.into_inner().destroy(args);
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