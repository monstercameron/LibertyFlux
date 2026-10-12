// original: 0x008A5310 audifsound_start_active_branch

/// Start the active branch of an if-sound node.
///
/// When the state word `STATE` is 0, resolves child `CHILD_A` through the
/// audio table (slot `TABLE_BIAS + pool * ROW_BYTES` past `TABLE`, plus
/// `STRIDE * index`) and tail-calls the branch starter with the resolved
/// target, returning 0 (low byte) when the child is `NONE` or resolves to
/// null. When the state is 1, resolves child `CHILD_B` the same way and
/// calls the branch preselector with (1, arg) before calling the starter
/// with its result. Any other state returns 0. The second 0xFF check in the
/// state-0 path is dead (the byte is already proven different with no
/// intervening write), so the zero-target tail it guards is omitted. Only
/// the low byte of an early-out return is set. Thiscall, object in ECX,
/// one word popped from the stack.
lf_checker_rt::export!(thiscall, rw_008A5310(this: u32, arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0xC0;
        const POOL: u32 = 0x40;
        const CHILD_A: u32 = 0x48;
        const CHILD_B: u32 = 0x49;
        const STRIDE: u32 = 0x0115_D964;
        const TABLE: u32 = 0x0115_D988;
        const TABLE_BIAS: u32 = 0x6F10;
        const ROW_BYTES: u32 = 0x6F40;
        const NONE: u32 = 0xFF;
        let state = (this.wrapping_add(STATE) as *const u32).read_unaligned();
        if state == 0 {
            let child = (this.wrapping_add(CHILD_A) as *const u8).read_unaligned() as u32;
            if child != NONE {
                let pool = (this.wrapping_add(POOL) as *const u8).read_unaligned() as u32;
                let stride =
                    (lf_checker_rt::relocated(STRIDE) as *const u32).read_unaligned();
                let table = (lf_checker_rt::relocated(TABLE) as *const u32).read_unaligned();
                let slot = (table
                    .wrapping_add(TABLE_BIAS)
                    .wrapping_add(pool.wrapping_mul(ROW_BYTES))
                    as *const u32)
                    .read_unaligned();
                let target = slot.wrapping_add(stride.wrapping_mul(child));
                if target != 0 {
                    return lf_checker_rt::callee_thiscall!(3, u32, target);
                }
            }
            0
        } else if state == 1 {
            let child = (this.wrapping_add(CHILD_B) as *const u8).read_unaligned() as u32;
            if child != NONE {
                let pool = (this.wrapping_add(POOL) as *const u8).read_unaligned() as u32;
                let stride =
                    (lf_checker_rt::relocated(STRIDE) as *const u32).read_unaligned();
                let table = (lf_checker_rt::relocated(TABLE) as *const u32).read_unaligned();
                let slot = (table
                    .wrapping_add(TABLE_BIAS)
                    .wrapping_add(pool.wrapping_mul(ROW_BYTES))
                    as *const u32)
                    .read_unaligned();
                let target = slot.wrapping_add(stride.wrapping_mul(child));
                if target != 0 {
                    let mid = lf_checker_rt::callee_thiscall!(1, u32, this, 1, arg);
                    return lf_checker_rt::callee_thiscall!(2, u32, mid);
                }
            }
            0
        } else {
            0
        }
    }
});
