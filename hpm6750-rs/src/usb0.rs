#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - General Purpose Timer #0 Load Register"]
    pub gptimer0ld: GPTIMER0LD,
    #[doc = "0x84 - General Purpose Timer #0 Controller Register"]
    pub gptimer0ctrl: GPTIMER0CTRL,
    #[doc = "0x88 - General Purpose Timer #1 Load Register"]
    pub gptimer1ld: GPTIMER1LD,
    #[doc = "0x8c - General Purpose Timer #1 Controller Register"]
    pub gptimer1ctrl: GPTIMER1CTRL,
    #[doc = "0x90 - System Bus Config Register"]
    pub sbuscfg: SBUSCFG,
    _reserved5: [u8; 0xac],
    #[doc = "0x140 - USB Command Register"]
    pub usbcmd: USBCMD,
    #[doc = "0x144 - USB Status Register"]
    pub usbsts: USBSTS,
    #[doc = "0x148 - Interrupt Enable Register"]
    pub usbintr: USBINTR,
    #[doc = "0x14c - USB Frame Index Register"]
    pub frindex: FRINDEX,
    _reserved9: [u8; 0x04],
    _reserved_9_deviceaddr: [u8; 0x04],
    _reserved_10_asynclistaddr: [u8; 0x04],
    _reserved11: [u8; 0x04],
    #[doc = "0x160 - Programmable Burst Size Register"]
    pub burstsize: BURSTSIZE,
    #[doc = "0x164 - TX FIFO Fill Tuning Register"]
    pub txfilltuning: TXFILLTUNING,
    _reserved13: [u8; 0x10],
    #[doc = "0x178 - Endpoint NAK Register"]
    pub endptnak: ENDPTNAK,
    #[doc = "0x17c - Endpoint NAK Enable Register"]
    pub endptnaken: ENDPTNAKEN,
    _reserved15: [u8; 0x04],
    #[doc = "0x184 - Port Status & Control"]
    pub portsc1: PORTSC1,
    _reserved16: [u8; 0x1c],
    #[doc = "0x1a4 - On-The-Go Status & control Register"]
    pub otgsc: OTGSC,
    #[doc = "0x1a8 - USB Device Mode Register"]
    pub usbmode: USBMODE,
    #[doc = "0x1ac - Endpoint Setup Status Register"]
    pub endptsetupstat: ENDPTSETUPSTAT,
    #[doc = "0x1b0 - Endpoint Prime Register"]
    pub endptprime: ENDPTPRIME,
    #[doc = "0x1b4 - Endpoint Flush Register"]
    pub endptflush: ENDPTFLUSH,
    #[doc = "0x1b8 - Endpoint Status Register"]
    pub endptstat: ENDPTSTAT,
    #[doc = "0x1bc - Endpoint Complete Register"]
    pub endptcomplete: ENDPTCOMPLETE,
    #[doc = "0x1c0 - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl0: ENDPTCTRL_ENDPTCTRL0,
    #[doc = "0x1c4 - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl1: ENDPTCTRL_ENDPTCTRL1,
    #[doc = "0x1c8 - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl2: ENDPTCTRL_ENDPTCTRL2,
    #[doc = "0x1cc - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl3: ENDPTCTRL_ENDPTCTRL3,
    #[doc = "0x1d0 - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl4: ENDPTCTRL_ENDPTCTRL4,
    #[doc = "0x1d4 - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl5: ENDPTCTRL_ENDPTCTRL5,
    #[doc = "0x1d8 - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl6: ENDPTCTRL_ENDPTCTRL6,
    #[doc = "0x1dc - Endpoint Control0 Register... Endpoint Control7 Register"]
    pub endptctrl_endptctrl7: ENDPTCTRL_ENDPTCTRL7,
    _reserved31: [u8; 0x20],
    #[doc = "0x200 - No description avaiable"]
    pub otg_ctrl0: OTG_CTRL0,
    _reserved32: [u8; 0x0c],
    #[doc = "0x210 - No description avaiable"]
    pub phy_ctrl0: PHY_CTRL0,
    #[doc = "0x214 - No description avaiable"]
    pub phy_ctrl1: PHY_CTRL1,
    _reserved34: [u8; 0x08],
    #[doc = "0x220 - No description avaiable"]
    pub top_status: TOP_STATUS,
    #[doc = "0x224 - No description avaiable"]
    pub phy_status: PHY_STATUS,
}
impl RegisterBlock {
    #[doc = "0x154 - Frame List Base Address Register"]
    #[inline(always)]
    pub const fn periodiclistbase(&self) -> &PERIODICLISTBASE {
        unsafe { &*(self as *const Self).cast::<u8>().add(340usize).cast() }
    }
    #[doc = "0x154 - Device Address Register"]
    #[inline(always)]
    pub const fn deviceaddr(&self) -> &DEVICEADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(340usize).cast() }
    }
    #[doc = "0x158 - Endpoint List Address Register"]
    #[inline(always)]
    pub const fn endptlistaddr(&self) -> &ENDPTLISTADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
    #[doc = "0x158 - Next Asynch. Address Register"]
    #[inline(always)]
    pub const fn asynclistaddr(&self) -> &ASYNCLISTADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(344usize).cast() }
    }
}
#[doc = "GPTIMER0LD (rw) register accessor: an alias for `Reg<GPTIMER0LD_SPEC>`"]
pub type GPTIMER0LD = crate::Reg<gptimer0ld::GPTIMER0LD_SPEC>;
#[doc = "General Purpose Timer #0 Load Register"]
pub mod gptimer0ld;
#[doc = "GPTIMER0CTRL (rw) register accessor: an alias for `Reg<GPTIMER0CTRL_SPEC>`"]
pub type GPTIMER0CTRL = crate::Reg<gptimer0ctrl::GPTIMER0CTRL_SPEC>;
#[doc = "General Purpose Timer #0 Controller Register"]
pub mod gptimer0ctrl;
#[doc = "GPTIMER1LD (rw) register accessor: an alias for `Reg<GPTIMER1LD_SPEC>`"]
pub type GPTIMER1LD = crate::Reg<gptimer1ld::GPTIMER1LD_SPEC>;
#[doc = "General Purpose Timer #1 Load Register"]
pub mod gptimer1ld;
#[doc = "GPTIMER1CTRL (rw) register accessor: an alias for `Reg<GPTIMER1CTRL_SPEC>`"]
pub type GPTIMER1CTRL = crate::Reg<gptimer1ctrl::GPTIMER1CTRL_SPEC>;
#[doc = "General Purpose Timer #1 Controller Register"]
pub mod gptimer1ctrl;
#[doc = "SBUSCFG (rw) register accessor: an alias for `Reg<SBUSCFG_SPEC>`"]
pub type SBUSCFG = crate::Reg<sbuscfg::SBUSCFG_SPEC>;
#[doc = "System Bus Config Register"]
pub mod sbuscfg;
#[doc = "USBCMD (rw) register accessor: an alias for `Reg<USBCMD_SPEC>`"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: an alias for `Reg<USBSTS_SPEC>`"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: an alias for `Reg<USBINTR_SPEC>`"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod usbintr;
#[doc = "FRINDEX (rw) register accessor: an alias for `Reg<FRINDEX_SPEC>`"]
pub type FRINDEX = crate::Reg<frindex::FRINDEX_SPEC>;
#[doc = "USB Frame Index Register"]
pub mod frindex;
#[doc = "DEVICEADDR (rw) register accessor: an alias for `Reg<DEVICEADDR_SPEC>`"]
pub type DEVICEADDR = crate::Reg<deviceaddr::DEVICEADDR_SPEC>;
#[doc = "Device Address Register"]
pub mod deviceaddr;
#[doc = "PERIODICLISTBASE (rw) register accessor: an alias for `Reg<PERIODICLISTBASE_SPEC>`"]
pub type PERIODICLISTBASE = crate::Reg<periodiclistbase::PERIODICLISTBASE_SPEC>;
#[doc = "Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "ASYNCLISTADDR (rw) register accessor: an alias for `Reg<ASYNCLISTADDR_SPEC>`"]
pub type ASYNCLISTADDR = crate::Reg<asynclistaddr::ASYNCLISTADDR_SPEC>;
#[doc = "Next Asynch. Address Register"]
pub mod asynclistaddr;
#[doc = "ENDPTLISTADDR (rw) register accessor: an alias for `Reg<ENDPTLISTADDR_SPEC>`"]
pub type ENDPTLISTADDR = crate::Reg<endptlistaddr::ENDPTLISTADDR_SPEC>;
#[doc = "Endpoint List Address Register"]
pub mod endptlistaddr;
#[doc = "BURSTSIZE (rw) register accessor: an alias for `Reg<BURSTSIZE_SPEC>`"]
pub type BURSTSIZE = crate::Reg<burstsize::BURSTSIZE_SPEC>;
#[doc = "Programmable Burst Size Register"]
pub mod burstsize;
#[doc = "TXFILLTUNING (rw) register accessor: an alias for `Reg<TXFILLTUNING_SPEC>`"]
pub type TXFILLTUNING = crate::Reg<txfilltuning::TXFILLTUNING_SPEC>;
#[doc = "TX FIFO Fill Tuning Register"]
pub mod txfilltuning;
#[doc = "ENDPTNAK (rw) register accessor: an alias for `Reg<ENDPTNAK_SPEC>`"]
pub type ENDPTNAK = crate::Reg<endptnak::ENDPTNAK_SPEC>;
#[doc = "Endpoint NAK Register"]
pub mod endptnak;
#[doc = "ENDPTNAKEN (rw) register accessor: an alias for `Reg<ENDPTNAKEN_SPEC>`"]
pub type ENDPTNAKEN = crate::Reg<endptnaken::ENDPTNAKEN_SPEC>;
#[doc = "Endpoint NAK Enable Register"]
pub mod endptnaken;
#[doc = "PORTSC1 (rw) register accessor: an alias for `Reg<PORTSC1_SPEC>`"]
pub type PORTSC1 = crate::Reg<portsc1::PORTSC1_SPEC>;
#[doc = "Port Status & Control"]
pub mod portsc1;
#[doc = "OTGSC (rw) register accessor: an alias for `Reg<OTGSC_SPEC>`"]
pub type OTGSC = crate::Reg<otgsc::OTGSC_SPEC>;
#[doc = "On-The-Go Status & control Register"]
pub mod otgsc;
#[doc = "USBMODE (rw) register accessor: an alias for `Reg<USBMODE_SPEC>`"]
pub type USBMODE = crate::Reg<usbmode::USBMODE_SPEC>;
#[doc = "USB Device Mode Register"]
pub mod usbmode;
#[doc = "ENDPTSETUPSTAT (rw) register accessor: an alias for `Reg<ENDPTSETUPSTAT_SPEC>`"]
pub type ENDPTSETUPSTAT = crate::Reg<endptsetupstat::ENDPTSETUPSTAT_SPEC>;
#[doc = "Endpoint Setup Status Register"]
pub mod endptsetupstat;
#[doc = "ENDPTPRIME (rw) register accessor: an alias for `Reg<ENDPTPRIME_SPEC>`"]
pub type ENDPTPRIME = crate::Reg<endptprime::ENDPTPRIME_SPEC>;
#[doc = "Endpoint Prime Register"]
pub mod endptprime;
#[doc = "ENDPTFLUSH (rw) register accessor: an alias for `Reg<ENDPTFLUSH_SPEC>`"]
pub type ENDPTFLUSH = crate::Reg<endptflush::ENDPTFLUSH_SPEC>;
#[doc = "Endpoint Flush Register"]
pub mod endptflush;
#[doc = "ENDPTSTAT (r) register accessor: an alias for `Reg<ENDPTSTAT_SPEC>`"]
pub type ENDPTSTAT = crate::Reg<endptstat::ENDPTSTAT_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod endptstat;
#[doc = "ENDPTCOMPLETE (rw) register accessor: an alias for `Reg<ENDPTCOMPLETE_SPEC>`"]
pub type ENDPTCOMPLETE = crate::Reg<endptcomplete::ENDPTCOMPLETE_SPEC>;
#[doc = "Endpoint Complete Register"]
pub mod endptcomplete;
#[doc = "ENDPTCTRL_ENDPTCTRL0 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL0_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL0 = crate::Reg<endptctrl_endptctrl0::ENDPTCTRL_ENDPTCTRL0_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl0;
#[doc = "ENDPTCTRL_ENDPTCTRL1 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL1_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL1 = crate::Reg<endptctrl_endptctrl1::ENDPTCTRL_ENDPTCTRL1_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl1;
#[doc = "ENDPTCTRL_ENDPTCTRL2 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL2_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL2 = crate::Reg<endptctrl_endptctrl2::ENDPTCTRL_ENDPTCTRL2_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl2;
#[doc = "ENDPTCTRL_ENDPTCTRL3 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL3_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL3 = crate::Reg<endptctrl_endptctrl3::ENDPTCTRL_ENDPTCTRL3_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl3;
#[doc = "ENDPTCTRL_ENDPTCTRL4 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL4_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL4 = crate::Reg<endptctrl_endptctrl4::ENDPTCTRL_ENDPTCTRL4_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl4;
#[doc = "ENDPTCTRL_ENDPTCTRL5 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL5_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL5 = crate::Reg<endptctrl_endptctrl5::ENDPTCTRL_ENDPTCTRL5_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl5;
#[doc = "ENDPTCTRL_ENDPTCTRL6 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL6_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL6 = crate::Reg<endptctrl_endptctrl6::ENDPTCTRL_ENDPTCTRL6_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl6;
#[doc = "ENDPTCTRL_ENDPTCTRL7 (rw) register accessor: an alias for `Reg<ENDPTCTRL_ENDPTCTRL7_SPEC>`"]
pub type ENDPTCTRL_ENDPTCTRL7 = crate::Reg<endptctrl_endptctrl7::ENDPTCTRL_ENDPTCTRL7_SPEC>;
#[doc = "Endpoint Control0 Register... Endpoint Control7 Register"]
pub mod endptctrl_endptctrl7;
#[doc = "OTG_CTRL0 (rw) register accessor: an alias for `Reg<OTG_CTRL0_SPEC>`"]
pub type OTG_CTRL0 = crate::Reg<otg_ctrl0::OTG_CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod otg_ctrl0;
#[doc = "PHY_CTRL0 (rw) register accessor: an alias for `Reg<PHY_CTRL0_SPEC>`"]
pub type PHY_CTRL0 = crate::Reg<phy_ctrl0::PHY_CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod phy_ctrl0;
#[doc = "PHY_CTRL1 (rw) register accessor: an alias for `Reg<PHY_CTRL1_SPEC>`"]
pub type PHY_CTRL1 = crate::Reg<phy_ctrl1::PHY_CTRL1_SPEC>;
#[doc = "No description avaiable"]
pub mod phy_ctrl1;
#[doc = "TOP_STATUS (rw) register accessor: an alias for `Reg<TOP_STATUS_SPEC>`"]
pub type TOP_STATUS = crate::Reg<top_status::TOP_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod top_status;
#[doc = "PHY_STATUS (rw) register accessor: an alias for `Reg<PHY_STATUS_SPEC>`"]
pub type PHY_STATUS = crate::Reg<phy_status::PHY_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod phy_status;
