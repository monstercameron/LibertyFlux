// original: 0x00a11da0 occupant_flag_sweep (proposed)
/// Sweep occupants and flag those eligible for the marker.
///
/// Does nothing when the byte at `obj + 0x73` is clear. Otherwise marks
/// `obj + 0x70`, fetches the occupant list head through the virtual slot at
/// `+0x2c` of `item`, and when its word at `+0x38` is non-negative, walks
/// the `count` occupants: a null occupant is skipped; otherwise the pair
/// gate runs on (identity, global), and when `item` carries flag 0x400000
/// at `+0x8e8` and the gate's low byte is zero, the register gate
/// runs and a non-zero answer from the final check stores 0xff at occupant
/// `+0x63`. No return value is set. Cdecl, two stack arguments.
export!(cdecl, rw_00a11da0(obj: u32, item: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 11;
        const FETCH: u32 = 12;
        const PAIR_GATE: u32 = 13;
        const REG_GATE: u32 = 14;
        const FINAL: u32 = 15;
        const ENABLE_OFF: u32 = 0x73;
        const MARK_OFF: u32 = 0x70;
        const VTABLE_SLOT: u32 = 0x2c;
        const HEAD_COUNT_OFF: u32 = 0x38;
        const HEAD_FLAG_OFF: u32 = 0x8e8;
        const HEAD_FLAG_BIT: u32 = 0x400000;
        const ID_OFF: u32 = 0x2e;
        const MARK_BYTE_OFF: u32 = 0x63;
        const SHARED_GLOBAL: u32 = 0x012b4138;
        const FINAL_THIS: u32 = 0x0128e400;
        if ((obj + ENABLE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        ((obj + MARK_OFF) as *mut u8).write(1);
        let vt = (item as *const u32).read_unaligned();
        let slot = ((vt + VTABLE_SLOT) as *const u32).read_unaligned();
        let head_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let head = head_fn(item);
        let n = ((head + HEAD_COUNT_OFF) as *const u32).read_unaligned() as i32;
        if n < 0 {
            return 0;
        }
        let count = callee_thiscall!(COUNT, u32, obj, n as u32);
        let shared = *global::<u32>(SHARED_GLOBAL);
        for i in 0..count {
            let occ = callee_thiscall!(
                FETCH,
                u32,
                obj,
                ((head + HEAD_COUNT_OFF) as *const u32).read_unaligned(),
                i
            );
            if occ == 0 {
                continue;
            }
            let id = ((occ + ID_OFF) as *const u16).read_unaligned() as i16 as i32 as u32;
            let g = callee_cdecl!(PAIR_GATE, u32, id, shared);
            if ((item + HEAD_FLAG_OFF) as *const u32).read_unaligned() & HEAD_FLAG_BIT
                == 0
            {
                continue;
            }
            if (g & 0xff) != 0 {
                continue;
            }
            callee_cdecl!(REG_GATE, u32, id, shared, 0);
            if (callee_thiscall!(FINAL, u32, relocated(FINAL_THIS)) & 0xff) != 0 {
                ((occ + MARK_BYTE_OFF) as *mut u8).write(0xff);
            }
        }
        0
    }
});
