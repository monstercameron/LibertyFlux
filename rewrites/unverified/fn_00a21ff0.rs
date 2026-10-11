// original: 0x00A21FF0 ped_task_update_enabled_records

/// Invoke the record helper for each enabled entry in four task record sets.
///
/// The object stores signed record counts at `+0x400..+0x40c` and one-byte
/// enable flags at `+0x410..+0x413`. The four arrays begin at `+0x90`,
/// `+0x140`, `+0x200` and `+0x2e0`, with strides `0xb0`, `0xc0`, `0xe0`
/// and `0x120` bytes. Counts use signed comparisons; nonpositive counts visit
/// no records. Each nonzero flag calls the intercepted thiscall helper with
/// that record as `this` and forwards all three stack arguments in order.
/// The callee's proved early-return path preserves EAX, ECX and EDX, and the
/// caller restores the second outer argument between the first two sets, so
/// the function returns that argument. The original thiscall callee removes
/// its three stack arguments before returning.
lf_checker_rt::export!(thiscall, rw_00A21FF0(this: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const COUNTS: [u32; 4] = [0x400, 0x404, 0x408, 0x40c];
        const FLAGS: [u32; 4] = [0x410, 0x411, 0x412, 0x413];
        const RECORD_BASES: [u32; 4] = [0x90, 0x140, 0x200, 0x2e0];
        const STRIDES: [u32; 4] = [0xb0, 0xc0, 0xe0, 0x120];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_i32(address: u32) -> i32 {
            unsafe { read_u32(address) as i32 }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let mut group = 0usize;
        while group < COUNTS.len() {
            let count = read_i32(this.wrapping_add(COUNTS[group]));
            let mut index = 0i32;
            while index < count {
                let flag = this
                    .wrapping_add(FLAGS[group])
                    .wrapping_add(index as u32);
                if read_u8(flag) != 0 {
                    let record = this
                        .wrapping_add(RECORD_BASES[group])
                        .wrapping_add((index as u32).wrapping_mul(STRIDES[group]));
                    let _ = lf_checker_rt::callee_thiscall!(
                        1,
                        u32,
                        record,
                        arg1,
                        arg2,
                        arg3
                    );
                }
                index = index.wrapping_add(1);
            }
            group += 1;
        }
        arg2
    }
});
