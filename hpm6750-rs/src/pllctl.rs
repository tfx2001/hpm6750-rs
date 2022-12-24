#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Crystal control and status"]
    pub xtal: XTAL,
    _reserved1: [u8; 0x7c],
    #[doc = "0x80 - PLLx config0"]
    pub pll_pll0_cfg0: PLL_PLL0_CFG0,
    #[doc = "0x84 - PLLx config1"]
    pub pll_pll0_cfg1: PLL_PLL0_CFG1,
    #[doc = "0x88 - PLLx config2"]
    pub pll_pll0_cfg2: PLL_PLL0_CFG2,
    #[doc = "0x8c - PLLx frac mode frequency adjust"]
    pub pll_pll0_freq: PLL_PLL0_FREQ,
    #[doc = "0x90 - PLLx lock control"]
    pub pll_pll0_lock: PLL_PLL0_LOCK,
    _reserved6: [u8; 0x0c],
    #[doc = "0xa0 - PLLx status"]
    pub pll_pll0_status: PLL_PLL0_STATUS,
    _reserved7: [u8; 0x1c],
    #[doc = "0xc0 - PLLx divider0 control"]
    pub pll_pll0_div0: PLL_PLL0_DIV0,
    #[doc = "0xc4 - PLLx divider1 control"]
    pub pll_pll0_div1: PLL_PLL0_DIV1,
    _reserved9: [u8; 0x38],
    #[doc = "0x100 - PLLx config0"]
    pub pll_pll1_cfg0: PLL_PLL1_CFG0,
    #[doc = "0x104 - PLLx config1"]
    pub pll_pll1_cfg1: PLL_PLL1_CFG1,
    #[doc = "0x108 - PLLx config2"]
    pub pll_pll1_cfg2: PLL_PLL1_CFG2,
    #[doc = "0x10c - PLLx frac mode frequency adjust"]
    pub pll_pll1_freq: PLL_PLL1_FREQ,
    #[doc = "0x110 - PLLx lock control"]
    pub pll_pll1_lock: PLL_PLL1_LOCK,
    _reserved14: [u8; 0x0c],
    #[doc = "0x120 - PLLx status"]
    pub pll_pll1_status: PLL_PLL1_STATUS,
    _reserved15: [u8; 0x1c],
    #[doc = "0x140 - PLLx divider0 control"]
    pub pll_pll1_div0: PLL_PLL1_DIV0,
    #[doc = "0x144 - PLLx divider1 control"]
    pub pll_pll1_div1: PLL_PLL1_DIV1,
    _reserved17: [u8; 0x38],
    #[doc = "0x180 - PLLx config0"]
    pub pll_pll2_cfg0: PLL_PLL2_CFG0,
    #[doc = "0x184 - PLLx config1"]
    pub pll_pll2_cfg1: PLL_PLL2_CFG1,
    #[doc = "0x188 - PLLx config2"]
    pub pll_pll2_cfg2: PLL_PLL2_CFG2,
    #[doc = "0x18c - PLLx frac mode frequency adjust"]
    pub pll_pll2_freq: PLL_PLL2_FREQ,
    #[doc = "0x190 - PLLx lock control"]
    pub pll_pll2_lock: PLL_PLL2_LOCK,
    _reserved22: [u8; 0x0c],
    #[doc = "0x1a0 - PLLx status"]
    pub pll_pll2_status: PLL_PLL2_STATUS,
    _reserved23: [u8; 0x1c],
    #[doc = "0x1c0 - PLLx divider0 control"]
    pub pll_pll2_div0: PLL_PLL2_DIV0,
    #[doc = "0x1c4 - PLLx divider1 control"]
    pub pll_pll2_div1: PLL_PLL2_DIV1,
    _reserved25: [u8; 0x38],
    #[doc = "0x200 - PLLx config0"]
    pub pll_pll3_cfg0: PLL_PLL3_CFG0,
    #[doc = "0x204 - PLLx config1"]
    pub pll_pll3_cfg1: PLL_PLL3_CFG1,
    #[doc = "0x208 - PLLx config2"]
    pub pll_pll3_cfg2: PLL_PLL3_CFG2,
    #[doc = "0x20c - PLLx frac mode frequency adjust"]
    pub pll_pll3_freq: PLL_PLL3_FREQ,
    #[doc = "0x210 - PLLx lock control"]
    pub pll_pll3_lock: PLL_PLL3_LOCK,
    _reserved30: [u8; 0x0c],
    #[doc = "0x220 - PLLx status"]
    pub pll_pll3_status: PLL_PLL3_STATUS,
    _reserved31: [u8; 0x1c],
    #[doc = "0x240 - PLLx divider0 control"]
    pub pll_pll3_div0: PLL_PLL3_DIV0,
    #[doc = "0x244 - PLLx divider1 control"]
    pub pll_pll3_div1: PLL_PLL3_DIV1,
    _reserved33: [u8; 0x38],
    #[doc = "0x280 - PLLx config0"]
    pub pll_pll4_cfg0: PLL_PLL4_CFG0,
    #[doc = "0x284 - PLLx config1"]
    pub pll_pll4_cfg1: PLL_PLL4_CFG1,
    #[doc = "0x288 - PLLx config2"]
    pub pll_pll4_cfg2: PLL_PLL4_CFG2,
    #[doc = "0x28c - PLLx frac mode frequency adjust"]
    pub pll_pll4_freq: PLL_PLL4_FREQ,
    #[doc = "0x290 - PLLx lock control"]
    pub pll_pll4_lock: PLL_PLL4_LOCK,
    _reserved38: [u8; 0x0c],
    #[doc = "0x2a0 - PLLx status"]
    pub pll_pll4_status: PLL_PLL4_STATUS,
    _reserved39: [u8; 0x1c],
    #[doc = "0x2c0 - PLLx divider0 control"]
    pub pll_pll4_div0: PLL_PLL4_DIV0,
    #[doc = "0x2c4 - PLLx divider1 control"]
    pub pll_pll4_div1: PLL_PLL4_DIV1,
}
#[doc = "XTAL (rw) register accessor: an alias for `Reg<XTAL_SPEC>`"]
pub type XTAL = crate::Reg<xtal::XTAL_SPEC>;
#[doc = "Crystal control and status"]
pub mod xtal;
#[doc = "PLL_PLL0_CFG0 (rw) register accessor: an alias for `Reg<PLL_PLL0_CFG0_SPEC>`"]
pub type PLL_PLL0_CFG0 = crate::Reg<pll_pll0_cfg0::PLL_PLL0_CFG0_SPEC>;
#[doc = "PLLx config0"]
pub mod pll_pll0_cfg0;
#[doc = "PLL_PLL0_CFG1 (rw) register accessor: an alias for `Reg<PLL_PLL0_CFG1_SPEC>`"]
pub type PLL_PLL0_CFG1 = crate::Reg<pll_pll0_cfg1::PLL_PLL0_CFG1_SPEC>;
#[doc = "PLLx config1"]
pub mod pll_pll0_cfg1;
#[doc = "PLL_PLL0_CFG2 (rw) register accessor: an alias for `Reg<PLL_PLL0_CFG2_SPEC>`"]
pub type PLL_PLL0_CFG2 = crate::Reg<pll_pll0_cfg2::PLL_PLL0_CFG2_SPEC>;
#[doc = "PLLx config2"]
pub mod pll_pll0_cfg2;
#[doc = "PLL_PLL0_FREQ (rw) register accessor: an alias for `Reg<PLL_PLL0_FREQ_SPEC>`"]
pub type PLL_PLL0_FREQ = crate::Reg<pll_pll0_freq::PLL_PLL0_FREQ_SPEC>;
#[doc = "PLLx frac mode frequency adjust"]
pub mod pll_pll0_freq;
#[doc = "PLL_PLL0_LOCK (rw) register accessor: an alias for `Reg<PLL_PLL0_LOCK_SPEC>`"]
pub type PLL_PLL0_LOCK = crate::Reg<pll_pll0_lock::PLL_PLL0_LOCK_SPEC>;
#[doc = "PLLx lock control"]
pub mod pll_pll0_lock;
#[doc = "PLL_PLL0_STATUS (r) register accessor: an alias for `Reg<PLL_PLL0_STATUS_SPEC>`"]
pub type PLL_PLL0_STATUS = crate::Reg<pll_pll0_status::PLL_PLL0_STATUS_SPEC>;
#[doc = "PLLx status"]
pub mod pll_pll0_status;
#[doc = "PLL_PLL0_DIV0 (rw) register accessor: an alias for `Reg<PLL_PLL0_DIV0_SPEC>`"]
pub type PLL_PLL0_DIV0 = crate::Reg<pll_pll0_div0::PLL_PLL0_DIV0_SPEC>;
#[doc = "PLLx divider0 control"]
pub mod pll_pll0_div0;
#[doc = "PLL_PLL0_DIV1 (rw) register accessor: an alias for `Reg<PLL_PLL0_DIV1_SPEC>`"]
pub type PLL_PLL0_DIV1 = crate::Reg<pll_pll0_div1::PLL_PLL0_DIV1_SPEC>;
#[doc = "PLLx divider1 control"]
pub mod pll_pll0_div1;
#[doc = "PLL_PLL1_CFG0 (rw) register accessor: an alias for `Reg<PLL_PLL1_CFG0_SPEC>`"]
pub type PLL_PLL1_CFG0 = crate::Reg<pll_pll1_cfg0::PLL_PLL1_CFG0_SPEC>;
#[doc = "PLLx config0"]
pub mod pll_pll1_cfg0;
#[doc = "PLL_PLL1_CFG1 (rw) register accessor: an alias for `Reg<PLL_PLL1_CFG1_SPEC>`"]
pub type PLL_PLL1_CFG1 = crate::Reg<pll_pll1_cfg1::PLL_PLL1_CFG1_SPEC>;
#[doc = "PLLx config1"]
pub mod pll_pll1_cfg1;
#[doc = "PLL_PLL1_CFG2 (rw) register accessor: an alias for `Reg<PLL_PLL1_CFG2_SPEC>`"]
pub type PLL_PLL1_CFG2 = crate::Reg<pll_pll1_cfg2::PLL_PLL1_CFG2_SPEC>;
#[doc = "PLLx config2"]
pub mod pll_pll1_cfg2;
#[doc = "PLL_PLL1_FREQ (rw) register accessor: an alias for `Reg<PLL_PLL1_FREQ_SPEC>`"]
pub type PLL_PLL1_FREQ = crate::Reg<pll_pll1_freq::PLL_PLL1_FREQ_SPEC>;
#[doc = "PLLx frac mode frequency adjust"]
pub mod pll_pll1_freq;
#[doc = "PLL_PLL1_LOCK (rw) register accessor: an alias for `Reg<PLL_PLL1_LOCK_SPEC>`"]
pub type PLL_PLL1_LOCK = crate::Reg<pll_pll1_lock::PLL_PLL1_LOCK_SPEC>;
#[doc = "PLLx lock control"]
pub mod pll_pll1_lock;
#[doc = "PLL_PLL1_STATUS (r) register accessor: an alias for `Reg<PLL_PLL1_STATUS_SPEC>`"]
pub type PLL_PLL1_STATUS = crate::Reg<pll_pll1_status::PLL_PLL1_STATUS_SPEC>;
#[doc = "PLLx status"]
pub mod pll_pll1_status;
#[doc = "PLL_PLL1_DIV0 (rw) register accessor: an alias for `Reg<PLL_PLL1_DIV0_SPEC>`"]
pub type PLL_PLL1_DIV0 = crate::Reg<pll_pll1_div0::PLL_PLL1_DIV0_SPEC>;
#[doc = "PLLx divider0 control"]
pub mod pll_pll1_div0;
#[doc = "PLL_PLL1_DIV1 (rw) register accessor: an alias for `Reg<PLL_PLL1_DIV1_SPEC>`"]
pub type PLL_PLL1_DIV1 = crate::Reg<pll_pll1_div1::PLL_PLL1_DIV1_SPEC>;
#[doc = "PLLx divider1 control"]
pub mod pll_pll1_div1;
#[doc = "PLL_PLL2_CFG0 (rw) register accessor: an alias for `Reg<PLL_PLL2_CFG0_SPEC>`"]
pub type PLL_PLL2_CFG0 = crate::Reg<pll_pll2_cfg0::PLL_PLL2_CFG0_SPEC>;
#[doc = "PLLx config0"]
pub mod pll_pll2_cfg0;
#[doc = "PLL_PLL2_CFG1 (rw) register accessor: an alias for `Reg<PLL_PLL2_CFG1_SPEC>`"]
pub type PLL_PLL2_CFG1 = crate::Reg<pll_pll2_cfg1::PLL_PLL2_CFG1_SPEC>;
#[doc = "PLLx config1"]
pub mod pll_pll2_cfg1;
#[doc = "PLL_PLL2_CFG2 (rw) register accessor: an alias for `Reg<PLL_PLL2_CFG2_SPEC>`"]
pub type PLL_PLL2_CFG2 = crate::Reg<pll_pll2_cfg2::PLL_PLL2_CFG2_SPEC>;
#[doc = "PLLx config2"]
pub mod pll_pll2_cfg2;
#[doc = "PLL_PLL2_FREQ (rw) register accessor: an alias for `Reg<PLL_PLL2_FREQ_SPEC>`"]
pub type PLL_PLL2_FREQ = crate::Reg<pll_pll2_freq::PLL_PLL2_FREQ_SPEC>;
#[doc = "PLLx frac mode frequency adjust"]
pub mod pll_pll2_freq;
#[doc = "PLL_PLL2_LOCK (rw) register accessor: an alias for `Reg<PLL_PLL2_LOCK_SPEC>`"]
pub type PLL_PLL2_LOCK = crate::Reg<pll_pll2_lock::PLL_PLL2_LOCK_SPEC>;
#[doc = "PLLx lock control"]
pub mod pll_pll2_lock;
#[doc = "PLL_PLL2_STATUS (r) register accessor: an alias for `Reg<PLL_PLL2_STATUS_SPEC>`"]
pub type PLL_PLL2_STATUS = crate::Reg<pll_pll2_status::PLL_PLL2_STATUS_SPEC>;
#[doc = "PLLx status"]
pub mod pll_pll2_status;
#[doc = "PLL_PLL2_DIV0 (rw) register accessor: an alias for `Reg<PLL_PLL2_DIV0_SPEC>`"]
pub type PLL_PLL2_DIV0 = crate::Reg<pll_pll2_div0::PLL_PLL2_DIV0_SPEC>;
#[doc = "PLLx divider0 control"]
pub mod pll_pll2_div0;
#[doc = "PLL_PLL2_DIV1 (rw) register accessor: an alias for `Reg<PLL_PLL2_DIV1_SPEC>`"]
pub type PLL_PLL2_DIV1 = crate::Reg<pll_pll2_div1::PLL_PLL2_DIV1_SPEC>;
#[doc = "PLLx divider1 control"]
pub mod pll_pll2_div1;
#[doc = "PLL_PLL3_CFG0 (rw) register accessor: an alias for `Reg<PLL_PLL3_CFG0_SPEC>`"]
pub type PLL_PLL3_CFG0 = crate::Reg<pll_pll3_cfg0::PLL_PLL3_CFG0_SPEC>;
#[doc = "PLLx config0"]
pub mod pll_pll3_cfg0;
#[doc = "PLL_PLL3_CFG1 (rw) register accessor: an alias for `Reg<PLL_PLL3_CFG1_SPEC>`"]
pub type PLL_PLL3_CFG1 = crate::Reg<pll_pll3_cfg1::PLL_PLL3_CFG1_SPEC>;
#[doc = "PLLx config1"]
pub mod pll_pll3_cfg1;
#[doc = "PLL_PLL3_CFG2 (rw) register accessor: an alias for `Reg<PLL_PLL3_CFG2_SPEC>`"]
pub type PLL_PLL3_CFG2 = crate::Reg<pll_pll3_cfg2::PLL_PLL3_CFG2_SPEC>;
#[doc = "PLLx config2"]
pub mod pll_pll3_cfg2;
#[doc = "PLL_PLL3_FREQ (rw) register accessor: an alias for `Reg<PLL_PLL3_FREQ_SPEC>`"]
pub type PLL_PLL3_FREQ = crate::Reg<pll_pll3_freq::PLL_PLL3_FREQ_SPEC>;
#[doc = "PLLx frac mode frequency adjust"]
pub mod pll_pll3_freq;
#[doc = "PLL_PLL3_LOCK (rw) register accessor: an alias for `Reg<PLL_PLL3_LOCK_SPEC>`"]
pub type PLL_PLL3_LOCK = crate::Reg<pll_pll3_lock::PLL_PLL3_LOCK_SPEC>;
#[doc = "PLLx lock control"]
pub mod pll_pll3_lock;
#[doc = "PLL_PLL3_STATUS (r) register accessor: an alias for `Reg<PLL_PLL3_STATUS_SPEC>`"]
pub type PLL_PLL3_STATUS = crate::Reg<pll_pll3_status::PLL_PLL3_STATUS_SPEC>;
#[doc = "PLLx status"]
pub mod pll_pll3_status;
#[doc = "PLL_PLL3_DIV0 (rw) register accessor: an alias for `Reg<PLL_PLL3_DIV0_SPEC>`"]
pub type PLL_PLL3_DIV0 = crate::Reg<pll_pll3_div0::PLL_PLL3_DIV0_SPEC>;
#[doc = "PLLx divider0 control"]
pub mod pll_pll3_div0;
#[doc = "PLL_PLL3_DIV1 (rw) register accessor: an alias for `Reg<PLL_PLL3_DIV1_SPEC>`"]
pub type PLL_PLL3_DIV1 = crate::Reg<pll_pll3_div1::PLL_PLL3_DIV1_SPEC>;
#[doc = "PLLx divider1 control"]
pub mod pll_pll3_div1;
#[doc = "PLL_PLL4_CFG0 (rw) register accessor: an alias for `Reg<PLL_PLL4_CFG0_SPEC>`"]
pub type PLL_PLL4_CFG0 = crate::Reg<pll_pll4_cfg0::PLL_PLL4_CFG0_SPEC>;
#[doc = "PLLx config0"]
pub mod pll_pll4_cfg0;
#[doc = "PLL_PLL4_CFG1 (rw) register accessor: an alias for `Reg<PLL_PLL4_CFG1_SPEC>`"]
pub type PLL_PLL4_CFG1 = crate::Reg<pll_pll4_cfg1::PLL_PLL4_CFG1_SPEC>;
#[doc = "PLLx config1"]
pub mod pll_pll4_cfg1;
#[doc = "PLL_PLL4_CFG2 (rw) register accessor: an alias for `Reg<PLL_PLL4_CFG2_SPEC>`"]
pub type PLL_PLL4_CFG2 = crate::Reg<pll_pll4_cfg2::PLL_PLL4_CFG2_SPEC>;
#[doc = "PLLx config2"]
pub mod pll_pll4_cfg2;
#[doc = "PLL_PLL4_FREQ (rw) register accessor: an alias for `Reg<PLL_PLL4_FREQ_SPEC>`"]
pub type PLL_PLL4_FREQ = crate::Reg<pll_pll4_freq::PLL_PLL4_FREQ_SPEC>;
#[doc = "PLLx frac mode frequency adjust"]
pub mod pll_pll4_freq;
#[doc = "PLL_PLL4_LOCK (rw) register accessor: an alias for `Reg<PLL_PLL4_LOCK_SPEC>`"]
pub type PLL_PLL4_LOCK = crate::Reg<pll_pll4_lock::PLL_PLL4_LOCK_SPEC>;
#[doc = "PLLx lock control"]
pub mod pll_pll4_lock;
#[doc = "PLL_PLL4_STATUS (r) register accessor: an alias for `Reg<PLL_PLL4_STATUS_SPEC>`"]
pub type PLL_PLL4_STATUS = crate::Reg<pll_pll4_status::PLL_PLL4_STATUS_SPEC>;
#[doc = "PLLx status"]
pub mod pll_pll4_status;
#[doc = "PLL_PLL4_DIV0 (rw) register accessor: an alias for `Reg<PLL_PLL4_DIV0_SPEC>`"]
pub type PLL_PLL4_DIV0 = crate::Reg<pll_pll4_div0::PLL_PLL4_DIV0_SPEC>;
#[doc = "PLLx divider0 control"]
pub mod pll_pll4_div0;
#[doc = "PLL_PLL4_DIV1 (rw) register accessor: an alias for `Reg<PLL_PLL4_DIV1_SPEC>`"]
pub type PLL_PLL4_DIV1 = crate::Reg<pll_pll4_div1::PLL_PLL4_DIV1_SPEC>;
#[doc = "PLLx divider1 control"]
pub mod pll_pll4_div1;
