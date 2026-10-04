// original: 0x00B2A310 PlaybackRecordingSlotSetup
/// Set up one playback recording slot (original 0x00B2A310).
///
/// Resolves the recording id, wakes the source object through its status
/// and command calls, then claims a slot: the requested index, or the
/// first free entry of the occupancy table for a negative request.
/// Publishes the slot's record fields (owner, flags, counters, pose copy),
/// marks it busy, and for a claimed-by-scan slot with a live pose quietly
/// finishes; otherwise it drives the mission task through its constructor
/// and setup calls and stamps the slot index into the record.
export!(cdecl, rw_b2a310(obj: u32, idw: u32, flagw: u32, bytew: u32, slot: u32, cfg: u32) -> u32 {
    const PROBE_SLOT: u32 = 0x128;
    const CMD_SLOT: u32 = 0x134;
    const OBJ_VT_OFF: usize = 0x00;
    const OBJ_ANCHOR_OFF: usize = 0x20;
    const ANCHOR_DELTA: u32 = 0x30;
    const POOL_OFF: usize = 0x224;
    const HEAD_DELTA: u32 = 0x44;
    const LINK_OFF: usize = 0xF50;
    const STAMP_OFF: usize = 0xF10;
    const MAX_SLOTS: u32 = 0x18;
    const ONE_F: u32 = 0x3F800000;
    const NEG_ONE_F: u32 = 0xBF800000;
    const T_OCC: u32 = 0x0165_7890;
    const T_OWNER: u32 = 0x0165_79B0;
    const T_FLAG: u32 = 0x0165_78A8;
    const T_CTR: u32 = 0x0165_7710;
    const T_REC_A: u32 = 0x0165_7A14;
    const T_REC_B: u32 = 0x0165_7A1C;
    const T_REC_C: u32 = 0x0165_7A18;
    const T_OUT_A: u32 = 0x0165_76B0;
    const T_OUT_B: u32 = 0x0165_77D0;
    const T_OUT_C: u32 = 0x0165_7830;
    const T_OUT_D: u32 = 0x0165_7770;
    const T_POSE: u32 = 0x0165_FBA0;
    const T_DONE: u32 = 0x0165_78C0;
    const T_PBYTE: u32 = 0x0165_78D8;
    const T_SLOT: u32 = 0x0165_7650;
    const DRIVER_CELL: u32 = 0x0167_E2A0;
    unsafe {
        let rec = callee_cdecl!(1, u32, idw);
        if obj != 0 {
            let vt = (obj as *const u32).byte_add(OBJ_VT_OFF).read();
            let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt.wrapping_add(PROBE_SLOT)) as *const u32).read() as usize,
            );
            if probe(obj) & 0xFF != 0 {
                let cmd: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(
                        ((vt.wrapping_add(CMD_SLOT)) as *const u32).read() as usize,
                    );
                cmd(obj, 1, 1, 0);
            }
        }
        let want = slot as i32;
        let mut bx: u32 = 0;
        if want >= 0 {
            bx = slot;
        } else {
            loop {
                if (relocated(T_OCC) as *const u8).add(bx as usize).read() == 0 {
                    break;
                }
                bx += 1;
                if bx >= MAX_SLOTS {
                    break;
                }
            }
        }
        // NOTE: bx == MAX_SLOTS (table full) is excluded from the contract:
        // the original calls its fatal-error routine there and never
        // returns (falls into padding traps). Trials always leave room.
        callee_cdecl!(4, u32, bx, obj);
        let b0b = bytew as u8;
        (relocated(T_OWNER) as *mut u32).add(bx as usize).write(rec);
        (relocated(T_FLAG) as *mut u8).add(bx as usize).write(b0b);
        let rx = rec.wrapping_mul(16) as usize;
        (relocated(T_CTR) as *mut u32).add(bx as usize).write(0);
        let va = (relocated(T_REC_A) as *const u32).byte_add(rx).read();
        let cnt = (relocated(T_REC_B) as *mut u8).byte_add(rx);
        cnt.write(cnt.read().wrapping_add(1));
        (relocated(T_OUT_A) as *mut u32).add(bx as usize).write(va);
        let vc = (relocated(T_REC_C) as *const u32).byte_add(rx).read();
        (relocated(T_OUT_B) as *mut u32).add(bx as usize).write(0);
        (relocated(T_OUT_C) as *mut u32).add(bx as usize).write(ONE_F);
        (relocated(T_OUT_D) as *mut u32).add(bx as usize).write(vc);
        let c0 = (cfg as *const u32).read();
        let c1 = (cfg as *const u32).add(1).read();
        let c2 = (cfg as *const u32).add(2).read();
        let dx = bx.wrapping_mul(16) as usize;
        (relocated(T_POSE) as *mut u32).byte_add(dx).write(c0);
        (relocated(T_POSE) as *mut u32).byte_add(dx + 4).write(c1);
        (relocated(T_POSE) as *mut u32).byte_add(dx + 8).write(c2);
        let c3 = (cfg as *const u32).add(3).read();
        (relocated(T_POSE) as *mut u32).byte_add(dx + 12).write(c3);
        let r5 = callee_thiscall!(5, u32, obj);
        (relocated(T_OCC) as *mut u8).add(bx as usize).write(1);
        let cfgbyte = flagw as u8;
        (relocated(T_DONE) as *mut u8).add(bx as usize).write(0);
        (relocated(T_PBYTE) as *mut u8).add(bx as usize).write(cfgbyte);
        if want >= 0 {
            // Exit value on this path is the callee's answer with its low
            // byte replaced; reproduce it exactly (deterministic: same
            // scripted answer on both sides).
            return (r5 & 0xFFFF_FF00) | u32::from(cfgbyte);
        }
        if cfgbyte != 0 {
            let tab = (relocated(T_SLOT) as *const u32).add(bx as usize).read();
            let ebp2 = (tab as *const u32).byte_add(LINK_OFF).read();
            let r6 = callee_thiscall!(6, u32, global::<u32>(DRIVER_CELL).read());
            let r7 = if r6 != 0 {
                callee_thiscall!(
                    7, u32, r6, tab, 0, 0xF, 2, 0x14, 0, 0, 0, 0, 5, 0xA,
                    NEG_ONE_F, 0x1E, 0x14, 1
                )
            } else {
                0
            };
            let pool = (ebp2 as *const u32).byte_add(POOL_OFF).read();
            callee_thiscall!(8, u32, pool.wrapping_add(HEAD_DELTA), r7, 3, 0);
            let av = (obj as *const u32).byte_add(OBJ_ANCHOR_OFF).read();
            callee_cdecl!(9, u32, bx, av.wrapping_add(ANCHOR_DELTA));
        }
        let tab = (relocated(T_SLOT) as *const u32).add(bx as usize).read();
        (tab as *mut u8).byte_add(STAMP_OFF).write(bx as u8);
        tab
    }
});
