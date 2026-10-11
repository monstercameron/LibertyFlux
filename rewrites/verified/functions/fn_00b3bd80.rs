// original: 0x00B3BD80 copy_setting_vector_and_mark_ready
/// Copy four float component bit patterns from the caller's vector into the
/// corresponding global setting words, then set the readiness byte. This
/// routine performs no floating-point arithmetic, so every input bit pattern
/// is preserved, including signed zero and NaN payloads.
lf_checker_rt::export!(cdecl, rw_00b3bd80(source: u32) -> () {
    unsafe {
        const GLOBAL_COMPONENTS: [u32; 4] = [
            0x01662500, 0x01662504, 0x01662508, 0x0166250c,
        ];
        const GLOBAL_READY: u32 = 0x01662490;
        for (index, &global_address) in GLOBAL_COMPONENTS.iter().enumerate() {
            let bits = (source as *const u32).add(index).read_unaligned();
            lf_checker_rt::global::<u32>(global_address).write_unaligned(bits);
        }
        lf_checker_rt::global::<u8>(GLOBAL_READY).write(1);
    }
});
