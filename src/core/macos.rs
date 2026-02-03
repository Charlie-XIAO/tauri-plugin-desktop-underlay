use std::ffi::c_void;
use std::os::raw::c_ulong;

use anyhow::Result;
use objc2::msg_send;
use objc2::runtime::AnyObject;

unsafe extern "C" {
    fn CGWindowLevelForKey(key: i32) -> i32;
}

// 1 << 0 - NSWindowCollectionBehaviorCanJoinAllSpaces
// 1 << 4 - NSWindowCollectionBehaviorStationary
// 1 << 6 - NSWindowCollectionBehaviorIgnoresCycle
// https://developer.apple.com/documentation/appkit/nswindowcollectionbehavior
const UNDERLAY_COLLECTION_BEHAVIOR: c_ulong = 1 << 0 | 1 << 4 | 1 << 6;

/// Set the window as a desktop underlay.
pub(super) unsafe fn set_underlay(ns_window: *mut c_void) -> Result<()> {
    let ns_window = ns_window as *mut AnyObject;

    // 2 - CGWindowLevelKey.desktopWindow
    // https://developer.apple.com/documentation/coregraphics/cgwindowlevelkey
    let () = msg_send![ns_window, setLevel: CGWindowLevelForKey(2) - 1];

    let behavior: c_ulong = msg_send![ns_window, collectionBehavior];
    let () = msg_send![ns_window, setCollectionBehavior: behavior | UNDERLAY_COLLECTION_BEHAVIOR];
    Ok(())
}

/// Unset the window from being a desktop underlay.
pub(super) unsafe fn unset_underlay(ns_window: *mut c_void) -> Result<()> {
    let ns_window = ns_window as *mut AnyObject;

    // 4 - CGWindowLevelKey.normalWindow
    // https://developer.apple.com/documentation/coregraphics/cgwindowlevelkey
    let () = msg_send![ns_window, setLevel: CGWindowLevelForKey(4)];

    let behavior: c_ulong = msg_send![ns_window, collectionBehavior];
    let () = msg_send![ns_window, setCollectionBehavior: behavior & !UNDERLAY_COLLECTION_BEHAVIOR];
    Ok(())
}
