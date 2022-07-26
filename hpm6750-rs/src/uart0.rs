#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x14 - Over Sample Control Register"]
    pub oscr: crate::Reg<oscr::OSCR_SPEC>,
    _reserved2: [u8; 0x08],
    _reserved_2_dll: [u8; 0x04],
    _reserved_3_dlm: [u8; 0x04],
    _reserved_4_fcr: [u8; 0x04],
    #[doc = "0x2c - Line Control Register"]
    pub lcr: crate::Reg<lcr::LCR_SPEC>,
    #[doc = "0x30 - Modem Control Register ("]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x34 - Line Status Register"]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0x38 - Modem Status Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x20 - Divisor Latch LSB (when DLAB = 1)"]
    #[inline(always)]
    pub fn dll(&self) -> &crate::Reg<dll::DLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<dll::DLL_SPEC>)
        }
    }
    #[doc = "0x20 - Transmitter Holding Register (when DLAB = 0)"]
    #[inline(always)]
    pub fn thr(&self) -> &crate::Reg<thr::THR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<thr::THR_SPEC>)
        }
    }
    #[doc = "0x20 - Receiver Buffer Register (when DLAB = 0)"]
    #[inline(always)]
    pub fn rbr(&self) -> &crate::Reg<rbr::RBR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(32usize)
                as *const crate::Reg<rbr::RBR_SPEC>)
        }
    }
    #[doc = "0x24 - Divisor Latch MSB (when DLAB = 1)"]
    #[inline(always)]
    pub fn dlm(&self) -> &crate::Reg<dlm::DLM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<dlm::DLM_SPEC>)
        }
    }
    #[doc = "0x24 - Interrupt Enable Register (when DLAB = 0)"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x28 - FIFO Control Register"]
    #[inline(always)]
    pub fn fcr(&self) -> &crate::Reg<fcr::FCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<fcr::FCR_SPEC>)
        }
    }
    #[doc = "0x28 - Interrupt Identification Register"]
    #[inline(always)]
    pub fn iir(&self) -> &crate::Reg<iir::IIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<iir::IIR_SPEC>)
        }
    }
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "OSCR register accessor: an alias for `Reg<OSCR_SPEC>`"]
pub type OSCR = crate::Reg<oscr::OSCR_SPEC>;
#[doc = "Over Sample Control Register"]
pub mod oscr;
#[doc = "RBR register accessor: an alias for `Reg<RBR_SPEC>`"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "Receiver Buffer Register (when DLAB = 0)"]
pub mod rbr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmitter Holding Register (when DLAB = 0)"]
pub mod thr;
#[doc = "DLL register accessor: an alias for `Reg<DLL_SPEC>`"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "Divisor Latch LSB (when DLAB = 1)"]
pub mod dll;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register (when DLAB = 0)"]
pub mod ier;
#[doc = "DLM register accessor: an alias for `Reg<DLM_SPEC>`"]
pub type DLM = crate::Reg<dlm::DLM_SPEC>;
#[doc = "Divisor Latch MSB (when DLAB = 1)"]
pub mod dlm;
#[doc = "IIR register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt Identification Register"]
pub mod iir;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "LCR register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control Register ("]
pub mod mcr;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status Register"]
pub mod msr;
