// original: 0x00D24AF0 CCustomShaderEffectVehicleFX::vf7
/// Allocates and initializes a draw-command object, then queries its virtual method twice to
/// update the command's packed page bits. The two scripted answers advance per call within each
/// trial, and the virtual table is supplied as a fabricated heap object containing the recorder
/// stub.
lf_checker_rt::export!(thiscall, rw_00d24af0(this: u32) -> () {
    const ALLOC_SIZE: u32 = 0x14;
    const TEXTURE_PAGE_MASK: u32 = 0x01ffc000;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    fn normalized_low_nibble(value: u32) -> i32 {
        let low = (value & 0xf) as i32;
        if value & 0x8000_0000 == 0 {
            low
        } else if low == 0 {
            0
        } else {
            low - 16
        }
    }
    unsafe {
        let allocation = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE, 0);
        let command = lf_checker_rt::callee_thiscall!(2, u32, allocation, this);
        let vtable = read_u32(command, 0);
        let target = read_u32(vtable, 8);
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let first_raw = query(command);
        let first = normalized_low_nibble(first_raw);
        let start_offset = (16i32.wrapping_sub(first) as u32) & 0xf;
        let second_raw = query(command);
        let combined = second_raw.wrapping_add(start_offset);
        let sign_adjust = if combined & 0x8000_0000 != 0 { 0xf } else { 0 };
        let rounded = combined.wrapping_add(sign_adjust) as i32;
        let page_bits = ((rounded >> 4) as u32).wrapping_shl(14);
        let old_flags = read_u32(command, 4);
        let changed = old_flags ^ ((page_bits ^ old_flags) & TEXTURE_PAGE_MASK);
        write_u32(command, 4, changed);
    }
});
