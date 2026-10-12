// original: 0x00AB9280 variation_record_init_new

/// Run the record's initializer and return the original receiver.
///
/// The receiver arrives in ECX and is forwarded as the only argument to a
/// thiscall initializer. Its return value is ignored; this method returns the
/// receiver unchanged after the initializer completes.
lf_checker_rt::export!(thiscall, rw_00ab9280(this: u32) -> u32 {
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
    this
});
