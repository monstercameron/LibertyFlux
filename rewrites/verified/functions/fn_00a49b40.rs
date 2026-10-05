// original: 0x00a49b40 NativeImpl_IS_VEH_WINDOW_INTACT
/// True when the resolved part's probe vector has positive squared length.
///
/// Returns 0 when the handle at `[this+0xDC4]` is null or the part id at
/// `tab2[index]` is -1. Otherwise the resolver (stdcall/1) maps the id, the
/// filler (thiscall/1) writes three floats 0x30 bytes past its scratch
/// argument, and 1 returns when `x*x + y*y + z*z` exceeds the shared
/// constant (thiscall, one stack argument). Only AL is compared.
export!(thiscall, rw_00a49b40(this: u32, index: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        const LIMIT_ADDR: u32 = 0x00fe8628;
        let field = (this.wrapping_add(0xdc4) as *const u32).read_unaligned();
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let tab2 = (row.wrapping_add(0xcc) as *const u32).read_unaligned();
        let part = (tab2.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if field == 0 {
            return 0;
        }
        if part == 0xffffffff {
            return 0;
        }
        let a: u32 = callee_stdcall!(1, u32, part);
        let mut buf = [0u32; 15];
        let _: u32 = callee_thiscall!(2, u32, (&mut buf as *mut u32) as u32, a);
        let x = f32::from_bits(buf[12]);
        let y = f32::from_bits(buf[13]);
        let z = f32::from_bits(buf[14]);
        let xx = core::hint::black_box(x) * core::hint::black_box(x);
        let yy = core::hint::black_box(y) * core::hint::black_box(y);
        let s = core::hint::black_box(yy) + core::hint::black_box(xx);
        let zz = core::hint::black_box(z) * core::hint::black_box(z);
        let s = core::hint::black_box(s) + core::hint::black_box(zz);
        let c = f32::from_bits((relocated(LIMIT_ADDR) as *const u32).read_unaligned());
        if core::hint::black_box(s) > core::hint::black_box(c) {
            1
        } else {
            0
        }
    }
});
