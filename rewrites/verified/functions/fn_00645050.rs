// original: 0x00645050 rage::ptxDomainSphere::ctor (proposed)

/// Initialise a sphere particle domain object.
///
/// Runs the base initialiser (callee C, id 1) with the first argument
/// and mode 2, plants the domain vtable, zeroes the output triples at
/// `+0xf0`, `+0x130`, `+0x140` and `+0x150`, writes identity rows into
/// the basis at `+0xb0`/`+0xc0`/`+0xd0` and zeroes the centre at `+0xe0`.
/// It then resolves a runtime hook through thread-local slot 0 (the
/// word at `+8`, one more hop, then a slot at `+0x3c`, all fabricated by
/// the contract) and calls it with the resolved context; when the hook
/// answers zero, the domain setup routine (callee D, id 3) runs over the
/// object.
/// Returns the object.
///
/// Original: 0x00645050 (thiscall, two stack words of which only the
/// first is read). The hook's low result byte alone decides the branch;
/// no signedness question arises. The class attribution is proposed:
/// the routine sits between the sphere domain's sampled-data methods,
/// writes the vtable its siblings read through, and initialises exactly
/// the fields they use.
lf_checker_rt::export!(thiscall, rw_00645050(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        const MODE: u32 = 2;
        const VTABLE: u32 = 0x00FE291C;
        const TLS_SLOT: usize = 0;
        const HOOK_SLOT: u32 = 0x3c;
        const ONE_BITS: u32 = 0x3F800000;
        const CALLEE_INIT: u32 = 1;
        const CALLEE_SETUP: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, this, a0, MODE);
        wr32(this, lf_checker_rt::relocated(VTABLE));

        wr32(this.wrapping_add(0xb0), ONE_BITS);
        wr32(this.wrapping_add(0xb4), 0);
        wr32(this.wrapping_add(0xb8), 0);
        wr32(this.wrapping_add(0xc0), 0);
        wr32(this.wrapping_add(0xc4), ONE_BITS);
        wr32(this.wrapping_add(0xc8), 0);
        wr32(this.wrapping_add(0xd0), 0);
        wr32(this.wrapping_add(0xd4), 0);
        wr32(this.wrapping_add(0xd8), ONE_BITS);
        wr32(this.wrapping_add(0xe8), 0);
        wr32(this.wrapping_add(0xe4), 0);
        wr32(this.wrapping_add(0xe0), 0);
        wr32(this.wrapping_add(0xf8), 0);
        wr32(this.wrapping_add(0xf4), 0);
        wr32(this.wrapping_add(0xf0), 0);
        wr32(this.wrapping_add(0x148), 0);
        wr32(this.wrapping_add(0x144), 0);
        wr32(this.wrapping_add(0x140), 0);
        wr32(this.wrapping_add(0x158), 0);
        wr32(this.wrapping_add(0x154), 0);
        wr32(this.wrapping_add(0x150), 0);
        wr32(this.wrapping_add(0x138), 0);
        wr32(this.wrapping_add(0x134), 0);
        wr32(this.wrapping_add(0x130), 0);

        let s0 = lf_checker_rt::tls_slot(TLS_SLOT);
        let e = rd32(s0.wrapping_add(8));
        let f = rd32(e);
        let target = rd32(f.wrapping_add(HOOK_SLOT));
        let hook: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let ans = hook(e);
        if ans as u8 == 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_SETUP, u32, this);
        }
        this
    }
});
