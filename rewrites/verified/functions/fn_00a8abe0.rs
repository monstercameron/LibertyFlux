// original: 0x00A8ABE0 pool_struct_kernel_dispatch

/// Initialise the pool's capacity and connect its two four-word descriptor
/// halves. The descriptor helper fills four words through each stack pointer;
/// the config helper receives the halves in reversed order with their final
/// words replaced by the relocated join thunk. Its return value is forwarded.
///
/// `this` is the pool object. The state word at byte offset `0x48` is cleared
/// before any helper runs. The function uses thiscall and takes no stack args.
lf_checker_rt::export!(thiscall, rw_00a8abe0(this: u32) -> u32 {
    unsafe {
        const STATE_WORD: u32 = 0x48;
        const CALLEE_CAPACITY: u32 = 1;
        const CALLEE_WIRE: u32 = 2;
        const CALLEE_CONFIG: u32 = 3;
        const HANDLER_A: u32 = 0x00a8abc0;
        const HANDLER_B: u32 = 0x004016a0;
        const JOIN_THUNK: u32 = 0x00430260;

        ((this + STATE_WORD) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(CALLEE_CAPACITY, u32, this);

        let mut descriptor_words = [0u32; 8];
        let first_half = core::ptr::addr_of_mut!(descriptor_words[4]) as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_WIRE,
            u32,
            first_half,
            0,
            lf_checker_rt::relocated(HANDLER_A),
            0,
            0
        );

        let second_half = core::ptr::addr_of_mut!(descriptor_words[0]) as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_WIRE,
            u32,
            second_half,
            0,
            lf_checker_rt::relocated(HANDLER_B),
            0,
            0
        );

        let join_thunk = lf_checker_rt::relocated(JOIN_THUNK);
        descriptor_words[3] = join_thunk;
        descriptor_words[7] = join_thunk;

        lf_checker_rt::callee_cdecl!(
            CALLEE_CONFIG,
            u32,
            descriptor_words[4],
            descriptor_words[5],
            descriptor_words[6],
            descriptor_words[7],
            descriptor_words[0],
            descriptor_words[1],
            descriptor_words[2],
            descriptor_words[3]
        )
    }
});
