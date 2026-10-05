// original: 0x00e5bfd0 notify_slot_then_dispatch_e5bfd0

/// Rewrite of a notify-then-dispatch stub: calls the shared table slot
/// with one constant data address, then forwards one constant code
/// address to the shared single-argument callee and returns its result.
///
/// Original shape: `push; (an instruction of the original); push; call rel32; (an instruction of the original); ret`.
/// The table slot below is the shared indirect-call slot (file VA).
export!(cdecl, rw_00e5bfd0() -> u32 {
    unsafe {
        let target = *(global::<u32>(0x00E731C4) as *const u32);
        let notify: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        notify(relocated(0x018DC210));
        callee_cdecl!(2, u32, relocated(0x00E6E290))
    }
});
