// original: 0x0093e480 stream_request_emit (proposed)

/// Emit a streaming request through the request callee.
///
/// Reads mode bytes and the level word from the globals, builds the
/// callee's scratch frame (the level float at -16, zeroes at +0/+4/+8)
/// and calls the request callee with (`arg`, mode1, mode0, 1, table,
/// entry-tag, frame, aux, flag). The entry tag is whatever the caller
/// left in `ecx`, forwarded untouched (in the real game this function
/// runs as a slot-set callback, so the tag is the set pointer arriving
/// as register residue; it cannot be read from Rust, so the proof skips
/// that call argument and the rewrite passes zero there). Returns 1 in
/// `al` (only the low byte is set, so only `al` is compared).
///
/// Original: 0x0093e480 (cdecl, one stack word; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e480(arg: u32) -> u32 {
    const REQUEST: u32 = 1;
    const MODE0: u32 = 0x11A4FB4;
    const MODE1: u32 = 0x11A4FB5;
    const FLAG: u32 = 0x11A4FB6;
    const AUX: u32 = 0x1036F0C;
    const LEVEL: u32 = 0x11A4FB0;
    // File address of the table, relocated like the original's pushed
    // immediate (the worker's reloc pass covers it).
    const TABLE_FILE: u32 = 0x11D4E40;
    unsafe {
        let flag = (lf_checker_rt::global::<u8>(FLAG) as *const u8).read() as u32;
        let aux = (lf_checker_rt::global::<u8>(AUX) as *const u8).read() as u32;
        let level = (lf_checker_rt::global::<u32>(LEVEL) as *const u32).read_unaligned();
        let mode0 = (lf_checker_rt::global::<u8>(MODE0) as *const u8).read() as u32;
        let mode1 = (lf_checker_rt::global::<u8>(MODE1) as *const u8).read() as u32;
        // Scratch frame as the original lays it out: the level float 16
        // bytes below the passed pointer, zeroes at +0/+4/+8. (Words -12
        // to -4 are uninitialized scratch on both sides and stay
        // unobserved.)
        let frame: [u32; 7] = [level, 0, 0, 0, 0, 0, 0];
        let frame_ptr = frame.as_ptr().add(4) as u32;
        lf_checker_rt::callee_cdecl!(
            REQUEST, u32, arg, mode1, mode0, 1u32,
            lf_checker_rt::relocated(TABLE_FILE), 0u32, frame_ptr, aux, flag
        );
        1
    }
});
