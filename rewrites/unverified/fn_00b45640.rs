// original: 0x00b45640 CEventPedToChase::CEventPedToChase
/// Rebuilds this event object in place with the original MSVC thiscall layout.
/// ECX is the receiver; each named parameter is one incoming 32-bit stack word.
/// calls the event base constructor, then installs its own vtable; ped_reference is stored at +0x0C (4 byte(s); reference-link calls use the object field address as their stack argument and the stored reference as ECX.
/// The returned EAX value is the receiver.
lf_checker_rt::export!(thiscall, rw_00b45640(this: u32, ped_reference: u32) -> u32 {
        const VTABLE_FIELD: u32 = 0x00000000;
        const VTABLE_FILE_VA: u32 = 0x00eaddd4;
        const FIELD_0C: u32 = 0x0000000c;
        const GLOBAL_EVENT_TOKEN: u32 = 0x011735b4;
        const INVALID_SENTINEL: u32 = 0xffffffff;
        let _ = GLOBAL_EVENT_TOKEN;
        #[inline(always)] unsafe fn read_u32(address: u32) -> u32 { core::ptr::read_unaligned(address as *const u32) }
        #[inline(always)] unsafe fn write_u32(address: u32, value: u32) { core::ptr::write_unaligned(address as *mut u32, value); }
        #[inline(always)] unsafe fn write_u16(address: u32, value: u16) { core::ptr::write_unaligned(address as *mut u16, value); }
        #[inline(always)] unsafe fn write_u8(address: u32, value: u8) { core::ptr::write(address as *mut u8, value); }
        #[inline(always)] unsafe fn copy_words(destination: u32, source: u32, count: usize) {
            for index in 0..count {
                let value = read_u32(source.wrapping_add((index as u32).wrapping_mul(4)));
                write_u32(destination.wrapping_add((index as u32).wrapping_mul(4)), value);
            }
        }
    unsafe {
        let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        write_u32(this.wrapping_add(VTABLE_FIELD), lf_checker_rt::relocated(VTABLE_FILE_VA));
        write_u32(this.wrapping_add(FIELD_0C), ped_reference);
        if read_u32(this.wrapping_add(FIELD_0C)) != 0 {
            let _link_result = lf_checker_rt::callee_thiscall!(2, u32, read_u32(this.wrapping_add(FIELD_0C)), this.wrapping_add(FIELD_0C));
        }
        this
    }
});
