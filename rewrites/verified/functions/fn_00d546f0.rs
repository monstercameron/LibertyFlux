// original: 0x00d546f0 ccam_dispatch_via_registry

/// Dispatch a camera request through the registry, or through the fallback
/// chain.
///
/// `arg` is an opaque request word. The registry fetch runs first; when it
/// returns non-null the request is dispatched to slot `VSLOT` of that
/// object's table with (`arg`, 0, 0) and the answer is returned. Otherwise
/// the fallback fetch runs: a null answer returns 0, and a live one is
/// probed with tag `PROBE_TAG`. A live probe answer whose word at `MID_LINK`
/// is also live gets the same three-word dispatch first; then the fallback
/// object itself always gets it, and that answer is returned. Entry ECX is
/// caller residue (this is a stdcall): it is pushed and overwritten twice
/// and never read, so the callees that take no stack arguments are declared
/// cdecl and ECX is not compared.
///
/// Original: 0x00d546f0 (stdcall, one stack argument, up to four calls).
lf_checker_rt::export!(stdcall, rw_00d546f0(arg: u32) -> u32 {
    unsafe {
        /// Virtual slot dispatched to (three stack words).
        const VSLOT: u32 = 0x7c;
        /// Peer link read from the fallback object for the probe call.
        const PEER_LINK: u32 = 0x224;
        /// Adjustment added to the peer link.
        const PEER_ADJ: u32 = 0x44;
        /// Link read from the probe answer for the middle dispatch.
        const MID_LINK: u32 = 0x14;
        /// Tag passed to the probe call.
        const PROBE_TAG: u32 = 0x13b;
        /// Registry fetch (intercepted; cdecl, one stack argument).
        const FETCH_REG: u32 = 1;
        /// Fallback fetch (intercepted; cdecl, no arguments).
        const FETCH_FB: u32 = 3;
        /// Fallback probe (intercepted; thiscall, one stack argument).
        const PROBE: u32 = 4;
        #[inline(always)]
        unsafe fn vcall(obj: u32, arg: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let tgt = (vt.wrapping_add(VSLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(obj, arg, 0, 0)
            }
        }
        let reg = lf_checker_rt::callee_cdecl!(FETCH_REG, u32, 0);
        if reg != 0 {
            return vcall(reg, arg);
        }
        let fb = lf_checker_rt::callee_cdecl!(FETCH_FB, u32,);
        if fb == 0 {
            return 0;
        }
        let peer = ((fb.wrapping_add(PEER_LINK)) as *const u32)
            .read_unaligned()
            .wrapping_add(PEER_ADJ);
        let probe = lf_checker_rt::callee_thiscall!(PROBE, u32, peer, PROBE_TAG);
        if probe != 0 {
            let mid = ((probe.wrapping_add(MID_LINK)) as *const u32).read_unaligned();
            if mid != 0 {
                vcall(mid, arg);
            }
        }
        vcall(fb, arg)
    }
});
