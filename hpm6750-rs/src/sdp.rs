#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDP control register"]
    pub sdpcr: crate::Reg<sdpcr::SDPCR_SPEC>,
    #[doc = "0x04 - Mod control register."]
    pub modctrl: crate::Reg<modctrl::MODCTRL_SPEC>,
    #[doc = "0x08 - packet counter registers."]
    pub pktcnt: crate::Reg<pktcnt::PKTCNT_SPEC>,
    #[doc = "0x0c - Status Registers"]
    pub sta: crate::Reg<sta::STA_SPEC>,
    #[doc = "0x10 - Key Address"]
    pub keyaddr: crate::Reg<keyaddr::KEYADDR_SPEC>,
    #[doc = "0x14 - Key Data"]
    pub keydat: crate::Reg<keydat::KEYDAT_SPEC>,
    #[doc = "0x18 - Cipher Initializtion Vector 0"]
    pub ciphiv_ciphiv0: crate::Reg<ciphiv_ciphiv0::CIPHIV_CIPHIV0_SPEC>,
    #[doc = "0x1c - Cipher Initializtion Vector 1"]
    pub ciphiv_ciphiv1: crate::Reg<ciphiv_ciphiv1::CIPHIV_CIPHIV1_SPEC>,
    #[doc = "0x20 - Cipher Initializtion Vector 2"]
    pub ciphiv_ciphiv2: crate::Reg<ciphiv_ciphiv2::CIPHIV_CIPHIV2_SPEC>,
    #[doc = "0x24 - Cipher Initializtion Vector 3"]
    pub ciphiv_ciphiv3: crate::Reg<ciphiv_ciphiv3::CIPHIV_CIPHIV3_SPEC>,
    #[doc = "0x28 - Hash Data Word 0"]
    pub haswrd_haswrd0: crate::Reg<haswrd_haswrd0::HASWRD_HASWRD0_SPEC>,
    #[doc = "0x2c - Hash Data Word 1"]
    pub haswrd_haswrd1: crate::Reg<haswrd_haswrd1::HASWRD_HASWRD1_SPEC>,
    #[doc = "0x30 - Hash Data Word 2"]
    pub haswrd_haswrd2: crate::Reg<haswrd_haswrd2::HASWRD_HASWRD2_SPEC>,
    #[doc = "0x34 - Hash Data Word 3"]
    pub haswrd_haswrd3: crate::Reg<haswrd_haswrd3::HASWRD_HASWRD3_SPEC>,
    #[doc = "0x38 - Hash Data Word 4"]
    pub haswrd_haswrd4: crate::Reg<haswrd_haswrd4::HASWRD_HASWRD4_SPEC>,
    #[doc = "0x3c - Hash Data Word 5"]
    pub haswrd_haswrd5: crate::Reg<haswrd_haswrd5::HASWRD_HASWRD5_SPEC>,
    #[doc = "0x40 - Hash Data Word 6"]
    pub haswrd_haswrd6: crate::Reg<haswrd_haswrd6::HASWRD_HASWRD6_SPEC>,
    #[doc = "0x44 - Hash Data Word 7"]
    pub haswrd_haswrd7: crate::Reg<haswrd_haswrd7::HASWRD_HASWRD7_SPEC>,
    #[doc = "0x48 - Command Pointer"]
    pub cmdptr: crate::Reg<cmdptr::CMDPTR_SPEC>,
    #[doc = "0x4c - Next Packet Address Pointer"]
    pub npktptr: crate::Reg<npktptr::NPKTPTR_SPEC>,
    #[doc = "0x50 - Packet Control Registers"]
    pub pktctl: crate::Reg<pktctl::PKTCTL_SPEC>,
    #[doc = "0x54 - Packet Memory Source Address"]
    pub pktsrc: crate::Reg<pktsrc::PKTSRC_SPEC>,
    #[doc = "0x58 - Packet Memory Destination Address"]
    pub pktdst: crate::Reg<pktdst::PKTDST_SPEC>,
    #[doc = "0x5c - Packet buffer size."]
    pub pktbuf: crate::Reg<pktbuf::PKTBUF_SPEC>,
}
#[doc = "SDPCR register accessor: an alias for `Reg<SDPCR_SPEC>`"]
pub type SDPCR = crate::Reg<sdpcr::SDPCR_SPEC>;
#[doc = "SDP control register"]
pub mod sdpcr;
#[doc = "MODCTRL register accessor: an alias for `Reg<MODCTRL_SPEC>`"]
pub type MODCTRL = crate::Reg<modctrl::MODCTRL_SPEC>;
#[doc = "Mod control register."]
pub mod modctrl;
#[doc = "PKTCNT register accessor: an alias for `Reg<PKTCNT_SPEC>`"]
pub type PKTCNT = crate::Reg<pktcnt::PKTCNT_SPEC>;
#[doc = "packet counter registers."]
pub mod pktcnt;
#[doc = "STA register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Registers"]
pub mod sta;
#[doc = "KEYADDR register accessor: an alias for `Reg<KEYADDR_SPEC>`"]
pub type KEYADDR = crate::Reg<keyaddr::KEYADDR_SPEC>;
#[doc = "Key Address"]
pub mod keyaddr;
#[doc = "KEYDAT register accessor: an alias for `Reg<KEYDAT_SPEC>`"]
pub type KEYDAT = crate::Reg<keydat::KEYDAT_SPEC>;
#[doc = "Key Data"]
pub mod keydat;
#[doc = "CIPHIV_CIPHIV0 register accessor: an alias for `Reg<CIPHIV_CIPHIV0_SPEC>`"]
pub type CIPHIV_CIPHIV0 = crate::Reg<ciphiv_ciphiv0::CIPHIV_CIPHIV0_SPEC>;
#[doc = "Cipher Initializtion Vector 0"]
pub mod ciphiv_ciphiv0;
#[doc = "CIPHIV_CIPHIV1 register accessor: an alias for `Reg<CIPHIV_CIPHIV1_SPEC>`"]
pub type CIPHIV_CIPHIV1 = crate::Reg<ciphiv_ciphiv1::CIPHIV_CIPHIV1_SPEC>;
#[doc = "Cipher Initializtion Vector 1"]
pub mod ciphiv_ciphiv1;
#[doc = "CIPHIV_CIPHIV2 register accessor: an alias for `Reg<CIPHIV_CIPHIV2_SPEC>`"]
pub type CIPHIV_CIPHIV2 = crate::Reg<ciphiv_ciphiv2::CIPHIV_CIPHIV2_SPEC>;
#[doc = "Cipher Initializtion Vector 2"]
pub mod ciphiv_ciphiv2;
#[doc = "CIPHIV_CIPHIV3 register accessor: an alias for `Reg<CIPHIV_CIPHIV3_SPEC>`"]
pub type CIPHIV_CIPHIV3 = crate::Reg<ciphiv_ciphiv3::CIPHIV_CIPHIV3_SPEC>;
#[doc = "Cipher Initializtion Vector 3"]
pub mod ciphiv_ciphiv3;
#[doc = "HASWRD_HASWRD0 register accessor: an alias for `Reg<HASWRD_HASWRD0_SPEC>`"]
pub type HASWRD_HASWRD0 = crate::Reg<haswrd_haswrd0::HASWRD_HASWRD0_SPEC>;
#[doc = "Hash Data Word 0"]
pub mod haswrd_haswrd0;
#[doc = "HASWRD_HASWRD1 register accessor: an alias for `Reg<HASWRD_HASWRD1_SPEC>`"]
pub type HASWRD_HASWRD1 = crate::Reg<haswrd_haswrd1::HASWRD_HASWRD1_SPEC>;
#[doc = "Hash Data Word 1"]
pub mod haswrd_haswrd1;
#[doc = "HASWRD_HASWRD2 register accessor: an alias for `Reg<HASWRD_HASWRD2_SPEC>`"]
pub type HASWRD_HASWRD2 = crate::Reg<haswrd_haswrd2::HASWRD_HASWRD2_SPEC>;
#[doc = "Hash Data Word 2"]
pub mod haswrd_haswrd2;
#[doc = "HASWRD_HASWRD3 register accessor: an alias for `Reg<HASWRD_HASWRD3_SPEC>`"]
pub type HASWRD_HASWRD3 = crate::Reg<haswrd_haswrd3::HASWRD_HASWRD3_SPEC>;
#[doc = "Hash Data Word 3"]
pub mod haswrd_haswrd3;
#[doc = "HASWRD_HASWRD4 register accessor: an alias for `Reg<HASWRD_HASWRD4_SPEC>`"]
pub type HASWRD_HASWRD4 = crate::Reg<haswrd_haswrd4::HASWRD_HASWRD4_SPEC>;
#[doc = "Hash Data Word 4"]
pub mod haswrd_haswrd4;
#[doc = "HASWRD_HASWRD5 register accessor: an alias for `Reg<HASWRD_HASWRD5_SPEC>`"]
pub type HASWRD_HASWRD5 = crate::Reg<haswrd_haswrd5::HASWRD_HASWRD5_SPEC>;
#[doc = "Hash Data Word 5"]
pub mod haswrd_haswrd5;
#[doc = "HASWRD_HASWRD6 register accessor: an alias for `Reg<HASWRD_HASWRD6_SPEC>`"]
pub type HASWRD_HASWRD6 = crate::Reg<haswrd_haswrd6::HASWRD_HASWRD6_SPEC>;
#[doc = "Hash Data Word 6"]
pub mod haswrd_haswrd6;
#[doc = "HASWRD_HASWRD7 register accessor: an alias for `Reg<HASWRD_HASWRD7_SPEC>`"]
pub type HASWRD_HASWRD7 = crate::Reg<haswrd_haswrd7::HASWRD_HASWRD7_SPEC>;
#[doc = "Hash Data Word 7"]
pub mod haswrd_haswrd7;
#[doc = "CMDPTR register accessor: an alias for `Reg<CMDPTR_SPEC>`"]
pub type CMDPTR = crate::Reg<cmdptr::CMDPTR_SPEC>;
#[doc = "Command Pointer"]
pub mod cmdptr;
#[doc = "NPKTPTR register accessor: an alias for `Reg<NPKTPTR_SPEC>`"]
pub type NPKTPTR = crate::Reg<npktptr::NPKTPTR_SPEC>;
#[doc = "Next Packet Address Pointer"]
pub mod npktptr;
#[doc = "PKTCTL register accessor: an alias for `Reg<PKTCTL_SPEC>`"]
pub type PKTCTL = crate::Reg<pktctl::PKTCTL_SPEC>;
#[doc = "Packet Control Registers"]
pub mod pktctl;
#[doc = "PKTSRC register accessor: an alias for `Reg<PKTSRC_SPEC>`"]
pub type PKTSRC = crate::Reg<pktsrc::PKTSRC_SPEC>;
#[doc = "Packet Memory Source Address"]
pub mod pktsrc;
#[doc = "PKTDST register accessor: an alias for `Reg<PKTDST_SPEC>`"]
pub type PKTDST = crate::Reg<pktdst::PKTDST_SPEC>;
#[doc = "Packet Memory Destination Address"]
pub mod pktdst;
#[doc = "PKTBUF register accessor: an alias for `Reg<PKTBUF_SPEC>`"]
pub type PKTBUF = crate::Reg<pktbuf::PKTBUF_SPEC>;
#[doc = "Packet buffer size."]
pub mod pktbuf;
