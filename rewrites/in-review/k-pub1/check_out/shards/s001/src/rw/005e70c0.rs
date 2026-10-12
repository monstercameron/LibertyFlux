// original: 0x005e70c0 CELL_CAM_SET_CENTRE_POS
// CELL_CAM_SET_CENTRE_POS: store the two float args in globals, find a free
// camera slot through the camera pool (thiscall/0 on the pool singleton),
// resolve its object (thiscall/1) and store the args at +0x944/+0x948.
// Bails out when the pool is full (-1) or the slot is empty (null). The x
// coordinate is also spilled over the incoming argument slot, as the
// original does.
export!(cdecl, rw_005e70c0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let x = *a;
        let y = *a.add(1);
        *arg_slot_of(ctx) = x;
        *global::<u32>(0x110E708) = x;
        *global::<u32>(0x110E70C) = y;
        let pool = relocated(0x161547C);
        let find_free: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(__lf_callee(1) as usize);
        let idx = find_free(pool);
        if idx == 0xFFFF_FFFF {
            return idx;
        }
        let slot_obj: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(__lf_callee(2) as usize);
        let obj = slot_obj(pool, idx);
        if obj == 0 {
            return obj;
        }
        *((obj + 0x944) as *mut u32) = x;
        *((obj + 0x948) as *mut u32) = y;
        obj
    }
});

/// Argument array of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn args_of(ctx: u32) -> *const u32 {
    *((ctx as *const u32).add(2)) as *const u32
}

/// Address of this call's incoming argument slot, passed by the harness in
/// `ctx+0xC`. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn arg_slot_of(ctx: u32) -> *mut u32 {
    *((ctx as *const u32).add(3)) as *mut u32
}
