/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use alloc::boxed::Box;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::hash::Hash;
use core::hash::Hasher;
use core::marker::PhantomData;
use core::mem;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr;

use crate::SharedPointer;
use crate::weak_pointer::kind::WeakPointerKind;

pub struct WeakPointer<T, P>
    where
        P: WeakPointerKind,
{
    ptr: ManuallyDrop<P>,
    _phantom_t: PhantomData<T>,
}

impl<T, P> WeakPointer<T, P>
    where
        P: WeakPointerKind,
{
    #[inline(always)]
    pub(crate) fn new_from_inner(ptr: P) -> WeakPointer<T, P> {
        WeakPointer { ptr: ManuallyDrop::new(ptr), _phantom_t: PhantomData }
    }

    #[inline(always)]
    pub fn new() -> WeakPointer<T, P> {
        WeakPointer::new_from_inner(P::new::<T>())
    }

    #[inline(always)]
    pub fn weak_count(&self) -> usize {
        unsafe { self.ptr.weak_count::<T>() }
    }

    #[inline(always)]
    pub fn strong_count(&self) -> usize {
        unsafe { self.ptr.strong_count::<T>() }
    }

    #[inline(always)]
    pub fn ptr_eq<PO: WeakPointerKind>(
        this: &WeakPointer<T, P>,
        other: &WeakPointer<T, PO>,
    ) -> bool {
        unsafe { ptr::eq(this.ptr.as_ptr::<T>(), other.ptr.as_ptr::<T>()) }
    }

    #[inline(always)]
    pub fn upgrade(&self) -> Option<SharedPointer<T, <P as WeakPointerKind>::SharedPtr>> {
        unsafe { self.ptr.upgrade::<T>() }
            .map(SharedPointer::new_from_inner)
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        unsafe { self.ptr.as_ptr::<T>() }
    }
}


impl<T, P> Clone for WeakPointer<T, P>
    where
        P: WeakPointerKind,
{
    #[inline(always)]
    fn clone(&self) -> WeakPointer<T, P> {
        WeakPointer::new_from_inner(unsafe { self.ptr.deref().clone::<T>() })
    }
}


impl<T, P> Debug for WeakPointer<T, P>
    where
        T: Debug,
        P: WeakPointerKind,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(&self.ptr, f)
    }
}


impl<T, P> Drop for WeakPointer<T, P>
    where
        P: WeakPointerKind,
{
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop::<T>();
        }
    }
}

pub mod kind;


#[cfg(test)]
mod test;
