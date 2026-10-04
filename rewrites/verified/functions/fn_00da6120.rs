// original: 0x00DA6120 CTaskComplexShockingEventHurryAway::vf18

/// Reaction update: virtual slot 0x0c of the subtask at `+0x08` must
/// answer 0x141, else report 0. On pass, fetch a four-word vector through
/// the out-param callee, subtract its first three words from the target
/// triple (found through the caller argument's `+0x20` link), keep the
/// differences at `+0x70..+0x78` with the fourth word at `+0x7c`, clear
/// `+0x78`, notify through the notify callee, then build through the
/// two-argument callee with (0x3ae, caller argument) and return its
/// answer. Original: thiscall, one stack word, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6120(this: u32, arg: u32) -> u32 {
    unsafe {
        const TYPE_CHECK: u32 = 1;
        const FETCH: u32 = 2;
        const NOTIFY: u32 = 3;
        const BUILD: u32 = 4;
        const WANT_TYPE: u32 = 0x141;
        const BUILD_MODE: u32 = 0x3AE;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let sub_obj = ((this + 0x08) as *const u32).read();
        let table = (sub_obj as *const u32).read();
        let slot = ((table + 0x0C) as *const u32).read();
        let type_check: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if type_check(sub_obj) != WANT_TYPE {
            return 0;
        }
        let mut spill = [0u32; 4];
        lf_checker_rt::callee_thiscall!(FETCH, u32, this + 0x20, spill.as_mut_ptr() as u32);
        let link = ((arg + 0x20) as *const u32).read();
        let tx = ((link + 0x30) as *const f32).read();
        let ty = ((link + 0x34) as *const f32).read();
        let tz = ((link + 0x38) as *const f32).read();
        let dx = sub(tx, f32::from_bits(spill[0]));
        let dy = sub(ty, f32::from_bits(spill[1]));
        let dz = sub(tz, f32::from_bits(spill[2]));
        ((this + 0x70) as *mut f32).write(dx);
        ((this + 0x74) as *mut f32).write(dy);
        ((this + 0x78) as *mut f32).write(dz);
        ((this + 0x7C) as *mut u32).write(spill[3]);
        ((this + 0x78) as *mut u32).write(0);
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, this + 0x70);
        lf_checker_rt::callee_thiscall!(BUILD, u32, this, BUILD_MODE, arg)
    }
});
