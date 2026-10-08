// original: 0x00C03810 stream_emit_1

/// Calls the output helper into a four-word local buffer and forwards that
/// same buffer to the next helper with two scaled owner bytes. The first
/// helper writes three words; the fourth word is outside this proof because
/// the original leaves it uninitialized, though the next helper may access
/// it. The helper's
/// scripted outputs vary independently of the owner words. The first call's
/// pointer and the second call's pointer value are skipped; the second call
/// snapshots all three initialized output words. Both helpers are stubbed,
/// and the wrapper's void return is not observed.
lf_checker_rt::export!(thiscall, rw_00c03810(this: u32) -> () {
    unsafe {
        let mut output = [0u32; 4];
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, output.as_mut_ptr() as u32);

        let owner_bytes = this as *const u8;
        let byte_15 = owner_bytes.add(0x15).read();
        let first_scaled = scaled_byte(byte_15, 0x00ebd530, 0x00fe8b20).to_bits();
        let byte_14 = owner_bytes.add(0x14).read();
        let second_scaled = scaled_byte(byte_14, 0x00ea4d28, 0x00fe88e8).to_bits();

        let _ = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            lf_checker_rt::relocated(0x013baba0),
            output.as_ptr() as u32,
            second_scaled,
            first_scaled,
            1
        );
    }
});

#[inline(always)]
fn scaled_byte(byte: u8, scale_va: u32, add_va: u32) -> f32 {
    unsafe {
        let scale = lf_checker_rt::global::<f32>(scale_va).read_unaligned();
        let add = lf_checker_rt::global::<f32>(add_va).read_unaligned();
        let product = core::hint::black_box(f32::from(byte)) * core::hint::black_box(scale);
        core::hint::black_box(product) + core::hint::black_box(add)
    }
}
