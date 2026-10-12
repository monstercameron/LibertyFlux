// original: 0x0089A810 rage::audEnvelopeSound::commit_params
/// Commit the resolved envelope timing parameters into the sound node.
///
/// A thiscall node pointer and a 32-bit start-time value are supplied. The
/// method asks its parameter resolver to fill five stack words, then uses the
/// final word as a signed duration. A nonnegative duration sets the ready bit
/// and either records the elapsed and end times or marks the node as already
/// committed. A negative duration calls the node's error handler, sets its
/// error flag, and clears the pending bit. The method has no meaningful
/// return value. The resolver and error handler are intercepted by contract.
///
/// The resolver receives five pointers into this function's local frame. Their
/// addresses are skipped in the call comparison; the pointed-to words are
/// snapshotted. The patterned stack fill is retained.
lf_checker_rt::export!(thiscall, rw_0089a810(
    this: u32,
    start_time: u32,
) -> u32 {
    unsafe {
        const NODE_FLAGS: u32 = 0x38;
        const NODE_ERROR: u32 = 0x39;
        const START_MODE: u32 = 0x84;
        const ELAPSED_TIME: u32 = 0xcc;
        const END_TIME: u32 = 0xd0;
        const COMMIT_STATE: u32 = 0xf6;
        const PENDING_FLAGS: u32 = 0xf8;
        const COMMITTED_STATE: u8 = 4;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut output_words = core::mem::MaybeUninit::<[u32; 5]>::uninit();
        let output_ptr = output_words.as_mut_ptr().cast::<u32>();
        let _ = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            output_ptr.add(4) as u32,
            output_ptr.add(3) as u32,
            output_ptr.add(2) as u32,
            output_ptr.add(1) as u32,
            output_ptr as u32,
        );

        let mode = read_u32(this.wrapping_add(START_MODE)) as i32;
        let elapsed = if mode >= 0 {
            start_time.wrapping_sub(mode as u32)
        } else {
            0
        };
        let duration = output_ptr.read_unaligned();

        let ready_flags = read_u8(this.wrapping_add(NODE_FLAGS)) | 0x40;
        write_u8(this.wrapping_add(NODE_FLAGS), ready_flags);
        if (duration as i32) < 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
            let error_flags = read_u8(this.wrapping_add(NODE_ERROR)) | 8;
            write_u8(this.wrapping_add(NODE_ERROR), error_flags);
            let pending = read_u8(this.wrapping_add(PENDING_FLAGS)) & !2;
            write_u8(this.wrapping_add(PENDING_FLAGS), pending);
            return 0;
        }

        let pending = read_u8(this.wrapping_add(PENDING_FLAGS)) | 2;
        write_u8(this.wrapping_add(PENDING_FLAGS), pending);
        if read_u8(this.wrapping_add(COMMIT_STATE)) != COMMITTED_STATE {
            write_u32(this.wrapping_add(ELAPSED_TIME), elapsed);
            write_u8(this.wrapping_add(COMMIT_STATE), COMMITTED_STATE);
            write_u32(this.wrapping_add(END_TIME), elapsed.wrapping_add(duration));
        }
        0
    }
});
