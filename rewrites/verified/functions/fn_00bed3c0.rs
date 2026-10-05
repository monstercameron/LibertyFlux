// original: 0x00bed3c0 scale_i8x4_960
/// Convert 4 signed bytes at `[this+...]` to scaled floats in an output buffer.
///
/// Each lane sign-extends its byte, converts it exactly to f32 and multiplies
/// by the shared constant at file VA 0x00fe8700 (read from the relocated image,
/// in the original's operand order with the order pinned against commuting).
/// Lane offsets: 0x0c, 0x0d, 0x0e, 0x0f. Returns the last sign-extended lane (the original's
/// EAX leftover). Thiscall, one stack argument (the destination).
export!(thiscall, rw_00bed3c0(this: u32, dst: u32) -> u32 {
    unsafe {
        const K_ADDR: u32 = 0x00fe8700;
        let k = f32::from_bits((relocated(K_ADDR) as *const u32).read_unaligned());
            let w0 = ((this + 0x0c) as *const i8).read_unaligned();
            let f0 = w0 as f32;
            let r0 = core::hint::black_box(f0) * core::hint::black_box(k);
            ((dst + 0) as *mut u32).write_unaligned(r0.to_bits());
            let w1 = ((this + 0x0d) as *const i8).read_unaligned();
            let f1 = w1 as f32;
            let r1 = core::hint::black_box(f1) * core::hint::black_box(k);
            ((dst + 4) as *mut u32).write_unaligned(r1.to_bits());
            let w2 = ((this + 0x0e) as *const i8).read_unaligned();
            let f2 = w2 as f32;
            let r2 = core::hint::black_box(f2) * core::hint::black_box(k);
            ((dst + 8) as *mut u32).write_unaligned(r2.to_bits());
            let w3 = ((this + 0x0f) as *const i8).read_unaligned();
            let f3 = w3 as f32;
            let r3 = core::hint::black_box(f3) * core::hint::black_box(k);
            ((dst + 12) as *mut u32).write_unaligned(r3.to_bits());
        w3 as i32 as u32
    }
});
