// original: 0x00CB33C0 CTaskComplexGetOutOfWater::vf18

/// Restart the get-out-of-water subtask once it reports leaving the water.
///
/// `this` is the complex task, `ped` the ped. The current subtask's type
/// (virtual slot 3, callee 1) decides: type 0xcb means the ped started
/// leaving the water, in which case the cached wading data (`this+0x30`)
/// is released (callee 2, one word, caller cleans up), the slot is
/// cleared, and a fresh leave-water subtask (id 0xcb) is created
/// (callee 3) and returned. A null cache slot, type 0xca, and every other
/// type keep the current subtask (null).
///
/// Original: 0x00CB33C0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB33C0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 8;
        const GET_TYPE_SLOT: u32 = 0x0c;
        const RELEASE_DATA: u32 = 2;
        const CREATE_SUB: u32 = 3;
        const WADING_SLOT: u32 = 0x30;
        const IN_WATER: u32 = 0xca;
        const LEAVING_WATER: u32 = 0xcb;
        let sub = (this.wrapping_add(SUBTASK) as *const u32).read_unaligned();
        let vtable = (sub as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(GET_TYPE_SLOT) as *const u32).read_unaligned();
        let get_type: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let kind = get_type(sub);
        if kind == IN_WATER {
            return 0;
        }
        if kind != LEAVING_WATER {
            return 0;
        }
        let cached = (this.wrapping_add(WADING_SLOT) as *const u32).read_unaligned();
        if cached == 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(RELEASE_DATA, u32, cached);
        (this.wrapping_add(WADING_SLOT) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, ped, LEAVING_WATER)
    }
});
