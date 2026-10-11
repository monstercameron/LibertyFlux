// original: 0x00D1D2F0 ped_task_group_dispatch
/// Visits four signed-counted arrays of task records. A nonpositive count
/// skips that array; each nonzero flag byte causes a thiscall to the record
/// at its group-specific base and stride, forwarding `owner` as the argument.
/// The function has a thiscall receiver and one callee-cleaned stack word.
lf_checker_rt::export!(thiscall, rw_00d1d2f0(this: u32, owner: u32) -> u32 {
    const _RECORD_LAYOUT_NOTE: u32 = 0x90;
    let count_0 = unsafe { (this.wrapping_add(0x13a0) as *const i32).read_unaligned() };
    let mut index_0 = 0i32;
    while index_0 < count_0 {
        let enabled = unsafe { (this.wrapping_add(0x13b0).wrapping_add(index_0 as u32) as *const u8).read_unaligned() };
        if enabled != 0 {
            let record = this.wrapping_add(0x90).wrapping_add((index_0 as u32).wrapping_mul(0xb0));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, record, owner);
        }
        index_0 += 1;
    }
    let count_1 = unsafe { (this.wrapping_add(0x13a4) as *const i32).read_unaligned() };
    let mut index_1 = 0i32;
    while index_1 < count_1 {
        let enabled = unsafe { (this.wrapping_add(0x13bb).wrapping_add(index_1 as u32) as *const u8).read_unaligned() };
        if enabled != 0 {
            let record = this.wrapping_add(0x820).wrapping_add((index_1 as u32).wrapping_mul(0xc0));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, record, owner);
        }
        index_1 += 1;
    }
    let count_2 = unsafe { (this.wrapping_add(0x13a8) as *const i32).read_unaligned() };
    let mut index_2 = 0i32;
    while index_2 < count_2 {
        let enabled = unsafe { (this.wrapping_add(0x13bc).wrapping_add(index_2 as u32) as *const u8).read_unaligned() };
        if enabled != 0 {
            let record = this.wrapping_add(0x8e0).wrapping_add((index_2 as u32).wrapping_mul(0xe0));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, record, owner);
        }
        index_2 += 1;
    }
    let count_3 = unsafe { (this.wrapping_add(0x13ac) as *const i32).read_unaligned() };
    let mut index_3 = 0i32;
    while index_3 < count_3 {
        let enabled = unsafe { (this.wrapping_add(0x13c7).wrapping_add(index_3 as u32) as *const u8).read_unaligned() };
        if enabled != 0 {
            let record = this.wrapping_add(0x1280).wrapping_add((index_3 as u32).wrapping_mul(0x120));
            let _ = lf_checker_rt::callee_thiscall!(1, u32, record, owner);
        }
        index_3 += 1;
    }
    0
});
