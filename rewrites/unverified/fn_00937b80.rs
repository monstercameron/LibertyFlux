// original: 0x00937B80 stream_obj_chain_init (proposed)

/// Run the four initialisers of a streaming object in order.
///
/// `this` points to the object. Calls the first two initialisers on `this`,
/// the third on `this+8`, then tail-calls the third again on `this+4`,
/// returning its answer (thiscall).
lf_checker_rt::export!(thiscall, rw_00937b80(this: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const SECOND: u32 = 2;
        const THIRD: u32 = 3;
        const UPPER_OFF: u32 = 8;
        const LOWER_OFF: u32 = 4;
        let _: u32 = lf_checker_rt::callee_thiscall!(FIRST, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(SECOND, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(THIRD, u32, this + UPPER_OFF);
        lf_checker_rt::callee_thiscall!(THIRD, u32, this + LOWER_OFF)
    }
});
