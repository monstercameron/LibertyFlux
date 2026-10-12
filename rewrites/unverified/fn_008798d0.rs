// original: 0x008798D0 rage::crmtComposer::vf5

/// Release the composer's active child components and clear their state.
///
/// Clear four counter words. If the byte flag at `+0x08` is set, release the
/// child at `+0x04` through vtable slot zero when it is non-null, then clear
/// the flag. Clear the first child pointer. Apply the same conditional release
/// to the child at `+0x0c` under the byte flag at `+0x10`, clear that flag, and
/// clear the second pointer. A release call uses the child in ECX and the
/// integer flag one as its stack argument.
///
/// Original calling convention: thiscall with no stack arguments; no semantic
/// return value.
lf_checker_rt::export!(thiscall, rw_008798d0(this: u32) -> () {
    const COUNTER_OFFSETS: [u32; 4] = [0x14, 0x18, 0x1c, 0x20];
    const CHILD_A: u32 = 0x04;
    const CHILD_A_FLAG: u32 = 0x08;
    const CHILD_B: u32 = 0x0c;
    const CHILD_B_FLAG: u32 = 0x10;
    unsafe {
        for offset in COUNTER_OFFSETS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        let child_a_flag = (this.wrapping_add(CHILD_A_FLAG) as *const u8).read();
        if child_a_flag != 0 {
            let child_a = (this.wrapping_add(CHILD_A) as *const u32).read_unaligned();
            if child_a != 0 {
                let vtable = (child_a as *const u32).read_unaligned();
                let target = (vtable as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _release_result = release(child_a, 1);
            }
            (this.wrapping_add(CHILD_A_FLAG) as *mut u8).write(0);
        }
        (this.wrapping_add(CHILD_A) as *mut u32).write_unaligned(0);

        let child_b_flag = (this.wrapping_add(CHILD_B_FLAG) as *const u8).read();
        if child_b_flag != 0 {
            let child_b = (this.wrapping_add(CHILD_B) as *const u32).read_unaligned();
            if child_b != 0 {
                let vtable = (child_b as *const u32).read_unaligned();
                let target = (vtable as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _release_result = release(child_b, 1);
            }
            (this.wrapping_add(CHILD_B_FLAG) as *mut u8).write(0);
        }
        (this.wrapping_add(CHILD_B) as *mut u32).write_unaligned(0);
    }
});
