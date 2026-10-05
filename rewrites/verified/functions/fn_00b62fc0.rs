// original: 0x00B62FC0 veh_lookup_or_null
/// Resolve the model-table entry for this object, or null when disabled.
///
/// Returns 0 when the enable byte at `[this+0x24]` is clear. Otherwise resolves
/// the handle (stubbed, cdecl/1) from `[this+0x18]`, asks it for the model index
/// (stubbed, thiscall/0), returns 0 for a negative index, else the table entry
/// at `0x1295CD8[index]`. The table read uses an unrelocated absolute address,
/// so trials only cover the null paths (scripted index always negative).
/// Thiscall, no stack words; entry registers except ECX are ignored.
export!(thiscall, rw_00b62fc0(this: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x18;
        const FLAG: u32 = 0x24;
        const TABLE: u32 = 0x1295CD8;
        if ((this + FLAG) as *const u8).read() == 0 {
            return 0;
        }
        let h: u32 = callee_cdecl!(1, u32, ((this + HANDLE) as *const u32).read_unaligned());
        let idx: u32 = callee_thiscall!(2, u32, h);
        if (idx as i32) <= -1 {
            return 0;
        }
        *global::<u32>(TABLE.wrapping_add(idx.wrapping_mul(4)))
    }
});
