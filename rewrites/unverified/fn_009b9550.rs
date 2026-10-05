// original: 0x009B9550 CCamScriptInstruction_AddPedToCinematographyAI::vf2

/// Execute the AddPedToCinematographyAI script instruction: look the target object up through
/// the camera manager singleton (callee id 1, thiscall with the index at `this+0x08`), then update it; returns early when the lookup misses (null answer).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9550(this: u32) -> u32 {
    unsafe {
        // Append the operand to the object's cinematography-ped list: store
        // at LIST[count], then increment the COUNT word in memory. The
        // increment re-reads the word: COUNT sits at LIST[2]'s address
        // (+0x148 = +0x140 + 2*4), so when count is 2 the slot store
        // overwrites COUNT first and the increment applies to the stored
        // operand, not to count + 1.
        const MGR: u32 = 0x0128E400;
        const FIELD_INDEX: u32 = 0x08;
        const FIELD_PED: u32 = 0x0c;
        const OBJ_LIST: u32 = 0x140;
        const OBJ_COUNT: u32 = 0x148;
        const LOOKUP: u32 = 1;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR), index);
        if obj != 0 {
            let count = ((obj + OBJ_COUNT) as *const u32).read_unaligned();
            let ped = ((this + FIELD_PED) as *const u32).read_unaligned();
            ((obj + OBJ_LIST + count.wrapping_mul(4)) as *mut u32)
                .write_unaligned(ped);
            let cell = (obj + OBJ_COUNT) as *mut u32;
            cell.write_unaligned(cell.read_unaligned().wrapping_add(1));
        }
        0
    }
});
