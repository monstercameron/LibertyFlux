//! Property, round-trip and fuzz tests on generated archives. Every byte is
//! generated here (see `crates/formats/tests/support.rs`), the keys are
//! random test keys, and no game files are needed.
//!
//! - Round trip: `encrypt_tables` then `decrypt_tables` is the identity for
//!   any length, and a generated tree written as RPF2, RPF3 or IMG (plain
//!   or encrypted, stored, deflated or resource entries) reads back with the
//!   same paths, sizes, flags and payload bytes.
//! - Invariants: every entry's payload range lies inside the file when the
//!   archive opens, and directory entries precede their children.
//! - Fuzz: mutated headers and tables (re-encrypted where the fixture is
//!   encrypted, so the mutation reaches the parser) never panic, whether
//!   opened or read entry by entry.

// Fixture builders narrow random words and lengths into smaller fields on
// purpose, and the property checks compare floats that were written
// bit-exactly, so these pedantic lints do not apply here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::format_collect,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::many_single_char_names,
    clippy::similar_names
)]

#[path = "../../tests/support.rs"]
mod support;

use std::collections::HashMap;
use std::io::{Cursor, Write};

use flate2::Compression;
use flate2::write::DeflateEncoder;
use lf_archive::{Archive, EntryKind, Key, crypto, hash::name_hash, img, rpf};
use support::{Buf, Rng, fuzz};

/// RPF payloads are placed on this alignment (resource offsets need 256).
const RPF_PAYLOAD_ALIGN: usize = 256;
/// IMG payloads are addressed in blocks of this size.
const IMG_BLOCK: usize = 0x800;
/// Directory marker bit in an RPF directory record's third word.
const RPF_DIR_BIT: u32 = 0x8000_0000;
/// Compressed flag in an RPF file record's control word.
const RPF_COMPRESSED: u32 = 0x4000_0000;
/// Both top bits set: the RPF file record is a resource.
const RPF_RESOURCE: u32 = 0xC000_0000;

/// How a generated file is stored.
#[derive(Clone, Debug)]
enum Storage {
    Stored,
    Deflated,
    /// A resource entry with this type id (low byte of the offset word).
    Resource(u8),
}

/// A generated tree node.
#[derive(Clone, Debug)]
enum Node {
    Dir(String, Vec<Node>),
    File(String, Vec<u8>, Storage),
}

impl Node {
    fn name(&self) -> &str {
        match self {
            Node::Dir(n, _) | Node::File(n, _, _) => n,
        }
    }
}

fn random_tree(rng: &mut Rng, depth: usize, allow_deflate: bool) -> Vec<Node> {
    let n = rng.below(5);
    let mut out: Vec<Node> = Vec::new();
    for i in 0..n {
        // Unique names per directory: a random stem plus the index.
        let stem = rng.ident(1, 10);
        if depth > 0 && rng.chance(1, 3) {
            let children = random_tree(rng, depth - 1, allow_deflate);
            out.push(Node::Dir(format!("{stem}{i}"), children));
        } else {
            let len = rng.below(600);
            let data = rng.bytes(len);
            let storage = match rng.below(3) {
                0 if allow_deflate => Storage::Deflated,
                1 => Storage::Resource(*rng.pick(&[0x01, 0x08, 0x6E])),
                _ => Storage::Stored,
            };
            out.push(Node::File(format!("{stem}{i}.bin"), data, storage));
        }
    }
    out
}

fn deflate(data: &[u8]) -> Vec<u8> {
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
    enc.write_all(data).unwrap();
    enc.finish().unwrap()
}

/// What a reader should report for one entry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Expect {
    path: String,
    is_dir: bool,
    data: Vec<u8>,
    compressed: bool,
    resource_type: Option<i32>,
}

/// Write `root` as an RPF archive. Records are laid out breadth-first, so
/// each directory's children occupy one contiguous record range as the
/// format requires. Returns the file and the expected entries.
fn write_rpf(
    root: &[Node],
    version: u8,
    key: Option<&Key>,
    content_encrypted: bool,
) -> (Vec<u8>, Vec<Expect>) {
    let root_node = Node::Dir("/".to_string(), root.to_vec());
    // Flatten breadth-first: (node, path, first child, child count).
    let mut flat: Vec<(Node, String, usize, usize)> = vec![(root_node, "/".to_string(), 0, 0)];
    let mut i = 0;
    while i < flat.len() {
        if let Node::Dir(_, children) = flat[i].0.clone() {
            let first = flat.len();
            let base = flat[i].1.clone();
            for c in &children {
                let path = if base == "/" {
                    format!("/{}", c.name())
                } else {
                    format!("{base}/{}", c.name())
                };
                flat.push((c.clone(), path, 0, 0));
            }
            flat[i].2 = first;
            flat[i].3 = children.len();
        }
        i += 1;
    }
    let count = flat.len();
    // Name block (RPF2 real names; RPF3 holds only the root).
    let mut names = Buf::new();
    let mut name_refs = Vec::with_capacity(count);
    for (node, _, _, _) in &flat {
        if version == 2 {
            name_refs.push(u32::try_from(names.len()).unwrap());
            names.bytes(node.name().as_bytes()).u8(0);
        } else {
            name_refs.push(name_hash(node.name()));
        }
    }
    if version == 3 {
        names.bytes(b"/\0");
        // The root carries a filler value instead of a hash.
        name_refs[0] = 0xFFFF_FFFF;
    }
    names.align(16);
    let toc_size = count * 16 + names.len();
    // Payloads after the table.
    let mut payload_at = (0x800 + toc_size).next_multiple_of(RPF_PAYLOAD_ALIGN);
    let mut payloads: Vec<(usize, Vec<u8>)> = Vec::new();
    let mut toc = Buf::new();
    let mut expect = Vec::new();
    for (idx, (node, path, first, n)) in flat.iter().enumerate() {
        match node {
            Node::Dir(..) => {
                toc.u32(name_refs[idx])
                    .u32(0)
                    .u32(RPF_DIR_BIT | u32::try_from(*first).unwrap())
                    .u32(u32::try_from(*n).unwrap());
                let shown = if version == 3 {
                    rpf3_dir_path(path)
                } else if path == "/" {
                    "/".to_string()
                } else {
                    format!("{path}/")
                };
                expect.push(Expect {
                    path: shown,
                    is_dir: true,
                    data: Vec::new(),
                    compressed: false,
                    resource_type: None,
                });
            }
            Node::File(_, data, storage) => {
                let mut stored = match storage {
                    Storage::Deflated => deflate(data),
                    _ => data.clone(),
                };
                if content_encrypted {
                    crypto::encrypt_tables(&mut stored, key.expect("content key"));
                }
                let size = u32::try_from(data.len()).unwrap();
                let at = u32::try_from(payload_at).unwrap();
                let stored_len = u32::try_from(stored.len()).unwrap();
                let (offset, control, resource_type) = match storage {
                    Storage::Stored => (at, stored_len, None),
                    Storage::Deflated => (at, stored_len | RPF_COMPRESSED, None),
                    Storage::Resource(t) => (at | u32::from(*t), RPF_RESOURCE, Some(i32::from(*t))),
                };
                toc.u32(name_refs[idx]).u32(size).u32(offset).u32(control);
                let shown = if version == 3 {
                    rpf3_file_path(path)
                } else {
                    path.clone()
                };
                expect.push(Expect {
                    path: shown,
                    is_dir: false,
                    data: data.clone(),
                    compressed: matches!(storage, Storage::Deflated),
                    resource_type,
                });
                payload_at = (payload_at + stored.len()).next_multiple_of(RPF_PAYLOAD_ALIGN);
                payloads.push((usize::try_from(at).unwrap(), stored));
            }
        }
    }
    toc.bytes(&names.0);
    assert_eq!(toc.len(), toc_size);
    let mut toc = toc.0;
    if let Some(k) = key {
        crypto::encrypt_tables(&mut toc, k);
    }
    let mut file = Buf::new();
    file.bytes(if version == 2 { b"RPF2" } else { b"RPF3" })
        .u32(u32::try_from(toc_size).unwrap())
        .u32(u32::try_from(count).unwrap())
        .u32(0)
        .u32(u32::from(key.is_some()))
        .u32(u32::from(content_encrypted));
    file.put(0x800, &toc);
    for (at, bytes) in payloads {
        file.put(at, &bytes);
    }
    file.align(16);
    (file.0, expect)
}

/// The placeholder path an RPF3 file entry gets before names resolve.
fn rpf3_file_path(path: &str) -> String {
    let mut out = String::from("/<hash:FFFFFFFF>");
    for seg in path.split('/').filter(|s| !s.is_empty()) {
        out.push_str(&format!("/<hash:{:08X}>", name_hash(seg)));
    }
    out
}

/// The placeholder path an RPF3 directory entry gets before names resolve.
fn rpf3_dir_path(path: &str) -> String {
    format!("{}/", rpf3_file_path(path))
}

/// Collect what the reader reports, payloads included.
fn read_back<A: Archive>(archive: &A, file: &[u8], key: Option<&Key>) -> Vec<Expect> {
    let mut reader = Cursor::new(file);
    archive
        .entries()
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let data = if e.kind == EntryKind::File {
                assert!(e.offset + e.stored_size <= file.len() as u64, "{}", e.path);
                archive
                    .read_file(&mut reader, i, key)
                    .expect("payload reads")
            } else {
                Vec::new()
            };
            Expect {
                path: e.path.clone(),
                is_dir: e.is_dir(),
                data,
                compressed: e.compressed,
                resource_type: e.resource.map(|r| r.type_id),
            }
        })
        .collect()
}

fn random_key(rng: &mut Rng) -> Key {
    let mut key = [0u8; 32];
    key.copy_from_slice(&rng.bytes(32));
    key
}

#[test]
fn table_cipher_round_trips_any_length() {
    let mut rng = Rng::for_test("archive cipher");
    for _ in 0..64 {
        let key = random_key(&mut rng);
        let len = rng.below(200);
        let plain = rng.bytes(len);
        let mut data = plain.clone();
        crypto::encrypt_tables(&mut data, &key);
        let full = len & !0x0F;
        // The tail under 16 bytes passes through unchanged.
        assert_eq!(data[full..], plain[full..]);
        if full > 0 {
            assert_ne!(data[..full], plain[..full]);
        }
        crypto::decrypt_tables(&mut data, &key);
        assert_eq!(data, plain);
    }
}

#[test]
fn rpf2_round_trip() {
    let mut rng = Rng::for_test("archive rpf2");
    for case in 0..60 {
        let tree = random_tree(&mut rng, 3, true);
        let key = random_key(&mut rng);
        let encrypt = case % 2 == 1;
        let content = case % 6 == 5;
        let used_key = (encrypt || content).then_some(&key);
        let (file, mut want) = write_rpf(&tree, 2, encrypt.then_some(&key), content);
        let archive = rpf::RpfArchive::open(&mut Cursor::new(&file), used_key).expect("opens");
        assert_eq!(archive.header().version, 2);
        assert_eq!(archive.header().toc_encrypted, encrypt);
        assert_eq!(archive.header().content_encrypted, content);
        assert_eq!(archive.len(), want.len());
        // Directories come before their children.
        for (i, e) in archive.entries().iter().enumerate() {
            if e.is_dir() && e.path != "/" {
                assert!(
                    archive.entries()[..i]
                        .iter()
                        .all(|p| !p.path.starts_with(&e.path)),
                    "{} listed after a child",
                    e.path
                );
            }
        }
        let mut got = read_back(&archive, &file, used_key);
        got.sort();
        want.sort();
        assert_eq!(got, want);
        // The generic opener agrees.
        let any = lf_archive::open(&mut Cursor::new(&file), used_key).unwrap();
        assert_eq!(any.len(), archive.len());
        assert_eq!(
            lf_archive::detect(&file[..20], None),
            Some(lf_archive::Kind::Rpf)
        );
    }
}

#[test]
fn rpf3_round_trip_and_name_resolution() {
    let mut rng = Rng::for_test("archive rpf3");
    for case in 0..40 {
        let tree = random_tree(&mut rng, 2, true);
        let key = random_key(&mut rng);
        let encrypt = case % 2 == 0;
        let (file, mut want) = write_rpf(&tree, 3, encrypt.then_some(&key), false);
        let mut archive =
            rpf::RpfArchive::open(&mut Cursor::new(&file), encrypt.then_some(&key)).unwrap();
        let mut got = read_back(&archive, &file, None);
        got.sort();
        want.sort();
        assert_eq!(got, want);
        // Resolve every name: paths become the RPF2 paths under the
        // unresolved root placeholder.
        let (_, mut plain) = write_rpf(&tree, 2, None, false);
        let mut names = HashMap::new();
        collect_names(&tree, &mut names);
        let resolved = archive.resolve_names(&names);
        assert_eq!(
            resolved,
            archive.len() - 1,
            "every entry but the root resolves"
        );
        let mut paths: Vec<String> = archive.entries().iter().map(|e| e.path.clone()).collect();
        paths.sort();
        let mut want_paths: Vec<String> = plain
            .drain(..)
            .map(|e| {
                if e.path == "/" {
                    "/<hash:FFFFFFFF>/".to_string()
                } else {
                    format!("/<hash:FFFFFFFF>{}", e.path)
                }
            })
            .collect();
        want_paths.sort();
        assert_eq!(paths, want_paths);
    }
}

fn collect_names(nodes: &[Node], out: &mut HashMap<u32, String>) {
    for n in nodes {
        out.insert(name_hash(n.name()), n.name().to_string());
        if let Node::Dir(_, c) = n {
            collect_names(c, out);
        }
    }
}

/// Write a flat list of files as an IMG v3 archive.
fn write_img(files: &[(String, Vec<u8>, Option<u8>)], key: Option<&Key>) -> Vec<u8> {
    let mut table = Buf::new();
    let mut names = Buf::new();
    for (name, _, _) in files {
        names.bytes(name.as_bytes()).u8(0);
    }
    let table_size = files.len() * 16 + names.len();
    let mut block = (20 + table_size).div_ceil(IMG_BLOCK);
    let mut payloads = Vec::new();
    for (_, data, resource) in files {
        let blocks = data.len().div_ceil(IMG_BLOCK).max(1);
        let padding = blocks * IMG_BLOCK - data.len();
        match resource {
            Some(t) => {
                table
                    .u32(RPF_RESOURCE)
                    .u32(u32::from(*t))
                    .u32(u32::try_from(block).unwrap())
                    .u16(u16::try_from(blocks).unwrap())
                    .u16(u16::try_from(padding).unwrap());
            }
            None => {
                table
                    .u32(u32::try_from(data.len()).unwrap())
                    .u32(0)
                    .u32(u32::try_from(block).unwrap())
                    .u16(u16::try_from(blocks).unwrap())
                    .u16(0);
            }
        }
        payloads.push((block * IMG_BLOCK, data.clone()));
        block += blocks;
    }
    table.bytes(&names.0);
    let mut header = Buf::new();
    header
        .u32(img::IMG_MAGIC)
        .u32(img::IMG_VERSION)
        .u32(u32::try_from(files.len()).unwrap())
        .u32(u32::try_from(table_size).unwrap())
        .u16(16)
        .u16(233);
    let mut header = header.0;
    let mut table = table.0;
    if let Some(k) = key {
        crypto::encrypt_tables(&mut header[0..16], k);
        crypto::encrypt_tables(&mut table, k);
    }
    let mut file = Buf::new();
    file.bytes(&header).bytes(&table);
    for (at, data) in payloads {
        file.put(at, &data);
    }
    file.put(block * IMG_BLOCK - 1, &[0]);
    file.0
}

fn random_img_files(rng: &mut Rng) -> Vec<(String, Vec<u8>, Option<u8>)> {
    (0..rng.below(8))
        .map(|i| {
            let len = rng.below(5000);
            let resource = rng.chance(1, 2).then_some(*rng.pick(&[0x01u8, 0x08, 0x20]));
            (
                format!("{}{i}.dat", rng.ident(1, 12)),
                rng.bytes(len),
                resource,
            )
        })
        .collect()
}

#[test]
fn img_round_trip() {
    let mut rng = Rng::for_test("archive img");
    for case in 0..60 {
        let files = random_img_files(&mut rng);
        let key = random_key(&mut rng);
        let encrypt = case % 2 == 1;
        let file = write_img(&files, encrypt.then_some(&key));
        let archive = img::ImgArchive::open(&mut Cursor::new(&file), encrypt.then_some(&key))
            .expect("generated IMG opens");
        assert_eq!(archive.header().encrypted, encrypt);
        assert_eq!(archive.header().entry_count as usize, files.len());
        let got = read_back(&archive, &file, None);
        assert_eq!(got.len(), files.len());
        for (g, (name, data, resource)) in got.iter().zip(&files) {
            assert_eq!(g.path, format!("/{name}"));
            assert_eq!(&g.data, data);
            assert_eq!(g.resource_type, resource.map(i32::from));
        }
        let detected = lf_archive::detect(&file[..20], encrypt.then_some(&key));
        let want = if encrypt {
            lf_archive::Kind::EncryptedImg
        } else {
            lf_archive::Kind::Img
        };
        assert_eq!(detected, Some(want));
    }
}

/// Open and read every entry; nothing may panic. On success the entries
/// must satisfy the reader's own invariants.
fn exercise(file: &[u8], key: Option<&Key>) {
    let _ = lf_archive::detect(file, key);
    let Ok(archive) = lf_archive::open(&mut Cursor::new(file), key) else {
        return;
    };
    let mut reader = Cursor::new(file);
    for (i, e) in archive.entries().iter().enumerate() {
        assert!(e.path.starts_with('/'), "path {:?}", e.path);
        if e.is_dir() {
            assert_eq!((e.size, e.stored_size, e.offset), (0, 0, 0));
        }
        if let Ok(data) = archive.read_file(&mut reader, i, key)
            && !e.compressed
        {
            assert_eq!(data.len() as u64, e.stored_size);
        }
    }
    let _ = archive.read_raw(&mut reader, usize::MAX, key);
}

/// RPF fixtures are fuzzed in a compact form without the zero padding
/// between the 24-byte header and the table at 0x800, so mutations land
/// on meaningful bytes; `expand_rpf` restores the padding.
fn compact_rpf(file: &[u8]) -> Vec<u8> {
    let mut out = file[..24].to_vec();
    out.extend_from_slice(&file[0x800..]);
    out
}

fn expand_rpf(compact: &[u8]) -> Vec<u8> {
    if compact.len() < 24 {
        return compact.to_vec();
    }
    let mut out = compact[..24].to_vec();
    out.resize(0x800, 0);
    out.extend_from_slice(&compact[24..]);
    out
}

fn fixed_tree(rng: &mut Rng) -> Vec<Node> {
    loop {
        let t = random_tree(rng, 3, true);
        if t.len() >= 3
            && t.iter()
                .any(|n| matches!(n, Node::Dir(_, c) if !c.is_empty()))
        {
            return t;
        }
    }
}

#[test]
fn fuzz_rpf2_plain() {
    let mut rng = Rng::for_test("archive fuzz rpf2 seeds");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| compact_rpf(&write_rpf(&fixed_tree(&mut rng), 2, None, false).0))
        .collect();
    fuzz("rpf2 plain", &seeds, 3000, |c| {
        exercise(&expand_rpf(c), None);
    });
}

#[test]
fn fuzz_rpf3_plain() {
    let mut rng = Rng::for_test("archive fuzz rpf3 seeds");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| compact_rpf(&write_rpf(&fixed_tree(&mut rng), 3, None, false).0))
        .collect();
    fuzz("rpf3 plain", &seeds, 2000, |c| {
        exercise(&expand_rpf(c), None);
    });
}

#[test]
fn fuzz_rpf_encrypted_table() {
    // Mutate the plaintext table, then encrypt it, so the decrypted table
    // the parser sees is the mutated one.
    let mut rng = Rng::for_test("archive fuzz rpf enc seeds");
    let key = random_key(&mut rng);
    let seed = compact_rpf(&write_rpf(&fixed_tree(&mut rng), 2, None, false).0);
    fuzz("rpf2 encrypted", &[seed], 1000, |c| {
        let mut file = expand_rpf(c);
        if file.len() >= 24 {
            file[16..20].copy_from_slice(&1u32.to_le_bytes());
            let toc_size = u32::from_le_bytes(file[4..8].try_into().unwrap()) as usize;
            let end = (0x800 + toc_size).min(file.len());
            if end > 0x800 {
                crypto::encrypt_tables(&mut file[0x800..end], &key);
            }
        }
        exercise(&file, Some(&key));
    });
}

#[test]
fn fuzz_img() {
    let mut rng = Rng::for_test("archive fuzz img seeds");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| {
            let mut files = random_img_files(&mut rng);
            files.push(("last.wdr".into(), vec![7; 100], Some(0x6E)));
            write_img(&files, None)
        })
        .collect();
    fuzz("img plain", &seeds, 3000, |f| exercise(f, None));
    let key = random_key(&mut rng);
    fuzz("img encrypted", &seeds, 1000, |f| {
        let mut file = f.to_vec();
        if file.len() >= 20 {
            let table_size = u32::from_le_bytes(file[12..16].try_into().unwrap()) as usize;
            crypto::encrypt_tables(&mut file[0..16], &key);
            let end = (20 + table_size).min(file.len());
            crypto::encrypt_tables(&mut file[20..end], &key);
        }
        exercise(&file, Some(&key));
    });
}

#[test]
fn fuzz_key_search() {
    let mut rng = Rng::for_test("archive key search");
    let seeds = vec![rng.bytes(4096)];
    fuzz("find_key", &seeds, 200, |b| {
        assert!(
            crypto::find_key(b).is_none(),
            "random bytes never hold the key"
        );
    });
}

// Regression inputs for the bugs the fuzzer found, reduced by hand.

#[test]
fn regression_rpf_directory_containing_itself_errors_or_lists_once() {
    // Root directory record 0 whose child range [0, 1) is itself: the walk
    // used to recurse until the stack overflowed.
    let mut toc = Buf::new();
    toc.u32(0)
        .u32(0)
        .u32(RPF_DIR_BIT)
        .u32(1)
        .bytes(b"/\0")
        .align(16);
    let mut file = Buf::new();
    file.bytes(b"RPF2")
        .u32(u32::try_from(toc.len()).unwrap())
        .u32(1)
        .u32(0)
        .u32(0)
        .u32(0);
    file.put(0x800, &toc.0);
    let archive = rpf::RpfArchive::open(&mut Cursor::new(&file.0), None).unwrap();
    assert_eq!(archive.len(), 1);
}

#[test]
fn regression_rpf_deep_nesting_is_an_error() {
    // 1000 directories each holding the next: deeper than any real archive.
    let n = 1000u32;
    let mut toc = Buf::new();
    for i in 0..n {
        let children = u32::from(i + 1 < n);
        toc.u32(0).u32(0).u32(RPF_DIR_BIT | (i + 1)).u32(children);
    }
    toc.bytes(b"d\0").align(16);
    let mut file = Buf::new();
    file.bytes(b"RPF2")
        .u32(u32::try_from(toc.len()).unwrap())
        .u32(n)
        .u32(0)
        .u32(0)
        .u32(0);
    file.put(0x800, &toc.0);
    let err = rpf::RpfArchive::open(&mut Cursor::new(&file.0), None).unwrap_err();
    assert!(matches!(err, lf_archive::Error::BadTree(_)), "{err}");
}

#[test]
fn regression_img_padding_larger_than_blocks_is_an_error() {
    // A resource record with zero blocks and non-zero padding used to
    // underflow the size computation.
    let mut file = Buf::new();
    file.u32(img::IMG_MAGIC)
        .u32(img::IMG_VERSION)
        .u32(1)
        .u32(18)
        .u16(16)
        .u16(0);
    file.u32(RPF_RESOURCE)
        .u32(1)
        .u32(1)
        .u16(0)
        .u16(8)
        .bytes(b"a\0");
    file.put(IMG_BLOCK * 2 - 1, &[0]);
    let err = img::ImgArchive::open(&mut Cursor::new(&file.0), None).unwrap_err();
    assert!(matches!(err, lf_archive::Error::BadEntry(_)), "{err}");
}

#[test]
fn regression_rpf3_resolved_directory_path_is_not_doubled() {
    // A resolved RPF3 directory used to get its name appended after the
    // trailing slash and then renamed again: "/<root>/name/name/".
    let tree = vec![Node::Dir(
        "dir".into(),
        vec![Node::File("f.bin".into(), vec![1, 2, 3], Storage::Stored)],
    )];
    let (file, _) = write_rpf(&tree, 3, None, false);
    let mut archive = rpf::RpfArchive::open(&mut Cursor::new(&file), None).unwrap();
    let mut names = HashMap::new();
    collect_names(&tree, &mut names);
    assert_eq!(archive.resolve_names(&names), 2);
    let paths: Vec<&str> = archive.entries().iter().map(|e| e.path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "/<hash:FFFFFFFF>/",
            "/<hash:FFFFFFFF>/dir/",
            "/<hash:FFFFFFFF>/dir/f.bin"
        ]
    );
}
