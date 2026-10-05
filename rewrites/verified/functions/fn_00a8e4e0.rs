// original: 0x00a8e4e0 CInteriorInst::vf23

/// Copy the four words at +0x90..+0x9C out, returning the kind's float.
///
/// `this` points to the interior instance, `dst` receives four words. The
/// signed 16-bit kind at +0x2E selects the handler-table entry whose float
/// at +0x1C is returned on the floating-point channel.
///
/// Original: 0x00A8E4E0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8e4e0(this: u32, dst: u32) -> f32 {
    unsafe {
        const FIELD0: u32 = 0x90;
        const FIELD1: u32 = 0x94;
        const FIELD2: u32 = 0x98;
        const FIELD3: u32 = 0x9c;
        const KIND: u32 = 0x2e;
        const HANDLER_TABLE: u32 = 0x1295cd8;
        const KIND_FLOAT: u32 = 0x1c;
        let w0 = ((this + FIELD0) as *const u32).read_unaligned();
        let w1 = ((this + FIELD1) as *const u32).read_unaligned();
        let w2 = ((this + FIELD2) as *const u32).read_unaligned();
        let w3 = ((this + FIELD3) as *const u32).read_unaligned();
        (dst as *mut u32).write_unaligned(w0);
        ((dst + 4) as *mut u32).write_unaligned(w1);
        ((dst + 8) as *mut u32).write_unaligned(w2);
        ((dst + 12) as *mut u32).write_unaligned(w3);
        let kind =
            ((this + KIND) as *const i16).read_unaligned() as i32 as usize;
        let entry =
            lf_checker_rt::global::<u32>(HANDLER_TABLE).add(kind).read_unaligned();
        ((entry + KIND_FLOAT) as *const f32).read_unaligned()
    }
});
