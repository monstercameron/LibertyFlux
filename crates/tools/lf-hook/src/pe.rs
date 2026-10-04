//! Minimal PE reader: import slots and section info from a loaded
//! module, export tables from a file. Dependency-free so the proxy's build
//! script can include this file directly (`#[path]`).
//!
//! Reads are bounds-checked for file input; for live module input the
//! headers of a loaded image are trusted but every dereference goes
//! through `read_*` helpers that the caller guards with `is_readable`.

// Live-module reads dereference image memory; callers must hold the
// module and have checked readability first (see the `live` docs).
#![allow(unsafe_code)]
// Address casts are inherent to header parsing; pedantic style lints stay
// off here (in the library and in the proxy build script, which includes
// this file) while correctness lints (clippy::all) apply.
#![allow(clippy::pedantic)]

/// One export-table entry of a DLL.
#[derive(Clone, Debug)]
pub struct Export {
    /// Export name (`None` for ordinal-only exports).
    pub name: Option<String>,
    /// Export ordinal.
    pub ordinal: u32,
    /// RVA of the export (or of the forwarder string).
    pub rva: u32,
    /// True when `rva` points at a forwarder string, not code.
    pub is_forwarder: bool,
}

fn u16_at(data: &[u8], off: usize) -> Option<u16> {
    let b = data.get(off..off + 2)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

fn u32_at(data: &[u8], off: usize) -> Option<u32> {
    let b = data.get(off..off + 4)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn cstr_at(data: &[u8], mut off: usize) -> Option<String> {
    let mut out = Vec::new();
    for _ in 0..512 {
        let b = *data.get(off)?;
        if b == 0 {
            break;
        }
        out.push(b);
        off += 1;
    }
    String::from_utf8(out).ok()
}

struct FileSections {
    sections: Vec<(u32, u32, u32)>, // (vaddr, vsize, rawptr)
}

impl FileSections {
    fn rva_to_off(&self, rva: u32) -> Option<usize> {
        if rva == 0 {
            return None;
        }
        for (vaddr, vsize, raw) in &self.sections {
            let size = (*vsize).max(1);
            if rva >= *vaddr && rva < vaddr.wrapping_add(size) {
                return Some(raw.wrapping_add(rva - vaddr) as usize);
            }
        }
        // Header data (before the first section) maps 1:1.
        if let Some(first) = self.sections.first()
            && rva < first.0
        {
            return Some(rva as usize);
        }
        None
    }
}

fn file_section_table(data: &[u8]) -> Option<(FileSections, usize)> {
    if data.get(0..2) != Some(b"MZ") {
        return None;
    }
    let pe = u32_at(data, 0x3C)? as usize;
    if data.get(pe..pe + 4) != Some(b"PE\0\0") {
        return None;
    }
    let num = u16_at(data, pe + 6)? as usize;
    let opt_size = u16_at(data, pe + 20)? as usize;
    let opt = pe + 24;
    let table = opt + opt_size;
    let mut sections = Vec::new();
    for i in 0..num {
        let e = table + i * 40;
        let vsize = u32_at(data, e + 8)?;
        let vaddr = u32_at(data, e + 12)?;
        let rawsize = u32_at(data, e + 16)?;
        let rawptr = u32_at(data, e + 20)?;
        if rawsize > 0 {
            sections.push((vaddr, vsize, rawptr));
        }
    }
    Some((FileSections { sections }, opt))
}

/// Export table of a PE file's bytes. Used by the proxy build script.
#[must_use]
pub fn file_exports(data: &[u8]) -> Option<Vec<Export>> {
    let (secs, opt) = file_section_table(data)?;
    let magic = u16_at(data, opt)?;
    // Export directory entry: offset 96 in PE32 optional header, 112 in PE32+.
    let entry = opt + if magic == 0x10B { 96 } else { 112 };
    let exp_rva = u32_at(data, entry)?;
    let exp_size = u32_at(data, entry + 4)?;
    let exp_off = secs.rva_to_off(exp_rva)?;
    let num_names = u32_at(data, exp_off + 24)? as usize;
    let addr_funcs = secs.rva_to_off(u32_at(data, exp_off + 28)?)?;
    let addr_names = secs.rva_to_off(u32_at(data, exp_off + 32)?)?;
    let addr_ord = secs.rva_to_off(u32_at(data, exp_off + 36)?)?;
    let base = u32_at(data, exp_off + 16)?;
    let mut out = Vec::new();
    // Named exports.
    for i in 0..num_names {
        let name_rva = u32_at(data, addr_names + i * 4)?;
        let name_off = secs.rva_to_off(name_rva)?;
        let name = cstr_at(data, name_off)?;
        let ord_idx = u16_at(data, addr_ord + i * 2)? as usize;
        let func_rva = u32_at(data, addr_funcs + ord_idx * 4)?;
        let is_forwarder = func_rva >= exp_rva && func_rva < exp_rva + exp_size;
        out.push(Export {
            name: Some(name),
            ordinal: base + ord_idx as u32,
            rva: func_rva,
            is_forwarder,
        });
    }
    // Ordinal-only exports: function entries no name points at.
    let num_funcs = u32_at(data, exp_off + 20)? as usize;
    let mut named_idx = vec![false; num_funcs];
    for i in 0..num_names {
        let ord_idx = u16_at(data, addr_ord + i * 2)? as usize;
        if ord_idx < num_funcs {
            named_idx[ord_idx] = true;
        }
    }
    for (idx, seen) in named_idx.iter().enumerate() {
        if !seen {
            let func_rva = u32_at(data, addr_funcs + idx * 4)?;
            if func_rva != 0 {
                let is_forwarder = func_rva >= exp_rva && func_rva < exp_rva + exp_size;
                out.push(Export {
                    name: None,
                    ordinal: base + idx as u32,
                    rva: func_rva,
                    is_forwarder,
                });
            }
        }
    }
    Some(out)
}

// ---- Live module views (memory-mapped headers) ----

pub mod live {
    //! Unsafe views over a loaded module's headers. The caller must pass the
    //! real base of a mapped image; reads use unaligned loads.

    use std::ptr::read_unaligned;

    unsafe fn ru16(base: usize, off: usize) -> u16 {
        // SAFETY: upheld by the caller (mapped image + readability check).
        unsafe { read_unaligned((base + off) as *const u16) }
    }

    unsafe fn ru32(base: usize, off: usize) -> u32 {
        // SAFETY: upheld by the caller (mapped image + readability check).
        unsafe { read_unaligned((base + off) as *const u32) }
    }

    unsafe fn rcstr(base: usize, rva: u32) -> Option<String> {
        // SAFETY: upheld by the caller (mapped image + readability check).
        unsafe {
            let mut out = Vec::new();
            for i in 0..512usize {
                let b = read_unaligned((base + rva as usize + i) as *const u8);
                if b == 0 {
                    break;
                }
                out.push(b);
            }
            String::from_utf8(out).ok()
        }
    }

    fn headers(base: usize) -> Option<(usize, usize, usize)> {
        if base == 0 {
            return None;
        }
        unsafe {
            if ru16(base, 0) != 0x5A4D {
                return None;
            }
            let pe = ru32(base, 0x3C) as usize;
            if pe > 1024 * 1024 {
                return None;
            }
            if ru32(base, pe) != 0x4550 {
                return None;
            }
            let num = ru16(base, pe + 6) as usize;
            let opt_size = ru16(base, pe + 20) as usize;
            if num > 96 || opt_size > 512 {
                return None;
            }
            let opt = pe + 24;
            Some((opt, opt_size, num))
        }
    }

    /// Sections of a loaded module: (name, vaddr, vsize).
    #[must_use]
    pub fn sections(base: usize) -> Option<Vec<(String, u32, u32)>> {
        let (opt, opt_size, num) = headers(base)?;
        let table = opt + opt_size;
        let mut out = Vec::new();
        for i in 0..num {
            let e = table + i * 40;
            unsafe {
                let mut name = [0u8; 8];
                for (k, slot) in name.iter_mut().enumerate() {
                    *slot = read_unaligned((base + e + k) as *const u8);
                }
                let end = name.iter().position(|&b| b == 0).unwrap_or(8);
                let vsize = ru32(base, e + 8);
                let vaddr = ru32(base, e + 12);
                out.push((
                    String::from_utf8_lossy(&name[..end]).into_owned(),
                    vaddr,
                    vsize,
                ));
            }
        }
        Some(out)
    }

    /// Find an import slot (`FirstThunk` entry) in a loaded module.
    ///
    /// `dll` matches case-insensitively with or without a `.dll` suffix.
    /// `name` is the imported symbol name, or `ordinal:<n>`.
    #[must_use]
    pub fn find_import_slot(base: usize, dll: &str, name: &str) -> Option<usize> {
        let (opt, _, _) = headers(base)?;
        unsafe {
            let magic = ru16(base, opt);
            let entry = opt + if magic == 0x10B { 104 } else { 120 }; // import dir
            let imp_rva = ru32(base, entry) as usize;
            if imp_rva == 0 {
                return None;
            }
            let want_ord: Option<u16> = name.strip_prefix("ordinal:").and_then(|n| n.parse().ok());
            let mut desc = imp_rva;
            for _ in 0..256 {
                let oft = ru32(base, desc) as usize;
                let _stamp = ru32(base, desc + 4);
                let _fwd = ru32(base, desc + 8);
                let name_rva = ru32(base, desc + 12);
                let ft = ru32(base, desc + 16) as usize;
                if oft == 0 && name_rva == 0 && ft == 0 {
                    break;
                }
                desc += 20;
                let dll_name = rcstr(base, name_rva)?;
                let stem = dll_name
                    .strip_suffix(".dll")
                    .or_else(|| dll_name.strip_suffix(".DLL"))
                    .unwrap_or(&dll_name);
                let want = dll
                    .strip_suffix(".dll")
                    .or_else(|| dll.strip_suffix(".DLL"))
                    .unwrap_or(dll);
                if !stem.eq_ignore_ascii_case(want) {
                    continue;
                }
                // Walk thunks. Prefer OriginalFirstThunk for names; fall back
                // to FirstThunk when bound or when OFT is absent.
                for i in 0..4096usize {
                    let ft_entry = ru32(base, ft + i * 4);
                    if ft_entry == 0 {
                        break;
                    }
                    let name_entry = if oft != 0 {
                        ru32(base, oft + i * 4)
                    } else {
                        ft_entry
                    };
                    if oft == 0 && name_entry & 0x8000_0000 != 0 {
                        continue; // bound slot, name lost
                    }
                    if let Some(want) = want_ord {
                        let is_ord = name_entry & 0x8000_0000 != 0;
                        let ord = (name_entry & 0xFFFF) as u16;
                        if is_ord && ord == want {
                            return Some(base + ft + i * 4);
                        }
                    } else if name_entry & 0x8000_0000 == 0 && name_entry != 0 {
                        let sym = rcstr(base, name_entry + 2)?;
                        if sym.eq_ignore_ascii_case(name) {
                            return Some(base + ft + i * 4);
                        }
                    }
                }
                return None;
            }
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal PE32 image (as a file) with the given exports, so the
    /// export-table reader can be tested without any real DLL. Each export is
    /// `(name or None, ordinal)`; function RVAs are assigned in order. One
    /// export is made a forwarder (its function RVA points inside the export
    /// directory) to exercise `is_forwarder`.
    fn build_pe(exports: &[(Option<&str>, u16)], base_ordinal: u16, forwarder: Option<u16>) -> Vec<u8> {
        // Layout: headers in section ".idata" at RVA 0x1000, raw 0x200.
        let pe = 0x80usize;
        let opt_size = 0xE0usize; // PE32 optional header: 0x60 standard fields + 16 * 8 data directories = 0xE0
        let sec_table = pe + 24 + opt_size;
        let raw_base = 0x200usize;
        let rva_base = 0x1000u32;
        let mut img = vec![0u8; raw_base];
        img[0..2].copy_from_slice(b"MZ");
        img[0x3C..0x40].copy_from_slice(&(pe as u32).to_le_bytes());
        img[pe..pe + 4].copy_from_slice(b"PE\0\0");
        img[pe + 6..pe + 8].copy_from_slice(&1u16.to_le_bytes()); // one section
        img[pe + 20..pe + 22].copy_from_slice(&(opt_size as u16).to_le_bytes());
        img[pe + 24..pe + 26].copy_from_slice(&0x10Bu16.to_le_bytes()); // PE32 magic

        // Section ".idata": vsize, vaddr, rawsize, rawptr.
        let mut sec = [0u8; 40];
        sec[..6].copy_from_slice(b".idata");
        sec[8..12].copy_from_slice(&0x1000u32.to_le_bytes());
        sec[12..16].copy_from_slice(&rva_base.to_le_bytes());
        sec[16..20].copy_from_slice(&0x1000u32.to_le_bytes());
        sec[20..24].copy_from_slice(&(raw_base as u32).to_le_bytes());
        img[sec_table..sec_table + 40].copy_from_slice(&sec);

        // Build the export section body at raw_base / rva_base.
        let count = exports.len();
        let dir = raw_base; // export directory at the section start
        let funcs = dir + 40;
        let names = funcs + count * 4;
        let ords = names + count * 4;
        let mut strings = ords + count * 2;
        img.resize(strings + 256, 0);
        let off_to_rva = |off: usize| rva_base + (off - raw_base) as u32;

        let named: Vec<&(Option<&str>, u16)> = exports.iter().filter(|e| e.0.is_some()).collect();
        // Export directory fields the reader uses.
        img[dir + 16..dir + 20].copy_from_slice(&(base_ordinal as u32).to_le_bytes());
        img[dir + 20..dir + 24].copy_from_slice(&(count as u32).to_le_bytes());
        img[dir + 24..dir + 28].copy_from_slice(&(named.len() as u32).to_le_bytes());
        img[dir + 28..dir + 32].copy_from_slice(&off_to_rva(funcs).to_le_bytes());
        img[dir + 32..dir + 36].copy_from_slice(&off_to_rva(names).to_le_bytes());
        img[dir + 36..dir + 40].copy_from_slice(&off_to_rva(ords).to_le_bytes());

        let exp_rva = off_to_rva(dir);
        let exp_size = (strings + 256 - dir) as u32;
        // Optional-header export data directory (entry 0, at opt + 96 in PE32): points at the table above.
        let opt = pe + 24;
        img[opt + 96..opt + 100].copy_from_slice(&exp_rva.to_le_bytes());
        img[opt + 100..opt + 104].copy_from_slice(&exp_size.to_le_bytes());
        for (i, (_, ordinal)) in exports.iter().enumerate() {
            let func_index = ordinal - base_ordinal;
            let func_rva = if Some(*ordinal) == forwarder {
                exp_rva + 4 // inside the export directory: a forwarder
            } else {
                0x2000 + i as u32 * 0x10
            };
            img[funcs + func_index as usize * 4..funcs + func_index as usize * 4 + 4]
                .copy_from_slice(&func_rva.to_le_bytes());
        }
        // Names and name-ordinal table (sorted by name as a real linker emits; order does not matter to the reader).
        for (i, (name, ordinal)) in named.iter().enumerate() {
            let name = name.unwrap();
            let name_rva = off_to_rva(strings);
            img[names + i * 4..names + i * 4 + 4].copy_from_slice(&name_rva.to_le_bytes());
            let ord_index = ordinal - base_ordinal;
            img[ords + i * 2..ords + i * 2 + 2].copy_from_slice(&ord_index.to_le_bytes());
            img[strings..strings + name.len()].copy_from_slice(name.as_bytes());
            strings += name.len() + 1;
        }
        img
    }

    #[test]
    fn reads_named_and_ordinal_exports() {
        let img = build_pe(
            &[(Some("timeGetTime"), 10), (Some("mmioOpenW"), 11), (None, 12), (Some("waveOutOpen"), 13)],
            10,
            None,
        );
        let mut exports = file_exports(&img).expect("parse");
        exports.sort_by_key(|e| e.ordinal);
        assert_eq!(exports.len(), 4);
        assert_eq!(exports[0].name.as_deref(), Some("timeGetTime"));
        assert_eq!(exports[0].ordinal, 10);
        assert!(!exports[0].is_forwarder);
        assert_eq!(exports[2].name, None); // ordinal-only export at 12
        assert_eq!(exports[2].ordinal, 12);
        assert_eq!(exports[3].name.as_deref(), Some("waveOutOpen"));
        assert_eq!(exports[3].ordinal, 13);
    }

    #[test]
    fn forwarders_are_flagged() {
        let img = build_pe(&[(Some("a"), 1), (Some("b"), 2)], 1, Some(2));
        let exports = file_exports(&img).expect("parse");
        let b = exports.iter().find(|e| e.name.as_deref() == Some("b")).unwrap();
        assert!(b.is_forwarder);
        let a = exports.iter().find(|e| e.name.as_deref() == Some("a")).unwrap();
        assert!(!a.is_forwarder);
    }

    #[test]
    fn ordinal_base_offsets_the_ordinal() {
        // winmm's real export base is not 1; the reader must add it.
        let img = build_pe(&[(Some("only"), 7)], 7, None);
        let exports = file_exports(&img).expect("parse");
        assert_eq!(exports[0].ordinal, 7);
    }

    #[test]
    fn non_pe_input_is_rejected() {
        assert!(file_exports(b"not a pe at all").is_none());
        assert!(file_exports(&[]).is_none());
        let mut img = build_pe(&[(Some("a"), 1)], 1, None);
        img[0] = b'Z'; // break the MZ signature
        assert!(file_exports(&img).is_none());
    }

    #[test]
    fn cstr_is_bounded_and_rva_mapping_matches_sections() {
        let img = build_pe(&[(Some("name"), 1)], 1, None);
        // The one section maps RVA 0x1000 to raw 0x200.
        let (secs, opt) = file_section_table(&img).unwrap();
        assert_eq!(secs.rva_to_off(0x1000), Some(0x200));
        assert_eq!(secs.rva_to_off(0x1004), Some(0x204));
        assert_eq!(secs.rva_to_off(0), None);
        // Header RVAs below the first section map 1:1.
        assert_eq!(secs.rva_to_off(0x80), Some(0x80));
        assert!(opt > 0);
        // A string with no terminator inside the buffer returns None (ran off the end)...
        assert_eq!(cstr_at(&[b'x'; 40], 0), None);
        // ...and a very long run stops at the 512-byte cap rather than reading forever.
        let mut capped = vec![b'a'; 600];
        capped.push(0);
        assert_eq!(cstr_at(&capped, 0).map(|s| s.len()), Some(512));
    }
}
