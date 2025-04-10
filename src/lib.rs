//! Utilities for working with user-space pointers.
#![no_std]
#![feature(layout_for_ptr)]
#![feature(maybe_uninit_slice)]
#![feature(pointer_is_aligned_to)]
#![feature(ptr_as_uninit)]
#![feature(ptr_sub_ptr)]

extern crate alloc;

use alloc::boxed::Box;
use core::alloc::Layout;

use axerrno::{AxError, AxResult};
use crate_interface::def_interface;
use memory_addr::VirtAddrRange;

pub trait AddrSpaceGuard {
    /// Tries to access a specific range of user memory.
    ///
    /// This function should also populate the memory area.
    ///
    /// Returns `Ok(())` if the access is allowed and the memory area can be
    /// populated.
    fn access_range(&mut self, range: VirtAddrRange, write: bool) -> AxResult;
}

/// The interface for checking user memory access.
#[def_interface]
pub trait AxPtrIf {
    fn lock_aspace() -> Box<dyn AddrSpaceGuard>;
}

fn lock_aspace() -> Box<dyn AddrSpaceGuard> {
    crate_interface::call_interface!(AxPtrIf::lock_aspace)
}

fn check_access<T: ?Sized>(ptr: *const T, write: bool) -> AxResult {
    let layout = unsafe { Layout::for_value_raw(ptr) };
    if !ptr.is_aligned_to(layout.align()) {
        return Err(AxError::BadAddress);
    }

    let range = VirtAddrRange::from_start_size(ptr.addr().into(), layout.size());
    lock_aspace().access_range(range, write)?;
    Ok(())
}

mod thin;
pub use thin::{UserMutPtr, UserPtr};

mod slice;
pub use slice::{
    UserMutSlicePtr, UserSlicePtr, cstr_until_nul, slice_until_nul, slice_until_nul_mut,
};
