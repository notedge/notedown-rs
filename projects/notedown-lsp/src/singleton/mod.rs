use notedown_rt::NoteVM;
use std::lazy::SyncLazy;

pub static VM: SyncLazy<NoteVM> = SyncLazy::new(|| NoteVM::default());
