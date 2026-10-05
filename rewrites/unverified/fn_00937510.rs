// original: 0x00937510 stream_global_scan_update (proposed)

/// Refresh the two streaming lanes and report the last visit's answer.
///
/// Opens the shared handle; when the biased base passes the ceiling the
/// fresh flag is set. Mode 2 arms lane work when nothing is held, nothing
/// is fresh and the ready flag stands; mode 1 additionally probes the
/// handle first. An open handle is stamped through the second probe.
/// Then each lane's state word is read: states 4 and 5 stay idle, an
/// unarmed pass stays idle, and lane 0 additionally needs its sentinel at
/// zero. Each lane is visited with (lane, flag) and the last answer wins.
lf_checker_rt::export!(cdecl, rw_00937510() -> u32 {
    unsafe {
        const OPEN: u32 = 1;
        const PROBE_A: u32 = 2;
        const PROBE_B: u32 = 3;
        const VISIT: u32 = 4;
        const BASE_GLOBAL: u32 = 0x1168258;
        const BASE_BIAS: u32 = 0x1388;
        const CEIL_GLOBAL: u32 = 0x11735B4;
        const MODE_GLOBAL: u32 = 0x11A2EA4;
        const HELD_FLAG: u32 = 0x11A2EA1;
        const READY_GLOBAL: u32 = 0x1160C94;
        const STAMP_OUT: u32 = 0x11A2EA9;
        const STATE_BASE: u32 = 0x11A4024;
        const ZERO_SENTINEL: u32 = 0x11A4DD4;
        const OBJ_BASE: u32 = 0x11A32C0;
        const OBJ_STRIDE: u32 = 0xDB0;
        const PROBE_OFF: u32 = 0x210;
        let g = |va: u32| -> u32 {
            lf_checker_rt::global::<u32>(va).read_unaligned()
        };
        let gb = |va: u32| -> u8 {
            lf_checker_rt::global::<u8>(va).read()
        };
        let h: u32 = lf_checker_rt::callee_cdecl!(OPEN, u32, 0);
        let fresh =
            u32::from(g(BASE_GLOBAL).wrapping_add(BASE_BIAS) > g(CEIL_GLOBAL));
        let mut armed: u8 = 0;
        let m = g(MODE_GLOBAL);
        if m == 2 {
            if gb(HELD_FLAG) == 0 && fresh == 0 && g(READY_GLOBAL) != 0 {
                armed = 1;
            }
        } else if m == 1 && h != 0 {
            let t: u32 =
                lf_checker_rt::callee_thiscall!(PROBE_A, u32, h.wrapping_add(PROBE_OFF));
            if (t & 0xFF) != 0 && gb(HELD_FLAG) == 0 && fresh == 0 && g(READY_GLOBAL) != 0
            {
                armed = 1;
            }
        }
        if h != 0 {
            let s: u32 =
                lf_checker_rt::callee_thiscall!(PROBE_B, u32, h.wrapping_add(PROBE_OFF));
            lf_checker_rt::global::<u8>(STAMP_OUT).write((s & 0xFF) as u8);
        }
        let mut answer: u32 = 0;
        for lane in 0..2u32 {
            let state = ((lf_checker_rt::relocated(STATE_BASE)
                + lane.wrapping_mul(OBJ_STRIDE))
                as *const u32)
                .read_unaligned();
            let mut flag: u32 = 0;
            if state != 5 && state != 4 && armed != 0 {
                if lane != 0 || g(ZERO_SENTINEL) == 0 {
                    flag = 1;
                }
            }
            let obj = lf_checker_rt::relocated(OBJ_BASE)
                .wrapping_add(lane.wrapping_mul(OBJ_STRIDE));
            answer = lf_checker_rt::callee_thiscall!(VISIT, u32, obj, lane, flag);
        }
        answer
    }
});
