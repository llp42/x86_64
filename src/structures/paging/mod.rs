//! Abstractions for page tables and other paging related structures.
//!
//! Page tables translate virtual memory “pages” to physical memory “frames”.
//!
//! # Accessing live tables
//!
//! A page table that the CPU can walk is modified by the hardware too: the CPU
//! sets the `ACCESSED` and `DIRTY` bits of live entries on its own, and other
//! CPUs may walk the same tables. Entries are therefore stored in an
//! [`AtomicU64`](core::sync::atomic::AtomicU64), and the right access depends
//! on what you are doing:
//!
//! - Building a table that nothing can reach yet, or one you hold the lock on:
//!   the `&mut` setters on [`PageTableEntry`](page_table::PageTableEntry) are
//!   relaxed stores and are enough.
//! - Reading a live entry: the accessors use
//!   [`Ordering::Relaxed`](core::sync::atomic::Ordering), which is what the
//!   hardware's accessed/dirty updates need.
//! - Changing a live entry that others may also touch: use `fetch_or`,
//!   `fetch_and` or `compare_exchange` so an accessed or dirty bit the CPU set
//!   in between is not overwritten.
//! - Publishing a new table to other CPUs: store the parent entry with
//!   [`Ordering::Release`](core::sync::atomic::Ordering) and read it back with
//!   [`Ordering::Acquire`](core::sync::atomic::Ordering).
//!
//! Changing an entry does not flush the TLB. Before relying on a new mapping in
//! a table the CPU has already walked, invalidate the old translation (see
//! `instructions::tlb`).

pub use self::frame::PhysFrame;
pub use self::frame_alloc::{FrameAllocator, FrameDeallocator};
#[doc(no_inline)]
pub use self::mapper::MappedPageTable;
#[cfg(all(feature = "instructions", target_arch = "x86_64"))]
#[doc(no_inline)]
pub use self::mapper::RecursivePageTable;
pub use self::mapper::{Mapper, Translate};
#[cfg(target_pointer_width = "64")]
#[doc(no_inline)]
pub use self::mapper::{OffsetPageTable, PhysOffset};
pub use self::page::{Page, PageSize, Size1GiB, Size2MiB, Size4KiB};
pub use self::page_table::{PageOffset, PageTable, PageTableFlags, PageTableIndex};

pub mod frame;
mod frame_alloc;
pub mod mapper;
pub mod page;
pub mod page_table;
