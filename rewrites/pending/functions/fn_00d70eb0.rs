// original: 0x00D70EB0 replay_input_poll (proposed)
/// Poll the replay input pad and advance the mark list when the trigger chord
/// is held.
///
/// Reads three masked pad bytes: unless the first mask clears the chord bit
/// while the second sets it, returns with the pad address in `eax`. Then
/// requires a non-empty mark list and a live cursor. Marks the owner active
/// and takes one of two paths: a single-entry list is consumed (notifying the
/// watchers, clearing the cursor and publishing the shared state), while a
/// longer list resolves the cursor's entry and, when the pending query
/// confirms it, records the entry's stamp delta into the shared cells before
/// the same notification tail. Every early exit returns the pointer it was
/// examining; the consume path returns the publish call's answer.
export!(thiscall, rw_d70eb0(this: u32) -> u32 {
    const SUB_OFF: usize = 0x04;
    const LIST_OFF: usize = 0x9C;
    const CURSOR_OFF: usize = 0xA0;
    const ACTIVE_OFF: usize = 0x18;
    const CLEAR_B: usize = 0xF8;
    const CLEAR_C: usize = 0xFC;
    const PAD_MASK: usize = 0x2FEC;
    const PAD_B: usize = 0x2FEE;
    const PAD_C: usize = 0x2FEF;
    const CHORD_BIT: u8 = 0x7F;
    const SHARED_THIS: u32 = 0x0103_E498;
    const SHARED_SLOT: usize = 0x38C;
    const FLAG_CELL: u32 = 0x011F_70E0;
    const ANCHOR_CELL: u32 = 0x011F_7028;
    const DELTA_CELL: u32 = 0x011F_70E4;
    unsafe {
        let pad = callee_cdecl!(1, u32, 1);
        let mask = (pad as *const u8).byte_add(PAD_MASK).read();
        let rb = (pad as *const u8).byte_add(PAD_B).read() ^ mask;
        if rb <= CHORD_BIT {
            return (pad & 0xFFFF_FF00) | u32::from(rb);
        }
        let rc = (pad as *const u8).byte_add(PAD_C).read() ^ mask;
        if rc > CHORD_BIT {
            return (pad & 0xFFFF_FF00) | u32::from(rc);
        }
        let list = (this as *const u32).byte_add(LIST_OFF).read();
        if (list as *const u16).byte_add(4).read() == 0 {
            return list;
        }
        let cursor_ptr = (this as *mut u32).byte_add(CURSOR_OFF);
        if (cursor_ptr as *const i32).read() < 0 {
            return list;
        }
        let sub = (this as *const u32).byte_add(SUB_OFF).read();
        (sub as *mut u8).byte_add(ACTIVE_OFF).write(1);
        let base = (list as *const u32).read();
        if (list as *const u16).byte_add(4).read() != 1 {
            // Longer list: resolve the cursor's entry.
            let idx = (cursor_ptr as *const i32).read();
            let entry_ptr = base.wrapping_add((idx as u32).wrapping_mul(4));
            let entry = (entry_ptr as *const u32).read();
            if idx > 0 {
                let pending = callee_thiscall!(9, u32, this);
                if pending != 0
                    && (pending as *const u8).read().wrapping_sub(1) == 8
                    && (pending as *const u32).byte_add(0x1C).read()
                        <= (entry as *const u32).byte_add(0x14).read()
                {
                    let n = callee_cdecl!(10, u32,);
                    let stamp = callee_cdecl!(11, u32, n.wrapping_sub(1));
                    // The original copies 32 stamp bytes into scratch and
                    // keeps only the word at +12; that word alone is observed.
                    let first = (stamp as *const u32).byte_add(12).read();
                    callee_cdecl!(12, u32, 0x0E);
                    let flag = global::<u32>(FLAG_CELL);
                    flag.write(flag.read() | 1);
                    let delta = first.wrapping_sub(global::<u32>(ANCHOR_CELL).read());
                    global::<u32>(DELTA_CELL).write(delta);
                }
            }
            callee_thiscall!(3, u32, list, entry_ptr);
            callee_cdecl!(2, u32, entry);
            if (cursor_ptr as *const i32).read() > 0 {
                cursor_ptr.write(0xFFFF_FFFF);
                (this as *mut u32).byte_add(CLEAR_B).write(0xFFFF_FFFF);
                (this as *mut u32).byte_add(CLEAR_C).write(0xFFFF_FFFF);
            }
        } else {
            // Single entry: consume it and publish the shared state.
            callee_cdecl!(2, u32, (base as *const u32).read());
            callee_thiscall!(3, u32, list, (list as *const u32).read());
            callee_thiscall!(4, u32, sub, 0);
            cursor_ptr.write(0xFFFF_FFFF);
            (this as *mut u32).byte_add(CLEAR_B).write(0xFFFF_FFFF);
            (this as *mut u32).byte_add(CLEAR_C).write(0xFFFF_FFFF);
            let ok = callee_thiscall!(5, u32, sub);
            if ok & 0xFF != 0 {
                callee_thiscall!(4, u32, sub, 0);
            }
            let shared = callee_thiscall!(6, u32, relocated(SHARED_THIS));
            (shared as *mut u32).byte_add(SHARED_SLOT).write(7);
            callee_cdecl!(7, u32,);
            return callee_cdecl!(8, u32, 0);
        }
        let ok = callee_thiscall!(5, u32, sub);
        if ok & 0xFF != 0 {
            callee_thiscall!(4, u32, sub, 0)
        } else {
            ok
        }
    }
});
