/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::weak_pointer::kind::WeakPointerKind;
use alloc::boxed::Box;
use alloc::sync::Weak;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::mem;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr;

type UntypedWeak = Weak<()>;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`Weak`](https://doc.rust-lang.org/std/sync/struct.Weak.html) pointers.
pub struct WeakArcK {
    /// We use `ManuallyDrop` here, so that we can drop it explicitly as `Weak<T>`.  Not sure if it
    /// can be dropped as `UntypedArc`, but it seems to be playing with fire (even more than we
    /// already are).
    inner: ManuallyDrop<UntypedWeak>,
}

impl WeakArcK {
    #[inline(always)]
    pub(crate) fn new_from_inner<T>(arc: Weak<T>) -> WeakArcK {
        WeakArcK { inner: ManuallyDrop::new(unsafe { mem::transmute(arc) }) }
    }

    #[inline(always)]
    unsafe fn as_inner_ref<T>(&self) -> &Weak<T> {
        let arc_t: *const Weak<T> = self.inner.deref() as *const UntypedWeak as *const Weak<T>;

        // Static check to make sure we are not messing up the sizes.
        // This could happen if we allowed for `T` to be unsized, because it would need to be
        // represented as a wide pointer inside `Arc`.
        // TODO Use static_assertion when https://github.com/nvzqz/static-assertions-rs/issues/21
        //   gets fixed
        let _ = mem::transmute::<UntypedWeak, Weak<T>>;

        &*arc_t
    }

    #[inline(always)]
    unsafe fn as_inner_mut<T>(&mut self) -> &mut Weak<T> {
        let arc_t: *mut Weak<T> = self.inner.deref_mut() as *mut UntypedWeak as *mut Weak<T>;

        &mut *arc_t
    }
}

impl WeakPointerKind for WeakArcK {
    type SharedPtr = crate::shared_pointer::kind::ArcK;

    #[inline(always)]
    fn new<T>() -> WeakArcK {
        WeakArcK::new_from_inner(Weak::<T>::new())
    }

    #[inline(always)]
    unsafe fn strong_count<T>(&self) -> usize {
        Weak::strong_count(self.as_inner_ref::<T>())
    }

    #[inline(always)]
    unsafe fn weak_count<T>(&self) -> usize {
        Weak::weak_count(self.as_inner_ref::<T>())
    }


    #[inline(always)]
    unsafe fn upgrade<T>(&self) -> Option<Self::SharedPtr> {
        Weak::<T>::upgrade(self.as_inner_ref())
            .map(|rc| Self::SharedPtr::new_from_inner(rc))
    }

    #[inline(always)]
    unsafe fn as_ptr<T>(&self) -> *const T {
        self.as_inner_ref::<T>().as_ptr()
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> WeakArcK {
        WeakArcK { inner: ManuallyDrop::new(Weak::clone(self.as_inner_ref())) }
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        ptr::drop_in_place::<Weak<T>>(self.as_inner_mut());
    }

}

impl Debug for WeakArcK {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("WeakArcK")
    }
}
