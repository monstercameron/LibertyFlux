//! Lifted text tokenizer and streaming device.
//!
//! The original reads its text data files through a tokenizer object
//! ([`Tokenizer`]): callers pull integers, doubles, float vectors and raw
//! tokens out of a stream, and push brace-delimited, indented text back
//! through the same object when writing. Beside it sits the streaming
//! device ([`StreamDevice`]), which resolves entries by key and answers
//! per-kind table words, data spans and channel posts for them.
//!
//! Each type owns its data as ordinary Rust (bytes and words in vectors,
//! no addresses, no virtual tables, no allocator calls) and each verified
//! 32-bit method with behaviour in it is restated as a method on it.
//! Collaborators the rewrites reach through callee slots or virtual slots
//! (the entry resolver, the token fetcher, the number parsers, the stream
//! reader and writers, the channel objects) are carried as plain values
//! or opaque cookies and reached through one trait per class
//! ([`StreamWorld`], [`TokenWorld`]): every slot is one trait method, so
//! dispatch is static and tests script answers through a fake.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results, every written
//! byte, and every collaborator call in order, floats bit for bit (see
//! the `lf-tokendiff` test crate). Nothing here is verified by the
//! checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

pub mod registry;
pub mod stream_device;
pub mod tokenizer;

pub use stream_device::{
    ChannelTag, DataSpan, StreamChannel, StreamDevice, StreamEntry, StreamTables, StreamWorld,
};
pub use tokenizer::{
    ForwardSlot, TokenFetch, TokenRead, TokenStream, TokenWorld, Tokenizer,
};
