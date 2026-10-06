// original: 0x00b2b370 obj_record_rebuild

/// Rebuilds an object's record through the rebuild callee, then commits.
///
/// Releases the record, then calls the rebuild callee with the object's
/// inner pointer, the clock, `arg1`, the record parts, a parameter block
/// mirroring the incoming arguments, two copies of the big-float constant
/// 999999.875, the flag bit derived from the byte at +0xE73 (inverted bit 0
/// of that byte shifted right by 1) and `arg2`. After two more release
/// calls, calls the probe callee with an out-word for the status: when the
/// status (compared signed) is below 2, tail-calls the record refresh with
/// the object and returns its answer with the low byte set; otherwise calls
/// the settle callee and returns its answer with the low byte cleared.
/// Cdecl, three stack words: object, buffer, flag.
lf_checker_rt::export!(cdecl, rw_00b2b370(obj: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const BIGF: u32 = 0x497423FE; // 999999.875
        const CLOCK: u32 = 0x010330DC;
        const REBUILD_OBJ: u32 = 0x01177A80;
        const RELEASE: u32 = 0;
        const REBUILD: u32 = 1;
        const REL2: u32 = 2;
        const REL3: u32 = 3;
        const PROBE: u32 = 4;
        const SETTLE: u32 = 5;
        const REFRESH: u32 = 6;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        lf_checker_rt::callee_thiscall!(RELEASE, u32, obj + 0xDD4);
        let g = rd32(lf_checker_rt::relocated(CLOCK));
        let b = ((obj + 0xE73) as *const u8).read();
        let c = ((!((b >> 1) as u8)) & 1) as u32;
        let inner = rd32(obj + 0x20);
        // The original passes a pointer to its own incoming argument slots;
        // the callee sees [obj, arg1, arg2], mirrored here.
        let params = [obj, arg1, arg2];
        let bp = params.as_ptr() as u32;
        lf_checker_rt::callee_thiscall!(
            REBUILD, u32, lf_checker_rt::relocated(REBUILD_OBJ),
            inner.wrapping_add(0x30), g, arg1, obj + 0xDDC, bp, 0xC, 0, BIGF,
            0, BIGF, c, g, 0, arg2, 0, 0, 0, 0
        );
        lf_checker_rt::callee_thiscall!(REL2, u32, obj + 0xDD4, 0);
        lf_checker_rt::callee_thiscall!(REL3, u32, obj + 0xDD4, 0);
        // The original passes its arg0 slot as the status out-word; it holds
        // the object until the probe overwrites it.
        let mut status = obj;
        lf_checker_rt::callee_thiscall!(
            PROBE, u32, lf_checker_rt::relocated(REBUILD_OBJ),
            inner.wrapping_add(0x30), obj + 0xDD4, &mut status as *mut u32 as u32, 2
        );
        if (status as i32) < 2 {
            let ans = lf_checker_rt::callee_cdecl!(REFRESH, u32, obj);
            (ans & 0xFFFF_FF00) | 1
        } else {
            let ans = lf_checker_rt::callee_thiscall!(SETTLE, u32, obj + 0xDD4);
            ans & 0xFFFF_FF00
        }
    }
});
