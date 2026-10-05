// original: 0x008C6F90 stream_backend_select
/// Select and arm the streaming backend for the current request state.
///
/// State 0 arms the primary backend (ready check, handle check, link
/// checks), records state 1 and reports 0; state 1 probes the fallback
/// chain and reports 0, 1 or 2 by how far the chain gets; any other state
/// reports 0 at once. Each probe is one no-argument callee. Only small
/// integer results occur. Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c6f90() -> u32 {
    unsafe {
        const READY_CALLEE: u32 = 1;
        const LINK_CALLEE: u32 = 2;
        const CHAIN_CALLEE: u32 = 3;
        const PROBE_CALLEE: u32 = 4;
        const STATE: u32 = 0x1173544;
        const ARMED_FLAG: u32 = 0x1031F2D;
        const HANDLE_SLOT: u32 = 0x118F4A8;
        match *lf_checker_rt::global::<u32>(STATE) {
            0 => {
                *lf_checker_rt::global::<u8>(ARMED_FLAG) = 1;
                let ready: u32 =
                    lf_checker_rt::callee_cdecl!(READY_CALLEE, u32,);
                if ready == 0 {
                    return 1;
                }
                let handle = *lf_checker_rt::global::<u32>(HANDLE_SLOT);
                if ((handle + 0x10) as *const u8).read() == 0 {
                    return 1;
                }
                let link: u32 =
                    lf_checker_rt::callee_cdecl!(LINK_CALLEE, u32,);
                if link == 0 {
                    let chain: u32 =
                        lf_checker_rt::callee_cdecl!(CHAIN_CALLEE, u32,);
                    if chain == 0 {
                        return 1;
                    }
                }
                *lf_checker_rt::global::<u32>(STATE) = 1;
                0
            }
            1 => {
                *lf_checker_rt::global::<u8>(ARMED_FLAG) = 0;
                let probe: u32 =
                    lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32,);
                if probe != 0 {
                    return 1;
                }
                let link: u32 =
                    lf_checker_rt::callee_cdecl!(LINK_CALLEE, u32,);
                if link == 0 {
                    let chain: u32 =
                        lf_checker_rt::callee_cdecl!(CHAIN_CALLEE, u32,);
                    if chain == 0 {
                        return 1;
                    }
                }
                let tail: u32 =
                    lf_checker_rt::callee_cdecl!(CHAIN_CALLEE, u32,);
                if tail == 0 {
                    0
                } else {
                    2
                }
            }
            _ => 0,
        }
    }
});
