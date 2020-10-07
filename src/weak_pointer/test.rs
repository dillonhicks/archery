/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![allow(clippy::eq_op)]
use static_assertions::assert_impl_all;
use std::cell::Cell;
use std::mem;
use std::string::ToString;
use alloc::boxed::Box;
use crate::{WeakPointer, WeakRcK, WeakArcK};

assert_impl_all!(WeakPointer<i32, WeakArcK>: Send, Sync);

#[test]
fn test_new() {
    let weak: WeakPointer<i32, WeakRcK> = WeakPointer::new();
    assert_eq!(weak.strong_count(), 0);
    assert_eq!(weak.weak_count(), 0);
    assert_eq!(weak.upgrade(), None);
}
