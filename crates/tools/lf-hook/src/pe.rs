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
