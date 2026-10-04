// original: 0x009e3b90 ped_iter_apply_float (proposed)

/// Apply a float to every iterator item whose count is positive.
///
/// Resolves the head (`thiscall` on `[this + 0x78]`, no stack words); a
/// null head returns 0. Otherwise each item with a signed count at +8
/// above zero gets bit 0x4000 ORed into its flags at +4 and one apply
/// call (`thiscall` on the item, the float word), then iteration
/// continues through the next callee until it returns null. Returns 0.
/// `thiscall`, one stack word (float bits).
lf_checker_rt::export!(thiscall, rw_009e3b90(this: u32, f: u32) -> u32 {
    unsafe {
        const ITER: u32 = 0x78;
        const COUNT: u32 = 8;
        const FLAGS: u32 = 4;
        const TOUCHED: u32 = 0x4000;
        const FIRST: u32 = 1;
        const APPLY: u32 = 2;
        const NEXT: u32 = 3;
        let head = ((this + ITER) as *const u32).read_unaligned();
        let mut it = lf_checker_rt::callee_thiscall!(FIRST, u32, head);
        if it == 0 {
            return 0;
        }
        loop {
            if (((it + COUNT) as *const u32).read_unaligned() as i32) > 0 {
                let fl = ((it + FLAGS) as *const u32).read_unaligned();
                ((it + FLAGS) as *mut u32).write_unaligned(fl | TOUCHED);
                lf_checker_rt::callee_thiscall!(APPLY, u32, it, f);
            }
            let head = ((this + ITER) as *const u32).read_unaligned();
            it = lf_checker_rt::callee_thiscall!(NEXT, u32, head);
            if it == 0 {
                return 0;
            }
        }
    }
});
