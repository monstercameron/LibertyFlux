// original: 0x009e85c0 ped_shared_sink_use
/// Uses the shared sink singleton for the item at `+0xD8`,
/// allocating (`0x20020`) and initialising it on first use, and
/// returns the sink call's answer. A failed allocation still calls the
/// sink with a null object. (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e85c0(this_ptr: u32) -> u32 {
    unsafe {
        const SINGLETON: u32 = 0x167E3B4;
        const ALLOC_TAG: u32 = 0x20020;
        const ITEM_OFF: u32 = 0xD8;
        let cached = lf_checker_rt::global::<u32>(SINGLETON).read_unaligned();
        let item = (this_ptr.wrapping_add(ITEM_OFF) as *const u32).read_unaligned();
        let live = if cached == 0 {
            let fresh: u32 = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_TAG);
            if fresh == 0 {
                lf_checker_rt::global::<u32>(SINGLETON).write_unaligned(0);
                0
            } else {
                let init: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh);
                lf_checker_rt::global::<u32>(SINGLETON).write_unaligned(init);
                init
            }
        } else {
            cached
        };
        lf_checker_rt::callee_thiscall!(3, u32, live, item)
    }
});
