//! Property and fuzz tests on generated audio metadata. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); no game files
//! are needed.
//!
//! - Property (schema-driven): for every schema and every object type the
//!   schema describes, a random value tree is encoded from the field
//!   definitions (counted strings and arrays, fixed arrays, presence-masked
//!   optional fields, enums, trailing bytes), wrapped in a versioned
//!   container with archive names and relocations, and decodes back to the
//!   identical value tree.
//! - Property: random speech files (variation blob, contexts, voices,
//!   banks) parse back with every cross-reference resolved.
//! - Fuzz: mutated containers (decoded with every schema) and mutated
//!   speech files never panic.

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

use lf_audio_config::container::MetaFile;
use lf_audio_config::decode::{DecodedField, EnumValue, Value, decode_object};
use lf_audio_config::schema::{CountWidth, FieldDef, FieldKind, IntWidth, Schema};
use lf_audio_config::speech::{CONTEXT_SIZE, SpeechFile};
use support::{Buf, Rng, fuzz};

const SCHEMAS: [Schema; 5] = [
    Schema::Categories,
    Schema::Curves,
    Schema::Effects,
    Schema::Sounds,
    Schema::Game,
];

fn put_count(out: &mut Buf, width: CountWidth, n: usize) {
    match width {
        CountWidth::U8 => out.u8(n as u8),
        CountWidth::U16 => out.u16(n as u16),
        CountWidth::U32 => out.u32(n as u32),
    };
}

/// Encode a random value of `kind` into `out` and return what the decoder
/// must report for it.
fn gen_field(rng: &mut Rng, kind: &FieldKind, out: &mut Buf) -> Value {
    match kind {
        FieldKind::U8 => {
            let v = rng.next_u32() as u8;
            out.u8(v);
            Value::U8(v)
        }
        FieldKind::U16 => {
            let v = rng.next_u32() as u16;
            out.u16(v);
            Value::U16(v)
        }
        FieldKind::U32 => {
            let v = rng.next_u32();
            out.u32(v);
            Value::U32(v)
        }
        FieldKind::I8 => {
            let v = rng.next_u32() as i8;
            out.u8(v as u8);
            Value::I8(v)
        }
        FieldKind::I16 => {
            let v = rng.next_u32() as i16;
            out.u16(v as u16);
            Value::I16(v)
        }
        FieldKind::I32 => {
            let v = rng.next_u32() as i32;
            out.u32(v as u32);
            Value::I32(v)
        }
        FieldKind::F32 => {
            let v = rng.f32_in(-1000.0, 1000.0);
            out.f32(v);
            Value::F32(v)
        }
        FieldKind::Hash => {
            let v = rng.next_u32();
            out.u32(v);
            Value::Hash(v)
        }
        FieldKind::Text(width) => {
            let s = rng.ident(0, 12);
            put_count(out, *width, s.len());
            out.bytes(s.as_bytes());
            Value::Text(s)
        }
        FieldKind::Array { count, element } => {
            let n = rng.below(4);
            put_count(out, *count, n);
            Value::Array((0..n).map(|_| gen_element(rng, element, out)).collect())
        }
        FieldKind::FixedArray { count, element } => Value::Array(
            (0..*count)
                .map(|_| gen_element(rng, element, out))
                .collect(),
        ),
        FieldKind::OptionalBitfield(children) => {
            let usable = children.len().min(32);
            let mut mask = 0u32;
            for i in 0..usable {
                if rng.chance(1, 2) {
                    mask |= 1 << i;
                }
            }
            out.u32(mask);
            let present = children
                .iter()
                .enumerate()
                .filter(|(i, _)| *i < 32 && mask & (1 << i) != 0)
                .map(|(_, c)| DecodedField {
                    name: c.name.to_string(),
                    value: gen_field(rng, &c.kind, out),
                })
                .collect();
            Value::Fields(present)
        }
        FieldKind::Enum {
            base: IntWidth::U8,
            values,
        } => {
            let raw = if !values.is_empty() && rng.chance(3, 4) {
                rng.pick(values).0 as u8
            } else {
                rng.next_u32() as u8
            };
            out.u8(raw);
            let raw = i64::from(raw);
            let name = values.iter().find(|(v, _)| *v == raw).map(|(_, n)| *n);
            Value::Enum(EnumValue { raw, name })
        }
        FieldKind::Rest => {
            let n = rng.below(8);
            let bytes = rng.bytes(n);
            out.bytes(&bytes);
            Value::Bytes(bytes)
        }
    }
}

fn gen_element(rng: &mut Rng, element: &[FieldDef], out: &mut Buf) -> Value {
    if element.len() == 1 && element[0].name.is_empty() {
        return gen_field(rng, &element[0].kind, out);
    }
    Value::Fields(gen_fields(rng, element, out))
}

fn gen_fields(rng: &mut Rng, fields: &[FieldDef], out: &mut Buf) -> Vec<DecodedField> {
    fields
        .iter()
        .map(|f| DecodedField {
            name: f.name.to_string(),
            value: gen_field(rng, &f.kind, out),
        })
        .collect()
}

/// A versioned container holding `objects` (name, body bytes), with a few
/// archive names and relocations pointing into the blob.
fn write_container(suffix: u32, objects: &[(String, Vec<u8>)], archives: &[String]) -> Vec<u8> {
    let mut blob = Buf::new();
    blob.u8(0);
    let mut dir = Vec::new();
    for (name, body) in objects {
        dir.push((name.clone(), blob.len() as u32, body.len() as u32));
        blob.bytes(body);
    }
    blob.align(4);
    let mut b = Buf::new();
    b.u32(suffix).u32(blob.len() as u32);
    let blob_start = b.len();
    b.bytes(&blob.0);
    let mut heap = Buf::new();
    let offsets: Vec<u32> = archives
        .iter()
        .map(|a| {
            let at = heap.len() as u32;
            heap.bytes(a.as_bytes()).u8(0);
            at
        })
        .collect();
    b.u32((4 + 4 * archives.len() + heap.len()) as u32)
        .u32(archives.len() as u32);
    for o in offsets {
        b.u32(o);
    }
    b.bytes(&heap.0);
    b.u32(objects.len() as u32).u32(0);
    for (name, off, size) in &dir {
        b.u8(name.len() as u8)
            .bytes(name.as_bytes())
            .u32(*off)
            .u32(*size);
    }
    // One hash relocation and one archive relocation into the blob.
    let reloc = (blob_start as u32, blob.len() >= 4);
    b.u32(u32::from(reloc.1));
    if reloc.1 {
        b.u32(reloc.0);
    }
    b.u32(0);
    b.0
}

#[test]
fn every_object_type_round_trips() {
    let mut rng = Rng::for_test("audio config schema round trip");
    let mut types_checked = 0;
    for schema in SCHEMAS {
        let header = schema.header();
        for type_id in 0..=255u8 {
            let Some(fields) = schema.type_fields(type_id) else {
                continue;
            };
            types_checked += 1;
            for _ in 0..4 {
                let mut body = Buf::new();
                body.u8(type_id).u32(0);
                let want_header = gen_fields(&mut rng, &header, &mut body);
                let want_body = gen_fields(&mut rng, &fields, &mut body);
                let name = format!("obj_{}", rng.ident(1, 10));
                let archives = vec![format!("ARCHIVE\\{}", rng.ident(1, 8))];
                let file = write_container(schema.suffix(), &[(name.clone(), body.0)], &archives);
                let meta = MetaFile::parse(&file).expect("generated container parses");
                assert_eq!(meta.suffix(), schema.suffix());
                assert_eq!(meta.archives()[0].name(), archives[0]);
                assert_eq!(meta.objects().len(), 1);
                let entry = meta.find(&name).expect("object listed");
                assert_eq!(entry.type_id(), type_id);
                let obj = decode_object(&schema, entry).expect("generated object decodes");
                assert_eq!(obj.type_id, type_id);
                assert_eq!(obj.type_name, schema.type_name(type_id));
                assert_eq!(obj.header, want_header, "{schema:?} type {type_id} header");
                assert_eq!(obj.body, want_body, "{schema:?} type {type_id} body");
                assert!(obj.trailing.is_empty());
            }
        }
    }
    assert!(types_checked > 10, "schemas describe {types_checked} types");
}

fn random_speech(rng: &mut Rng) -> (Vec<u8>, usize, usize, Vec<String>) {
    let banks: Vec<String> = (0..rng.range(1, 4))
        .map(|i| format!("BANK_{i}{}", rng.ident(1, 6)))
        .collect();
    let n = rng.below(40);
    let variation = rng.bytes(n);
    let contexts: Vec<(u32, i32, u32, u8)> = (0..rng.below(8))
        .map(|_| {
            let count = rng.below(4) as u8;
            let off = if variation.len() >= usize::from(count) && rng.chance(3, 4) {
                rng.below(variation.len() - usize::from(count) + 1) as i32
            } else {
                -1
            };
            (rng.below(banks.len()) as u32, off, rng.next_u32(), count)
        })
        .collect();
    let mut voices = Vec::new();
    let mut first = 0;
    while first < contexts.len() {
        let n = rng.range(1, contexts.len() - first);
        voices.push((first, n));
        first += n;
    }
    let mut b = Buf::new();
    b.u32(variation.len() as u32)
        .bytes(&variation)
        .u32(contexts.len() as u32);
    for (bank, off, hash, count) in &contexts {
        b.u32(*bank).u32(*off as u32).u32(*hash).u8(0).u8(*count);
    }
    b.u32(voices.len() as u32);
    for (first, n) in &voices {
        b.u32((first * CONTEXT_SIZE) as u32)
            .u32(rng.next_u32())
            .u16(*n as u16);
    }
    b.u32(banks.len() as u32);
    let mut heap = Buf::new();
    for name in &banks {
        b.u32(heap.len() as u32);
        heap.bytes(name.as_bytes()).u8(0);
    }
    b.bytes(&heap.0);
    (b.0, contexts.len(), voices.len(), banks)
}

#[test]
fn speech_round_trip() {
    let mut rng = Rng::for_test("audio config speech");
    for _ in 0..100 {
        let (bytes, contexts, voices, banks) = random_speech(&mut rng);
        let f = SpeechFile::parse(&bytes).expect("generated speech file parses");
        assert_eq!(f.contexts().len(), contexts);
        assert_eq!(f.voices().len(), voices);
        assert_eq!(f.banks(), &banks[..]);
        let mut covered = 0;
        for v in f.voices() {
            let run = f.contexts_of(v);
            covered += run.len();
            for c in run {
                assert!(banks.iter().any(|b| b == f.bank_of(c)));
                assert!(f.variations_of(c).len() <= usize::from(c.variation_count()));
            }
        }
        assert_eq!(covered, contexts);
    }
}

#[test]
fn fuzz_containers_with_every_schema() {
    let mut rng = Rng::for_test("audio config fuzz");
    let seeds: Vec<Vec<u8>> = SCHEMAS
        .iter()
        .map(|schema| {
            let header = schema.header();
            let objects: Vec<(String, Vec<u8>)> = (0..=255u8)
                .filter_map(|t| schema.type_fields(t).map(|f| (t, f)))
                .take(3)
                .map(|(t, fields)| {
                    let mut body = Buf::new();
                    body.u8(t).u32(0);
                    gen_fields(&mut rng, &header, &mut body);
                    gen_fields(&mut rng, &fields, &mut body);
                    (format!("o{t}"), body.0)
                })
                .collect();
            write_container(schema.suffix(), &objects, &["A\\B".to_string()])
        })
        .collect();
    fuzz("audio config container", &seeds, 3000, |b| {
        let Ok(meta) = MetaFile::parse(b) else { return };
        for entry in meta.objects() {
            assert!(entry.data().len() >= 5);
            let _ = (entry.type_id(), entry.name_offset());
            for schema in SCHEMAS {
                let _ = decode_object(&schema, entry);
            }
        }
        let _ = meta.hash_offsets().len() + meta.archive_offsets().len();
    });
}

#[test]
fn fuzz_speech() {
    let mut rng = Rng::for_test("audio config speech fuzz");
    let seeds: Vec<Vec<u8>> = (0..4).map(|_| random_speech(&mut rng).0).collect();
    fuzz("speech", &seeds, 3000, |b| {
        if let Ok(f) = SpeechFile::parse(b) {
            for v in f.voices() {
                for c in f.contexts_of(v) {
                    let _ = (f.bank_of(c), f.variations_of(c));
                }
            }
        }
    });
}

#[test]
fn fuzz_object_bodies_in_valid_containers() {
    // Mutate one object's body and re-wrap it, so the container stays
    // valid and every mutation reaches the schema decoder.
    let mut rng = Rng::for_test("audio config body fuzz");
    for schema in SCHEMAS {
        let header = schema.header();
        let seeds: Vec<Vec<u8>> = (0..=255u8)
            .filter_map(|t| schema.type_fields(t).map(|f| (t, f)))
            .map(|(t, fields)| {
                let mut body = Buf::new();
                body.u8(t).u32(0);
                gen_fields(&mut rng, &header, &mut body);
                gen_fields(&mut rng, &fields, &mut body);
                body.0
            })
            .collect();
        fuzz(
            &format!("audio config {schema:?} bodies"),
            &seeds,
            1500,
            |body| {
                let file =
                    write_container(schema.suffix(), &[("o".to_string(), body.to_vec())], &[]);
                if let Ok(meta) = MetaFile::parse(&file) {
                    for entry in meta.objects() {
                        let _ = decode_object(&schema, entry);
                    }
                }
            },
        );
    }
}
