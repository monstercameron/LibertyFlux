// original: 0x0089A7E0 rage::audEnvelopeSound::~audEnvelopeSound__deleting
/// Run the base sound cleanup, optionally release the envelope-manager slot,
/// and return the node pointer.
///
/// The deleting destructor is a thiscall with one stack deletion flag. It
/// always forwards the node to the base sound cleanup. When bit zero of the
/// flag is set and the node pointer is non-null, it also forwards the node
/// and its channel byte to the envelope manager. Both callees are intercepted
/// by this function's contract; their return values are ignored.
lf_checker_rt::export!(thiscall, rw_0089a7e0(
    this: u32,
    delete_flag: u32,
) -> u32 {
    unsafe {
        const CHANNEL: u32 = 0x40;
        const ENVELOPE_MANAGER: u32 = 0x0115_d8a0;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if !false && (delete_flag & 1) != 0 && this != 0 {
            let channel = u32::from(read_u8(this.wrapping_add(CHANNEL)));
            let manager = lf_checker_rt::relocated(ENVELOPE_MANAGER);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, manager, this, channel);
        }
        this
    }
});
