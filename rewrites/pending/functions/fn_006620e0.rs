// original: 0x006620e0 rage::snHostSessionTask::setup_gamer_data
/// Host-session gamer-list setup: clamp ranges, copy list, optional trailer.
///
/// thiscall/9, returns 1 (low byte; the upper bytes carry the incoming value
/// or the trailer result). Stores the list parameters, clamps the two gamer
/// ranges so their signed total fits 32 entries, copies the 0x103-dword gamer
/// array from the caller and, when a trailer size is given, appends the
/// trailer record through callee 1.
export!(thiscall, rw_006620e0(
    this_ptr: u32,
    _w0: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    src: u32,
    a5: u32,
    trailer_arg: u32,
    a7: u32,
) -> u32 {
    unsafe {
        ((this_ptr + 0x9c) as *mut u32).write(a0);
        ((this_ptr + 0xa4) as *mut u32).write(a1);
        if a1 == 0 {
            ((this_ptr + 0xac) as *mut u32).write(0);
            ((this_ptr + 0xa8) as *mut u32).write(a2.wrapping_add(a3));
        } else {
            ((this_ptr + 0xa8) as *mut u32).write(a2);
            ((this_ptr + 0xac) as *mut u32).write(a3);
        }
        let mut first = ((this_ptr + 0xa8) as *const u32).read() as i32;
        let mut second = ((this_ptr + 0xac) as *const u32).read() as i32;
        if first.wrapping_add(second) > 0x20 {
            let over = 0x20i32.wrapping_sub(first).wrapping_sub(second);
            first = first.wrapping_sub(over);
            let clamped = if first > 0 { first } else { 0 };
            ((this_ptr + 0xa8) as *mut u32).write(clamped as u32);
            if clamped.wrapping_add(second) > 0x20 {
                second = second.wrapping_sub(over);
                let clamped2 = if second > 0 { second } else { 0 };
                ((this_ptr + 0xac) as *mut u32).write(clamped2 as u32);
            }
        }
        let empty =
            ((((this_ptr + 0xa8) as *const u32).read() as i32) <= 0) as u32;
        ((this_ptr + 0xa0) as *mut u32).write(empty);
        let dst = (this_ptr + 0xb0) as *mut u32;
        let from = src as *const u32;
        for i in 0..0x103usize {
            dst.add(i).write(from.add(i).read());
        }
        ((this_ptr + 0x4bc) as *mut u32).write(a5);
        ((this_ptr + 0x4c0) as *mut u32).write(a7);
        let tag = if a7 != 0 {
            callee_cdecl!(1, u32, this_ptr.wrapping_add(0x4c4), trailer_arg, a7)
        } else {
            a7
        };
        (tag & 0xffffff00) | 1
    }
});
