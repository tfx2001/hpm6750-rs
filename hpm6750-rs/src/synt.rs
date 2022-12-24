#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global control register"]
    pub gcr: GCR,
    #[doc = "0x04 - Counter reload register"]
    pub rld: RLD,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Counter"]
    pub cnt: CNT,
    _reserved3: [u8; 0x10],
    #[doc = "0x20 - Comparator"]
    pub cmp_0: CMP_0,
    #[doc = "0x24 - Comparator"]
    pub cmp_1: CMP_1,
    #[doc = "0x28 - Comparator"]
    pub cmp_2: CMP_2,
    #[doc = "0x2c - Comparator"]
    pub cmp_3: CMP_3,
}
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "RLD (rw) register accessor: an alias for `Reg<RLD_SPEC>`"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Counter reload register"]
pub mod rld;
#[doc = "CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "CMP_0 (rw) register accessor: an alias for `Reg<CMP_0_SPEC>`"]
pub type CMP_0 = crate::Reg<cmp_0::CMP_0_SPEC>;
#[doc = "Comparator"]
pub mod cmp_0;
#[doc = "CMP_1 (rw) register accessor: an alias for `Reg<CMP_1_SPEC>`"]
pub type CMP_1 = crate::Reg<cmp_1::CMP_1_SPEC>;
#[doc = "Comparator"]
pub mod cmp_1;
#[doc = "CMP_2 (rw) register accessor: an alias for `Reg<CMP_2_SPEC>`"]
pub type CMP_2 = crate::Reg<cmp_2::CMP_2_SPEC>;
#[doc = "Comparator"]
pub mod cmp_2;
#[doc = "CMP_3 (rw) register accessor: an alias for `Reg<CMP_3_SPEC>`"]
pub type CMP_3 = crate::Reg<cmp_3::CMP_3_SPEC>;
#[doc = "Comparator"]
pub mod cmp_3;
