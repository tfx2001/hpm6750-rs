#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x08 - Machine Time"]
    pub mtime: MTIME,
    #[doc = "0x08..0x10 - Machine Time Compare"]
    pub mtimecmp: MTIMECMP,
}
#[doc = "MTIME (rw) register accessor: an alias for `Reg<MTIME_SPEC>`"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Machine Time"]
pub mod mtime;
#[doc = "MTIMECMP (rw) register accessor: an alias for `Reg<MTIMECMP_SPEC>`"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Machine Time Compare"]
pub mod mtimecmp;
