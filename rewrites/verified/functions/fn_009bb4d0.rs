// original: 0x009bb4d0 build_input_hook_xor (proposed)

/// Build an input hook through two helpers, then deposit derived bits into it.
///
/// Calls the allocator helper (id 1, cdecl, `(0x60, 0)`); when it answers
/// null the function faults reading address 0 exactly like the original.
/// Otherwise initialises the object through helper id 2 (thiscall, the four
/// stack words forwarded in order) and reads its vtable slot at `+0x08`
/// twice (id 3, thiscall, no stack arguments).
///
/// Both vtable answers are SIGNED 32-bit values. The first is reduced
/// modulo 16 (signed remainder), subtracted from 0x10 and reduced modulo 16
/// again to give a shift count `s` in 0..15; the double reduction erases the
/// first remainder's signedness (shown by a passing unsigned mutant), so only
/// the residue class matters there. The second answer is added to `s`
/// (wrapping) and the sum divided by 16 with SIGNED truncation toward zero
/// (an unsigned division fails the proof) and shifted left 14. The word at
/// `hook+0x04` then keeps every bit outside 0x01FFC000 and takes the bits
/// inside it from the computed value; the deposited bits are also the
/// return value.
///
/// Edge cases: allocator-null faults before any other call; the vtable
/// answers exercise negative, edge (0x80000000, 0xFFFFFFFF, 0x7FFFFFFF)
/// and bound-adjacent (15, 16, 17 and negatives) values across trials.
///
/// Original: cdecl, four stack words, three callees (two direct, one vtable).
lf_checker_rt::export!(cdecl, rw_009bb4d0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const INIT: u32 = 2;
        const VTABLE_SLOT: u32 = 0x08;
        const CELL_OFF: u32 = 0x04;
        const DEPOSIT_MASK: u32 = 0x01ffc000;
        let obj: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, 0x60, 0);
        let hook: u32 = if obj == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(INIT, u32, obj, a0, a1, a2, a3)
        };
        let vtable = (hook as *const u32).read_unaligned();
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vtable + VTABLE_SLOT) as *const u32).read_unaligned() as usize);
        let first = slot(hook);
        let m1 = (first as i32) % 16;
        let shift = (0x10 - m1) % 16;
        let second = slot(hook);
        let combined = (second as i32).wrapping_add(shift);
        let placed = ((combined / 16) << 14) as u32;
        let cell = (hook + CELL_OFF) as *mut u32;
        let old = cell.read_unaligned();
        cell.write_unaligned((old & !DEPOSIT_MASK) | (placed & DEPOSIT_MASK));
        (old ^ placed) & DEPOSIT_MASK
    }
});
