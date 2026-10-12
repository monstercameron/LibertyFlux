// original: 0x00E6EFA0 network_mark_then_clear_nav_request

/// Marks the pending navigation request, then clears the global request slot.
///
/// The cdecl function reads one 32-bit request pointer from the image data.
/// For a nonzero request it calls the navigation marker with that pointer and
/// returns the marker's EAX value. For a zero request it skips the call and
/// returns zero. The global slot is cleared on either path after the optional
/// call. The checker contract intercepts the direct callee and supplies its
/// return value.
lf_checker_rt::export!(cdecl, rw_00e6efa0() -> u32 {
    const NAV_REQUEST_SLOT: u32 = 0x019E_8614;
    const NAV_REQUEST_MARKER: u32 = 1;

    // SAFETY: the slot is writable image data that this function reads and
    // clears; its value is part of the function's global state.
    unsafe {
        let request_slot = lf_checker_rt::global::<u32>(NAV_REQUEST_SLOT);
        let request = request_slot.read_unaligned();
        let result = if request == 0 {
            0
        } else {
            lf_checker_rt::callee_cdecl!(NAV_REQUEST_MARKER, u32, request)
        };
        request_slot.write_unaligned(0);
        result
    }
});
