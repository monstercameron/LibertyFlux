// original: 0x00DA6600 CTaskComplexShockingEventHurryAway::vf7

/// Spawner: when the task is live, allocate and construct the subtask for
/// the current target and return it, otherwise return null.
///
/// Liveness is the shared gate: a set flag with a set mode fires at once,
/// mixed flag/mode returns null, and a clear flag with a clear mode fires
/// only when the squared length of (`+0x30`, `+0x34`, `+0x38`) exceeds the
/// read-only threshold (float arithmetic in the original's order). Past
/// the gate a fresh object is allocated through the memory manager; a
/// null allocation returns null. With a set mode the entity form is built `(mode, 0, 1, +0x20)`; with a clear mode the vector form `(vector, 0, 1, +0x20)`.
/// The vector form passes the spilled position by address; the contract
/// skips that address and compares the three snapped words.
/// The caller's stack word is unused. Original: thiscall, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6600(this: u32, _unused: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR_A: u32 = 2;
        const CTOR_B: u32 = 3;
        const MEMMGR_SLOT: u32 = 0x0171FAF4;
        const THRESH_SLOT: u32 = 0x00FE876C;

        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let flag = ((this + 0x46) as *const u8).read();
        let mode = ((this + 0x48) as *const u32).read();
        let fire = if flag != 0 {
            mode != 0
        } else if mode != 0 {
            false
        } else {
            let x = ((this + 0x30) as *const f32).read();
            let y = ((this + 0x34) as *const f32).read();
            let z = ((this + 0x38) as *const f32).read();
            let sq = add(add(mul(x, x), mul(y, y)), mul(z, z));
            let thresh = (lf_checker_rt::relocated(THRESH_SLOT) as *const f32).read();
            sq > thresh
        };
        if !fire {
            return 0;
        }
        let manager = (lf_checker_rt::relocated(MEMMGR_SLOT) as *const u32).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if fresh == 0 {
            return 0;
        }
        let w20 = ((this + 0x20) as *const u32).read();
        if mode != 0 {
            lf_checker_rt::callee_thiscall!(CTOR_A, u32, fresh, mode, 0, 1, w20)
        } else {
            let spill = [((this + 0x30) as *const u32).read(),
                         ((this + 0x34) as *const u32).read(),
                         ((this + 0x38) as *const u32).read()];
            lf_checker_rt::callee_thiscall!(CTOR_B, u32, fresh, spill.as_ptr() as u32, 0, 1, w20)
        }
    }
});
