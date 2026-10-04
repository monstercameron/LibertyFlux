// original: 0x00b25a30 touching_list_remove
// s08_b25a30: remove an entity from the touching list. thiscall/1 (other):
// scans the list at +0x154 (count byte at +0x150) from the end, shifting
// down and recounting on every match. Returns nothing.
export!(thiscall, rw_b25a30(this: *mut u8, other: u32) -> () {
    unsafe {
        let count = *this.add(0x150) as i32;
        let mut i = count - 1;
        while i >= 0 {
            let list = this.add(0x154) as *mut u32;
            if *list.add(i as usize) == other {
                let n = *this.add(0x150) as i32;
                let mut j = i;
                while j + 1 < n {
                    let v = *list.add((j + 1) as usize);
                    *list.add(j as usize) = v;
                    j += 1;
                }
                let c = *this.add(0x150);
                if c != 0 {
                    *this.add(0x150) = c - 1;
                }
            }
            i -= 1;
        }
    }
});
