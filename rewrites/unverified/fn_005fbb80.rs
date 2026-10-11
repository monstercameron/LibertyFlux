// original: 0x005FBB80 fragment_tls_counter_update

/// Set the object's first word to its class marker, then consult the fabricated
/// thread-local record at slot 0. Three lookup helpers write through pointers to
/// the object's words at `+4`, `+8` and `+0xc`; on a result other than `0xffffffff`,
/// a nonzero written value is passed to the corresponding update helper and its
/// return is added with 32-bit wrapping. All three words are cleared before return,
/// which is the original `this` pointer. The contract disables stack comparison
/// for the original's temporary stack scratch; its helper context is compared in
/// the outgoing call log. This is a thiscall method.

lf_checker_rt::export!(thiscall, rw_005fbb80(this: u32) -> u32 {
    unsafe { run_005fbb80(this, 0x00fe_17d4) }
}
