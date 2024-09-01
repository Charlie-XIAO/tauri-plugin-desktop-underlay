// Copyright 2024 Yao Xiao
// SPDX-License-Identifier: MIT

use std::os::raw::c_ulong;

use objc::{msg_send, runtime::Object, sel, sel_impl};

extern "C" {
    fn CGWindowLevelForKey(key: i32) -> i32;
}

/// Set the window as a desktop underlay.
pub(super) unsafe fn set_underlay(ns_window: *mut Object) {
    // 2 - CGWindowLevelKey.desktopWindow
    // https://developer.apple.com/documentation/coregraphics/cgwindowlevelkey
    let () = msg_send![ns_window, setLevel: CGWindowLevelForKey(2) - 1];

    // 1 << 0 - NSWindowCollectionBehaviorCanJoinAllSpaces
    // 1 << 4 - NSWindowCollectionBehaviorStationary
    // 1 << 6 - NSWindowCollectionBehaviorIgnoresCycle
    // https://developer.apple.com/documentation/appkit/nswindowcollectionbehavior
    let behavior: c_ulong = msg_send![ns_window, collectionBehavior];
    let () = msg_send![ns_window, setCollectionBehavior: behavior | (1 << 0) | (1 << 4) | (1 << 6)];
}

/// Unset the window from being a desktop underlay.
pub(super) unsafe fn unset_underlay(ns_window: *mut Object) {
    // 4 - CGWindowLevelKey.normalWindow
    // https://developer.apple.com/documentation/coregraphics/cgwindowlevelkey
    let () = msg_send![ns_window, setLevel:CGWindowLevelForKey(4)];

    // 1 << 0 - NSWindowCollectionBehaviorCanJoinAllSpaces
    // 1 << 4 - NSWindowCollectionBehaviorStationary
    // 1 << 6 - NSWindowCollectionBehaviorIgnoresCycle
    // https://developer.apple.com/documentation/appkit/nswindowcollectionbehavior
    let behavior: c_ulong = msg_send![ns_window, collectionBehavior];
    let () = msg_send![ns_window, setCollectionBehavior: behavior & !((1 << 0) | (1 << 4) | (1 << 6))];
}
