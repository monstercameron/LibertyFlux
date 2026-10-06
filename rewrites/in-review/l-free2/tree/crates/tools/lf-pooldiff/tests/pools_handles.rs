//! Differential cases, part 7 (second lane): handle-indexed pages.
//!
//! Each case plants the handle table, its entries and their pages, runs
//! the rewrite and the lifted method on the same handle, and compares the
//! answers. Each method has a deliberately wrong lift that must be caught.
//! 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_pooldiff::rewrites::*;
    use lf_pooldiff::rt;
    use lf_world::pools::{HandlePool, Page};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{H_TAB_VA, Rng, addr, lock, put_u32};

    /// Plants a handle pool of `n` pages; returns the slot address, the
    /// lift pool and the page addresses. The slot carries the handle at
    /// +0x2E; every entry points at its page. Boxes are leaked so their
    /// addresses stay valid.
    fn plant_pool(n: usize, fill: &mut Rng) -> (u32, HandlePool, Vec<u32>) {
        let mut pages = Vec::with_capacity(n);
        let mut entry_addrs = Vec::with_capacity(n);
        let mut page_addrs = Vec::with_capacity(n);
        for _ in 0..n {
            let datum = fill.u32();
            let flags = fill.u32();
            pages.push(Page { datum, flags });
            let mut page = vec![0u8; 0x74];
            fill.bytes(&mut page);
            put_u32(&mut page, 8, datum);
            put_u32(&mut page, 0x6c, flags);
            let page_box = page.into_boxed_slice();
            page_addrs.push(addr(&page_box[0]));
            let mut entry = vec![0u8; 0x78];
            fill.bytes(&mut entry);
            put_u32(&mut entry, 0x70, page_addrs[page_addrs.len() - 1]);
            let entry_box = entry.into_boxed_slice();
            entry_addrs.push(addr(&entry_box[0]));
            std::mem::forget(page_box);
            std::mem::forget(entry_box);
        }
        let mut table = vec![0u8; n * 4];
        for (i, a) in entry_addrs.iter().enumerate() {
            put_u32(&mut table, i * 4, *a);
        }
        let table_box = table.into_boxed_slice();
        rt::set_relocated(H_TAB_VA, addr(&table_box[0]));
        std::mem::forget(table_box);
        let slot = Box::leak(Box::new([0u8; 0x40]));
        (addr(&slot[0]), HandlePool { pages }, page_addrs)
    }

    unsafe fn write_handle(slot: u32, handle: i16) {
        unsafe {
            ((slot + 0x2e) as *mut i16).write_unaligned(handle);
        }
    }

    unsafe fn read_page_word(page: u32, off: u32) -> u32 {
        unsafe { ((page + off) as *const u32).read_unaligned() }
    }

    #[test]
    fn datum_field_matches() {
        let _guard = lock();
        let mut rng = Rng(0x6A01);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..8 {
            let n = 1 + (rng.below(6) as usize);
            let (slot, pool, addrs) = plant_pool(n, &mut rng);
            for h in 0..n {
                let handle = h as i16;
                unsafe { write_handle(slot, handle) };
                let got = unsafe { fn_00A8EAD0::rw_00A8EAD0(slot) };
                assert_eq!(got, pool.datum_field(handle), "handle {handle} of {n}");
                // Wrong lift: the word 4 past the datum.
                let w = unsafe { read_page_word(addrs[h], 0x0c) };
                if w != got {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases > 8, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong datum never caught ({cases} cases)");
    }

    /// Runs one flag-bit instance over crafted pools. Returns (cases, caught).
    fn run_flag(bit: u32, seed: u32, fetch: extern "thiscall" fn(u32) -> u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..10 {
            let n = 1 + (rng.below(5) as usize);
            let (slot, mut pool, addrs) = plant_pool(n, &mut rng);
            // Craft every page so the tested bit and its neighbour
            // disagree (the neighbour bit decides the mutant).
            for (page, addr) in pool.pages.iter_mut().zip(addrs.iter()) {
                let rest = rng.u32() & !(1 << bit) & !(1 << (bit + 1));
                page.flags = if rng.u32() & 1 == 0 {
                    rest | (1 << bit)
                } else {
                    rest | (1 << (bit + 1))
                };
                unsafe {
                    ((*addr + 0x6c) as *mut u32).write_unaligned(page.flags);
                }
            }
            for h in 0..n {
                let handle = h as i16;
                unsafe { write_handle(slot, handle) };
                let got = unsafe { fetch(slot) };
                let want = pool.flag_bit(handle, bit);
                assert_eq!(got, u32::from(want), "handle {handle} bit {bit}");
                // Wrong lift: the neighbouring bit up.
                if pool.flag_bit(handle, bit + 1) != want {
                    caught += 1;
                }
                cases += 1;
            }
        }
        (cases, caught)
    }

    #[test]
    fn flag_bit15_matches() {
        let _guard = lock();
        let (cases, caught) = run_flag(15, 0x6A02, fn_00A8F2B0::rw_00A8F2B0);
        assert!(cases > 4, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong flag never caught ({cases} cases)");
    }

    #[test]
    fn flag_bit10_matches() {
        let _guard = lock();
        let (cases, caught) = run_flag(10, 0x6A03, fn_00A8F640::rw_00A8F640);
        assert!(cases > 4, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong flag never caught ({cases} cases)");
    }
}
