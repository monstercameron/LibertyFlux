// original: 0x00da46a0 either_list_contains
// s10f17: whether either of two dword lists contains the value (thiscall/1).
export!(thiscall, rw_s10f17(this: *const u8, val: u32) -> u8 {
    unsafe {
        let a0 = *(this.add(0x18) as *const u32);
        let a1 = *(this.add(0x1c) as *const u32);
        let na = (a1.wrapping_sub(a0) as i32) >> 2;
        let mut i = 0i32;
        while i < na {
            if *((a0 as *const u32).offset(i as isize)) == val {
                return 1;
            }
            i += 1;
        }
        let b0 = *(this.add(0x24) as *const u32);
        let b1 = *(this.add(0x28) as *const u32);
        let nb = (b1.wrapping_sub(b0) as i32) >> 2;
        let mut j = 0i32;
        while j < nb {
            if *((b0 as *const u32).offset(j as isize)) == val {
                return 1;
            }
            j += 1;
        }
        0
    }
});
