// original: 0x00985eb0 audRadioEmitter::vf12
/// Mark two radio-emitter slots invalid.
///
/// Stores 0xFFFFFFFF through both out-pointers and returns the second one.
export!(stdcall, rw_00985eb0(o1: u32, o2: u32) -> u32 {
    unsafe {
        *(o1 as *mut u32) = 0xFFFFFFFF;
        *(o2 as *mut u32) = 0xFFFFFFFF;
        o2
    }
});
