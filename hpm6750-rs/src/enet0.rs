#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC Configuration Register"]
    pub maccfg: MACCFG,
    #[doc = "0x04 - MAC Frame Filter"]
    pub macff: MACFF,
    #[doc = "0x08 - Hash Table High Register"]
    pub hash_h: HASH_H,
    #[doc = "0x0c - Hash Table Low Register"]
    pub hash_l: HASH_L,
    #[doc = "0x10 - GMII Address Register"]
    pub gmii_addr: GMII_ADDR,
    #[doc = "0x14 - GMII Data Register"]
    pub gmii_data: GMII_DATA,
    #[doc = "0x18 - Flow Control Register"]
    pub flowctrl: FLOWCTRL,
    #[doc = "0x1c - VLAN Tag Register"]
    pub vlan_tag: VLAN_TAG,
    #[doc = "0x20 - Version Register"]
    pub version: VERSION,
    #[doc = "0x24 - Debug Register"]
    pub debugging: DEBUGGING,
    #[doc = "0x28 - Remote Wake-Up Frame Filter Register"]
    pub rwkfrmfilt: RWKFRMFILT,
    #[doc = "0x2c - PMT Control and Status Register"]
    pub pmt_csr: PMT_CSR,
    #[doc = "0x30 - LPI Control and Status Regsiter"]
    pub lpi_csr: LPI_CSR,
    #[doc = "0x34 - LPI Timers Control Register"]
    pub lpi_tcr: LPI_TCR,
    #[doc = "0x38 - Interrupt Status Register"]
    pub intr_status: INTR_STATUS,
    #[doc = "0x3c - Interrupt Mask Register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x40 - MAC Address 0 High Register"]
    pub mac_addr_0_high: MAC_ADDR_0_HIGH,
    #[doc = "0x44 - MAC Address 0 Low Register"]
    pub mac_addr_0_low: MAC_ADDR_0_LOW,
    #[doc = "0x48 - MAC Address High Register"]
    pub mac_addr_1_high: MAC_ADDR_1_HIGH,
    #[doc = "0x4c - MAC Address Low Register"]
    pub mac_addr_1_low: MAC_ADDR_1_LOW,
    #[doc = "0x50 - MAC Address2 High Register"]
    pub mac_addr_2_high: MAC_ADDR_2_HIGH,
    #[doc = "0x54 - MAC Address2 Low Register"]
    pub mac_addr_2_low: MAC_ADDR_2_LOW,
    #[doc = "0x58 - MAC Address3 High Register"]
    pub mac_addr_3_high: MAC_ADDR_3_HIGH,
    #[doc = "0x5c - MAC Address3 Low Register"]
    pub mac_addr_3_low: MAC_ADDR_3_LOW,
    #[doc = "0x60 - MAC Address4 High Register"]
    pub mac_addr_4_high: MAC_ADDR_4_HIGH,
    #[doc = "0x64 - MAC Address4 Low Register"]
    pub mac_addr_4_low: MAC_ADDR_4_LOW,
    _reserved26: [u8; 0x70],
    #[doc = "0xd8 - SGMII/RGMII/SMII Control and Status Register"]
    pub xmii_csr: XMII_CSR,
    #[doc = "0xdc - Watchdog Timeout Register"]
    pub wdog_wto: WDOG_WTO,
    #[doc = "0xe0 - General Purpose IO Register"]
    pub gpio: GPIO,
    _reserved29: [u8; 0x1c],
    #[doc = "0x100 - MMC Control establishes the operating mode of MMC."]
    pub mmc_cntrl: MMC_CNTRL,
    #[doc = "0x104 - MMC Receive Interrupt maintains the interrupt generated from all of the receive statistic counters."]
    pub mmc_intr_rx: MMC_INTR_RX,
    #[doc = "0x108 - MMC Transmit Interrupt maintains the interrupt generated from all of the transmit statistic counters"]
    pub mmc_intr_tx: MMC_INTR_TX,
    #[doc = "0x10c - MMC Receive Interrupt mask maintains the mask for the interrupt generated from all of the receive statistic counters"]
    pub mmc_intr_mask_rx: MMC_INTR_MASK_RX,
    #[doc = "0x110 - MMC Transmit Interrupt Mask"]
    pub mmc_intr_mask_tx: MMC_INTR_MASK_TX,
    #[doc = "0x114 - Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
    pub txoctetcount_gb: TXOCTETCOUNT_GB,
    #[doc = "0x118 - Number of good and bad frames transmitted, exclusive of retried frames."]
    pub txframecount_gb: TXFRAMECOUNT_GB,
    #[doc = "0x11c - Number of good broadcast frames transmitted"]
    pub txbroadcastframes_g: TXBROADCASTFRAMES_G,
    #[doc = "0x120 - Number of good multicast frames transmitted"]
    pub txmlticastframes_g: TXMLTICASTFRAMES_G,
    #[doc = "0x124 - Number of good and bad frames transmitted with length 64 bytes, exclusive of preamble and retried frames."]
    pub tx64octets_gb: TX64OCTETS_GB,
    #[doc = "0x128 - Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
    pub tx65to127octets_gb: TX65TO127OCTETS_GB,
    #[doc = "0x12c - Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
    pub tx128to255octets_gb: TX128TO255OCTETS_GB,
    #[doc = "0x130 - Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
    pub tx256to511octets_gb: TX256TO511OCTETS_GB,
    #[doc = "0x134 - Number of good and bad frames transmitted with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
    pub tx512to1023octets_gb: TX512TO1023OCTETS_GB,
    #[doc = "0x138 - Number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
    pub tx1024tomaxoctets_gb: TX1024TOMAXOCTETS_GB,
    #[doc = "0x13c - Number of good and bad unicast frames transmitted."]
    pub txunicastframes_gb: TXUNICASTFRAMES_GB,
    #[doc = "0x140 - Number of good and bad multicast frames transmitted."]
    pub txmulticastframes_gb: TXMULTICASTFRAMES_GB,
    #[doc = "0x144 - Number of good and bad broadcast frames transmitted."]
    pub txbroadcastframes_gb: TXBROADCASTFRAMES_GB,
    #[doc = "0x148 - Number of frames aborted because of frame underflow error."]
    pub txunderflowerror: TXUNDERFLOWERROR,
    #[doc = "0x14c - Number of successfully transmitted frames after a single collision in the half-duplex mode."]
    pub txsinglecol_g: TXSINGLECOL_G,
    #[doc = "0x150 - Number of successfully transmitted frames after multiple collisions in the half-duplex mode."]
    pub txmulticol_g: TXMULTICOL_G,
    #[doc = "0x154 - Number of successfully transmitted frames after a deferral in the half-duplex mode."]
    pub txdeferred: TXDEFERRED,
    #[doc = "0x158 - Number of frames aborted because of late collision error"]
    pub txlatecol: TXLATECOL,
    #[doc = "0x15c - Number of frames aborted because of excessive (16) collision errors"]
    pub txexesscol: TXEXESSCOL,
    #[doc = "0x160 - Number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
    pub txcarriererror: TXCARRIERERROR,
    #[doc = "0x164 - Number of bytes transmitted, exclusive of preamble, only in good frames."]
    pub txoctetcount_g: TXOCTETCOUNT_G,
    #[doc = "0x168 - Number of good frames transmitted"]
    pub txframecount_g: TXFRAMECOUNT_G,
    #[doc = "0x16c - Number of frames aborted because of excessive deferral error (deferred for more than two max-sized frame times)."]
    pub txexcessdef: TXEXCESSDEF,
    #[doc = "0x170 - Number of good Pause frames transmitted"]
    pub txpauseframes: TXPAUSEFRAMES,
    #[doc = "0x174 - Number of good VLAN frames transmitted, exclusive of retried frames."]
    pub txvlanframes_g: TXVLANFRAMES_G,
    #[doc = "0x178 - Number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled in Bit 27 of Register 0 (MAC Configuration Register))."]
    pub txoversize_g: TXOVERSIZE_G,
    _reserved60: [u8; 0x04],
    #[doc = "0x180 - Number of good and bad frames received"]
    pub rxframecount_gb: RXFRAMECOUNT_GB,
    #[doc = "0x184 - Number of bytes received, exclusive of preamble, only in good frames."]
    pub rxoctetcount_g: RXOCTETCOUNT_G,
    #[doc = "0x188 - Number of bytes received, exclusive of preamble, in good and bad frames."]
    pub rxoctetcount_gb: RXOCTETCOUNT_GB,
    #[doc = "0x18c - Number of good broadcast frames received"]
    pub rxbroadcastframes_g: RXBROADCASTFRAMES_G,
    #[doc = "0x190 - Number of good multicast frames received"]
    pub rxmulticastframes_g: RXMULTICASTFRAMES_G,
    #[doc = "0x194 - Number of frames received with CRC error"]
    pub rxcrcerror: RXCRCERROR,
    #[doc = "0x198 - Number of frames received with alignment (dribble) error. Valid only in 10/100 mode"]
    pub rxalignmenterror: RXALIGNMENTERROR,
    #[doc = "0x19c - Number of frames received with runt (<64 bytes and CRC error) error."]
    pub rxrunterror: RXRUNTERROR,
    #[doc = "0x1a0 - Number of giant frames received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error. If Jumbo Frame mode is enabled, then frames of length greater than 9,018 bytes (9,022 for VLAN tagged) are considered as giant frames."]
    pub rxjabbererror: RXJABBERERROR,
    #[doc = "0x1a4 - Number of frames received with length less than 64 bytes, without any errors."]
    pub rxundersize_g: RXUNDERSIZE_G,
    #[doc = "0x1a8 - Number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled in Bit 27 of Register 0 (MAC Configuration Register))"]
    pub rxoversize_g: RXOVERSIZE_G,
    #[doc = "0x1ac - Number of good and bad frames received with length 64 bytes, exclusive of preamble."]
    pub rx64octets_gb: RX64OCTETS_GB,
    #[doc = "0x1b0 - No description avaiable"]
    pub rx65to127octets_gb: RX65TO127OCTETS_GB,
    #[doc = "0x1b4 - No description avaiable"]
    pub rx128to255octets_gb: RX128TO255OCTETS_GB,
    #[doc = "0x1b8 - Number of good and bad frames received with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
    pub rx256to511octets_gb: RX256TO511OCTETS_GB,
    #[doc = "0x1bc - Number of good and bad frames received with length between 512 and 1023 (inclusive) bytes, exclusive of preamble."]
    pub rx512to1023octets_gb: RX512TO1023OCTETS_GB,
    #[doc = "0x1c0 - Number of good and bad frames received with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble."]
    pub rx1024tomaxoctets_gb: RX1024TOMAXOCTETS_GB,
    #[doc = "0x1c4 - Number of received good unicast frames."]
    pub rxunicastframes_g: RXUNICASTFRAMES_G,
    #[doc = "0x1c8 - Number of frames received with length error (Length type field ≠ frame size), for all frames with valid length field."]
    pub rxlengtherror: RXLENGTHERROR,
    #[doc = "0x1cc - Number of frames received with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
    pub rxoutofrangetype: RXOUTOFRANGETYPE,
    #[doc = "0x1d0 - Number of good and valid Pause frames received."]
    pub rxpauseframes: RXPAUSEFRAMES,
    #[doc = "0x1d4 - Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration."]
    pub rxfifooverflow: RXFIFOOVERFLOW,
    #[doc = "0x1d8 - Number of good and bad VLAN frames received."]
    pub rxvlanframes_gb: RXVLANFRAMES_GB,
    #[doc = "0x1dc - Number of frames received with error because of watchdog timeout error (frames with a data load larger than 2,048 bytes or the value programmed in Register 55 (Watchdog Timeout Register))."]
    pub rxwatchdogerror: RXWATCHDOGERROR,
    #[doc = "0x1e0 - Number of frames received with Receive error or Frame Extension error on the GMII or MII interface."]
    pub rxrcverror: RXRCVERROR,
    #[doc = "0x1e4 - Number of received good control frames"]
    pub rxctrlframes_g: RXCTRLFRAMES_G,
    _reserved86: [u8; 0x18],
    #[doc = "0x200 - MMC IPC Receive Checksum Offload Interrupt Mask maintains the mask for the interrupt generated from the receive IPC statistic counters."]
    pub mmc_ipc_intr_mask_rx: MMC_IPC_INTR_MASK_RX,
    _reserved87: [u8; 0x04],
    #[doc = "0x208 - MMC Receive Checksum Offload Interrupt maintains the interrupt that the receive IPC statistic counters generate. See Table 4-25 for further detail."]
    pub mmc_ipc_intr_rx: MMC_IPC_INTR_RX,
    _reserved88: [u8; 0x04],
    #[doc = "0x210 - Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload"]
    pub rxipv4_gd_fms: RXIPV4_GD_FMS,
    #[doc = "0x214 - Number of IPv4 datagrams received with header (checksum, length, or version mismatch) errors"]
    pub rxipv4_hdrerr_frms: RXIPV4_HDRERR_FRMS,
    #[doc = "0x218 - Number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine"]
    pub rxipv4_nopay_frms: RXIPV4_NOPAY_FRMS,
    #[doc = "0x21c - Number of good IPv4 datagrams with fragmentation"]
    pub rxipv4_frag_frms: RXIPV4_FRAG_FRMS,
    #[doc = "0x220 - Number of good IPv4 datagrams received that had a UDP payload with checksum disabled"]
    pub rxipv4_udsbl_frms: RXIPV4_UDSBL_FRMS,
    #[doc = "0x224 - Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads"]
    pub rxipv6_gd_frms: RXIPV6_GD_FRMS,
    #[doc = "0x228 - Number of IPv6 datagrams received with header errors (length or version mismatch)"]
    pub rxipv6_hdrerr_frms: RXIPV6_HDRERR_FRMS,
    #[doc = "0x22c - Number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers"]
    pub rxipv6_nopay_frms: RXIPV6_NOPAY_FRMS,
    #[doc = "0x230 - Number of good IP datagrams with a good UDP payload. This counter is not updated when the rxipv4_udsbl_frms counter is incremented."]
    pub rxudp_gd_frms: RXUDP_GD_FRMS,
    #[doc = "0x234 - Number of good IP datagrams whose UDP payload has a checksum error"]
    pub rxudp_err_frms: RXUDP_ERR_FRMS,
    #[doc = "0x238 - Number of good IP datagrams with a good TCP payload"]
    pub rxtcp_gd_frms: RXTCP_GD_FRMS,
    #[doc = "0x23c - Number of good IP datagrams whose TCP payload has a checksum error"]
    pub rxtcp_err_frms: RXTCP_ERR_FRMS,
    #[doc = "0x240 - Number of good IP datagrams with a good ICMP payload"]
    pub rxicmp_gd_frms: RXICMP_GD_FRMS,
    #[doc = "0x244 - Number of good IP datagrams whose ICMP payload has a checksum error"]
    pub rxicmp_err_frms: RXICMP_ERR_FRMS,
    _reserved102: [u8; 0x08],
    #[doc = "0x250 - Number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. (Ethernet header, FCS, pad, or IP pad bytes are not included in this counter or in the octet counters listed below)."]
    pub rxipv4_gd_octets: RXIPV4_GD_OCTETS,
    #[doc = "0x254 - Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
    pub rxipv4_hdrerr_octets: RXIPV4_HDRERR_OCTETS,
    #[doc = "0x258 - Number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 header’s Length field is used to update this counter."]
    pub rxipv4_nopay_octets: RXIPV4_NOPAY_OCTETS,
    #[doc = "0x25c - Number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 header’s Length field is used to update this counter"]
    pub rxipv4_frag_octets: RXIPV4_FRAG_OCTETS,
    #[doc = "0x260 - Number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count IP Header bytes."]
    pub rxipv4_udsbl_octets: RXIPV4_UDSBL_OCTETS,
    #[doc = "0x264 - Number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data"]
    pub rxipv6_gd_octets: RXIPV6_GD_OCTETS,
    #[doc = "0x268 - Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header’s Length field is used to update this counter."]
    pub rxipv6_hdrerr_octets: RXIPV6_HDRERR_OCTETS,
    #[doc = "0x26c - Number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 header’s Length field is used to update this counter."]
    pub rxipv6_nopay_octets: RXIPV6_NOPAY_OCTETS,
    #[doc = "0x270 - Number of bytes received in a good UDP segment. This counter (and the counters below) does not count IP header bytes."]
    pub rxudp_gd_octets: RXUDP_GD_OCTETS,
    #[doc = "0x274 - Number of bytes received in a UDP segment that had checksum errors"]
    pub rxudp_err_octets: RXUDP_ERR_OCTETS,
    #[doc = "0x278 - Number of bytes received in a good TCP segment"]
    pub rxtcp_gd_octets: RXTCP_GD_OCTETS,
    #[doc = "0x27c - Number of bytes received in a TCP segment with checksum errors"]
    pub rxtcp_err_octets: RXTCP_ERR_OCTETS,
    #[doc = "0x280 - Number of bytes received in a good ICMP segment"]
    pub rxicmp_gd_octets: RXICMP_GD_OCTETS,
    _reserved115: [u8; 0x017c],
    #[doc = "0x400 - Layer 3 and Layer 4 Control Register"]
    pub l3_l4_cfg_0_l3_l4_ctrl: L3_L4_CFG_0_L3_L4_CTRL,
    #[doc = "0x404 - Layer 4 Address Register"]
    pub l3_l4_cfg_0_l4_addr: L3_L4_CFG_0_L4_ADDR,
    _reserved117: [u8; 0x08],
    #[doc = "0x410 - Layer 3 Address 0 Register"]
    pub l3_l4_cfg_0_l3_addr_0: L3_L4_CFG_0_L3_ADDR_0,
    #[doc = "0x414 - Layer 3 Address 1 Register"]
    pub l3_l4_cfg_0_l3_addr_1: L3_L4_CFG_0_L3_ADDR_1,
    #[doc = "0x418 - Layer 3 Address 2 Register"]
    pub l3_l4_cfg_0_l3_addr_2: L3_L4_CFG_0_L3_ADDR_2,
    #[doc = "0x41c - Layer 3 Address 3 Register"]
    pub l3_l4_cfg_0_l3_addr_3: L3_L4_CFG_0_L3_ADDR_3,
    _reserved121: [u8; 0x10],
    #[doc = "0x430 - Layer 3 and Layer 4 Control Register"]
    pub l3_l4_cfg_1_l3_l4_ctrl: L3_L4_CFG_1_L3_L4_CTRL,
    #[doc = "0x434 - Layer 4 Address Register"]
    pub l3_l4_cfg_1_l4_addr: L3_L4_CFG_1_L4_ADDR,
    _reserved123: [u8; 0x08],
    #[doc = "0x440 - Layer 3 Address 0 Register"]
    pub l3_l4_cfg_1_l3_addr_0: L3_L4_CFG_1_L3_ADDR_0,
    #[doc = "0x444 - Layer 3 Address 1 Register"]
    pub l3_l4_cfg_1_l3_addr_1: L3_L4_CFG_1_L3_ADDR_1,
    #[doc = "0x448 - Layer 3 Address 2 Register"]
    pub l3_l4_cfg_1_l3_addr_2: L3_L4_CFG_1_L3_ADDR_2,
    #[doc = "0x44c - Layer 3 Address 3 Register"]
    pub l3_l4_cfg_1_l3_addr_3: L3_L4_CFG_1_L3_ADDR_3,
    _reserved127: [u8; 0x10],
    #[doc = "0x460 - Layer 3 and Layer 4 Control Register"]
    pub l3_l4_cfg_2_l3_l4_ctrl: L3_L4_CFG_2_L3_L4_CTRL,
    #[doc = "0x464 - Layer 4 Address Register"]
    pub l3_l4_cfg_2_l4_addr: L3_L4_CFG_2_L4_ADDR,
    _reserved129: [u8; 0x08],
    #[doc = "0x470 - Layer 3 Address 0 Register"]
    pub l3_l4_cfg_2_l3_addr_0: L3_L4_CFG_2_L3_ADDR_0,
    #[doc = "0x474 - Layer 3 Address 1 Register"]
    pub l3_l4_cfg_2_l3_addr_1: L3_L4_CFG_2_L3_ADDR_1,
    #[doc = "0x478 - Layer 3 Address 2 Register"]
    pub l3_l4_cfg_2_l3_addr_2: L3_L4_CFG_2_L3_ADDR_2,
    #[doc = "0x47c - Layer 3 Address 3 Register"]
    pub l3_l4_cfg_2_l3_addr_3: L3_L4_CFG_2_L3_ADDR_3,
    _reserved133: [u8; 0x10],
    #[doc = "0x490 - Layer 3 and Layer 4 Control Register"]
    pub l3_l4_cfg_3_l3_l4_ctrl: L3_L4_CFG_3_L3_L4_CTRL,
    #[doc = "0x494 - Layer 4 Address Register"]
    pub l3_l4_cfg_3_l4_addr: L3_L4_CFG_3_L4_ADDR,
    _reserved135: [u8; 0x08],
    #[doc = "0x4a0 - Layer 3 Address 0 Register"]
    pub l3_l4_cfg_3_l3_addr_0: L3_L4_CFG_3_L3_ADDR_0,
    #[doc = "0x4a4 - Layer 3 Address 1 Register"]
    pub l3_l4_cfg_3_l3_addr_1: L3_L4_CFG_3_L3_ADDR_1,
    #[doc = "0x4a8 - Layer 3 Address 2 Register"]
    pub l3_l4_cfg_3_l3_addr_2: L3_L4_CFG_3_L3_ADDR_2,
    #[doc = "0x4ac - Layer 3 Address 3 Register"]
    pub l3_l4_cfg_3_l3_addr_3: L3_L4_CFG_3_L3_ADDR_3,
    _reserved139: [u8; 0x50],
    #[doc = "0x500 - Hash Table Register 0"]
    pub hash_table_register0: HASH_TABLE_REGISTER0,
    #[doc = "0x504 - Hash Table Register 1"]
    pub hash_table_register1: HASH_TABLE_REGISTER1,
    #[doc = "0x508 - Hash Table Register 2"]
    pub hash_table_register2: HASH_TABLE_REGISTER2,
    #[doc = "0x50c - Hash Table Register 3"]
    pub hash_table_register3: HASH_TABLE_REGISTER3,
    #[doc = "0x510 - Hash Table Register 4"]
    pub hash_table_register4: HASH_TABLE_REGISTER4,
    #[doc = "0x514 - Hash Table Register 5"]
    pub hash_table_register5: HASH_TABLE_REGISTER5,
    #[doc = "0x518 - Hash Table Register 6"]
    pub hash_table_register6: HASH_TABLE_REGISTER6,
    #[doc = "0x51c - Hash Table Register 7"]
    pub hash_table_register7: HASH_TABLE_REGISTER7,
    _reserved147: [u8; 0x64],
    #[doc = "0x584 - VLAN Tag Inclusion or Replacement Register"]
    pub vlan_tag_inc_rpl: VLAN_TAG_INC_RPL,
    #[doc = "0x588 - VLAN Hash Table Register"]
    pub vlan_hash: VLAN_HASH,
    _reserved149: [u8; 0x0174],
    #[doc = "0x700 - Timestamp Control Register"]
    pub ts_ctrl: TS_CTRL,
    #[doc = "0x704 - Sub-Second Increment Register"]
    pub sub_sec_incr: SUB_SEC_INCR,
    #[doc = "0x708 - System Time - Seconds Register"]
    pub syst_sec: SYST_SEC,
    #[doc = "0x70c - System Time - Nanoseconds Register"]
    pub syst_nsec: SYST_NSEC,
    #[doc = "0x710 - System Time - Seconds Update Register"]
    pub syst_sec_upd: SYST_SEC_UPD,
    #[doc = "0x714 - System Time - Nanoseconds Update Register"]
    pub syst_nsec_upd: SYST_NSEC_UPD,
    #[doc = "0x718 - Timestamp Addend Register"]
    pub ts_addend: TS_ADDEND,
    #[doc = "0x71c - Target Time Seconds Register"]
    pub tgttm_sec: TGTTM_SEC,
    #[doc = "0x720 - Target Time Nanoseconds Register"]
    pub tgttm_nsec: TGTTM_NSEC,
    #[doc = "0x724 - System Time - Higher Word Seconds Register"]
    pub systm_h_sec: SYSTM_H_SEC,
    #[doc = "0x728 - Timestamp Status Register"]
    pub ts_status: TS_STATUS,
    #[doc = "0x72c - PPS Control Register"]
    pub pps_ctrl: PPS_CTRL,
    #[doc = "0x730 - Auxiliary Timestamp - Nanoseconds Register"]
    pub aux_ts_nsec: AUX_TS_NSEC,
    #[doc = "0x734 - Auxiliary Timestamp - Seconds Register"]
    pub aux_ts_sec: AUX_TS_SEC,
    _reserved163: [u8; 0x28],
    #[doc = "0x760 - PPS0 Interval Register"]
    pub pps0_interval: PPS0_INTERVAL,
    #[doc = "0x764 - PPS0 Width Register"]
    pub pps0_width: PPS0_WIDTH,
    _reserved165: [u8; 0x18],
    #[doc = "0x780 - PPS Target Time Seconds Register"]
    pub pps_1_tgttm_sec: PPS_1_TGTTM_SEC,
    #[doc = "0x784 - PPS Target Time Nanoseconds Register"]
    pub pps_1_tgttm_nsec: PPS_1_TGTTM_NSEC,
    #[doc = "0x788 - PPS Interval Register"]
    pub pps_1_interval: PPS_1_INTERVAL,
    #[doc = "0x78c - PPS Width Register"]
    pub pps_1_width: PPS_1_WIDTH,
    _reserved169: [u8; 0x10],
    #[doc = "0x7a0 - PPS2 Target Time Seconds Register"]
    pub pps_2_tgttm_sec: PPS_2_TGTTM_SEC,
    #[doc = "0x7a4 - PPS Target Time Nanoseconds Register"]
    pub pps_2_tgttm_nsec: PPS_2_TGTTM_NSEC,
    #[doc = "0x7a8 - PPS Interval Register"]
    pub pps_2_interval: PPS_2_INTERVAL,
    #[doc = "0x7ac - PPS Width Register"]
    pub pps_2_width: PPS_2_WIDTH,
    _reserved173: [u8; 0x10],
    #[doc = "0x7c0 - PPS3 Target Time Seconds Register"]
    pub pps_3_tgttm_sec: PPS_3_TGTTM_SEC,
    #[doc = "0x7c4 - PPS Target Time Nanoseconds Register"]
    pub pps_3_tgttm_nsec: PPS_3_TGTTM_NSEC,
    #[doc = "0x7c8 - PPS Interval Register"]
    pub pps_3_interval: PPS_3_INTERVAL,
    #[doc = "0x7cc - PPS Width Register"]
    pub pps_3_width: PPS_3_WIDTH,
    _reserved177: [u8; 0x0830],
    #[doc = "0x1000 - Bus Mode Register"]
    pub dma_bus_mode: DMA_BUS_MODE,
    #[doc = "0x1004 - Transmit Poll Demand Register"]
    pub dma_tx_poll_demand: DMA_TX_POLL_DEMAND,
    #[doc = "0x1008 - Receive Poll Demand Register"]
    pub dma_rx_poll_demand: DMA_RX_POLL_DEMAND,
    #[doc = "0x100c - Receive Descriptor List Address Register"]
    pub dma_rx_desc_list_addr: DMA_RX_DESC_LIST_ADDR,
    #[doc = "0x1010 - Transmit Descriptor List Address Register"]
    pub dma_tx_desc_list_addr: DMA_TX_DESC_LIST_ADDR,
    #[doc = "0x1014 - Status Register"]
    pub dma_status: DMA_STATUS,
    #[doc = "0x1018 - Operation Mode Register"]
    pub dma_op_mode: DMA_OP_MODE,
    #[doc = "0x101c - Interrupt Enable Register"]
    pub dma_intr_en: DMA_INTR_EN,
    #[doc = "0x1020 - Missed Frame And Buffer Overflow Counter Register"]
    pub dma_miss_ovf_cnt: DMA_MISS_OVF_CNT,
    #[doc = "0x1024 - Receive Interrupt Watchdog Timer Register"]
    pub dma_rx_intr_wdog: DMA_RX_INTR_WDOG,
    #[doc = "0x1028 - AXI Bus Mode Register"]
    pub dma_axi_mode: DMA_AXI_MODE,
    #[doc = "0x102c - AHB or AXI Status Register"]
    pub dma_bus_status: DMA_BUS_STATUS,
    _reserved189: [u8; 0x18],
    #[doc = "0x1048 - Current Host Transmit Descriptor Register"]
    pub dma_curr_host_tx_desc: DMA_CURR_HOST_TX_DESC,
    #[doc = "0x104c - Current Host Receive Descriptor Register"]
    pub dma_curr_host_rx_desc: DMA_CURR_HOST_RX_DESC,
    #[doc = "0x1050 - Current Host Transmit Buffer Address Register"]
    pub dma_curr_host_tx_buf: DMA_CURR_HOST_TX_BUF,
    #[doc = "0x1054 - Current Host Receive Buffer Address Register"]
    pub dma_curr_host_rx_buf: DMA_CURR_HOST_RX_BUF,
    #[doc = "0x1058 - HW Feature Register"]
    pub dma_hw_feature: DMA_HW_FEATURE,
    _reserved194: [u8; 0x1fa4],
    #[doc = "0x3000 - Control Register 0"]
    pub ctrl0: CTRL0,
    _reserved195: [u8; 0x04],
    #[doc = "0x3008 - Control Register 1"]
    pub ctrl2: CTRL2,
}
#[doc = "MACCFG (rw) register accessor: an alias for `Reg<MACCFG_SPEC>`"]
pub type MACCFG = crate::Reg<maccfg::MACCFG_SPEC>;
#[doc = "MAC Configuration Register"]
pub mod maccfg;
#[doc = "MACFF (rw) register accessor: an alias for `Reg<MACFF_SPEC>`"]
pub type MACFF = crate::Reg<macff::MACFF_SPEC>;
#[doc = "MAC Frame Filter"]
pub mod macff;
#[doc = "HASH_H (rw) register accessor: an alias for `Reg<HASH_H_SPEC>`"]
pub type HASH_H = crate::Reg<hash_h::HASH_H_SPEC>;
#[doc = "Hash Table High Register"]
pub mod hash_h;
#[doc = "HASH_L (rw) register accessor: an alias for `Reg<HASH_L_SPEC>`"]
pub type HASH_L = crate::Reg<hash_l::HASH_L_SPEC>;
#[doc = "Hash Table Low Register"]
pub mod hash_l;
#[doc = "GMII_ADDR (rw) register accessor: an alias for `Reg<GMII_ADDR_SPEC>`"]
pub type GMII_ADDR = crate::Reg<gmii_addr::GMII_ADDR_SPEC>;
#[doc = "GMII Address Register"]
pub mod gmii_addr;
#[doc = "GMII_DATA (rw) register accessor: an alias for `Reg<GMII_DATA_SPEC>`"]
pub type GMII_DATA = crate::Reg<gmii_data::GMII_DATA_SPEC>;
#[doc = "GMII Data Register"]
pub mod gmii_data;
#[doc = "FLOWCTRL (rw) register accessor: an alias for `Reg<FLOWCTRL_SPEC>`"]
pub type FLOWCTRL = crate::Reg<flowctrl::FLOWCTRL_SPEC>;
#[doc = "Flow Control Register"]
pub mod flowctrl;
#[doc = "VLAN_TAG (rw) register accessor: an alias for `Reg<VLAN_TAG_SPEC>`"]
pub type VLAN_TAG = crate::Reg<vlan_tag::VLAN_TAG_SPEC>;
#[doc = "VLAN Tag Register"]
pub mod vlan_tag;
#[doc = "VERSION (r) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version Register"]
pub mod version;
#[doc = "DEBUGGING (r) register accessor: an alias for `Reg<DEBUGGING_SPEC>`"]
pub type DEBUGGING = crate::Reg<debugging::DEBUGGING_SPEC>;
#[doc = "Debug Register"]
pub mod debugging;
#[doc = "RWKFRMFILT (rw) register accessor: an alias for `Reg<RWKFRMFILT_SPEC>`"]
pub type RWKFRMFILT = crate::Reg<rwkfrmfilt::RWKFRMFILT_SPEC>;
#[doc = "Remote Wake-Up Frame Filter Register"]
pub mod rwkfrmfilt;
#[doc = "PMT_CSR (rw) register accessor: an alias for `Reg<PMT_CSR_SPEC>`"]
pub type PMT_CSR = crate::Reg<pmt_csr::PMT_CSR_SPEC>;
#[doc = "PMT Control and Status Register"]
pub mod pmt_csr;
#[doc = "LPI_CSR (rw) register accessor: an alias for `Reg<LPI_CSR_SPEC>`"]
pub type LPI_CSR = crate::Reg<lpi_csr::LPI_CSR_SPEC>;
#[doc = "LPI Control and Status Regsiter"]
pub mod lpi_csr;
#[doc = "LPI_TCR (rw) register accessor: an alias for `Reg<LPI_TCR_SPEC>`"]
pub type LPI_TCR = crate::Reg<lpi_tcr::LPI_TCR_SPEC>;
#[doc = "LPI Timers Control Register"]
pub mod lpi_tcr;
#[doc = "INTR_STATUS (r) register accessor: an alias for `Reg<INTR_STATUS_SPEC>`"]
pub type INTR_STATUS = crate::Reg<intr_status::INTR_STATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intr_status;
#[doc = "INTR_MASK (rw) register accessor: an alias for `Reg<INTR_MASK_SPEC>`"]
pub type INTR_MASK = crate::Reg<intr_mask::INTR_MASK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod intr_mask;
#[doc = "MAC_ADDR_0_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDR_0_HIGH_SPEC>`"]
pub type MAC_ADDR_0_HIGH = crate::Reg<mac_addr_0_high::MAC_ADDR_0_HIGH_SPEC>;
#[doc = "MAC Address 0 High Register"]
pub mod mac_addr_0_high;
#[doc = "MAC_ADDR_0_LOW (rw) register accessor: an alias for `Reg<MAC_ADDR_0_LOW_SPEC>`"]
pub type MAC_ADDR_0_LOW = crate::Reg<mac_addr_0_low::MAC_ADDR_0_LOW_SPEC>;
#[doc = "MAC Address 0 Low Register"]
pub mod mac_addr_0_low;
#[doc = "MAC_ADDR_1_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDR_1_HIGH_SPEC>`"]
pub type MAC_ADDR_1_HIGH = crate::Reg<mac_addr_1_high::MAC_ADDR_1_HIGH_SPEC>;
#[doc = "MAC Address High Register"]
pub mod mac_addr_1_high;
#[doc = "MAC_ADDR_1_LOW (rw) register accessor: an alias for `Reg<MAC_ADDR_1_LOW_SPEC>`"]
pub type MAC_ADDR_1_LOW = crate::Reg<mac_addr_1_low::MAC_ADDR_1_LOW_SPEC>;
#[doc = "MAC Address Low Register"]
pub mod mac_addr_1_low;
#[doc = "MAC_ADDR_2_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDR_2_HIGH_SPEC>`"]
pub type MAC_ADDR_2_HIGH = crate::Reg<mac_addr_2_high::MAC_ADDR_2_HIGH_SPEC>;
#[doc = "MAC Address2 High Register"]
pub mod mac_addr_2_high;
#[doc = "MAC_ADDR_2_LOW (rw) register accessor: an alias for `Reg<MAC_ADDR_2_LOW_SPEC>`"]
pub type MAC_ADDR_2_LOW = crate::Reg<mac_addr_2_low::MAC_ADDR_2_LOW_SPEC>;
#[doc = "MAC Address2 Low Register"]
pub mod mac_addr_2_low;
#[doc = "MAC_ADDR_3_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDR_3_HIGH_SPEC>`"]
pub type MAC_ADDR_3_HIGH = crate::Reg<mac_addr_3_high::MAC_ADDR_3_HIGH_SPEC>;
#[doc = "MAC Address3 High Register"]
pub mod mac_addr_3_high;
#[doc = "MAC_ADDR_3_LOW (rw) register accessor: an alias for `Reg<MAC_ADDR_3_LOW_SPEC>`"]
pub type MAC_ADDR_3_LOW = crate::Reg<mac_addr_3_low::MAC_ADDR_3_LOW_SPEC>;
#[doc = "MAC Address3 Low Register"]
pub mod mac_addr_3_low;
#[doc = "MAC_ADDR_4_HIGH (rw) register accessor: an alias for `Reg<MAC_ADDR_4_HIGH_SPEC>`"]
pub type MAC_ADDR_4_HIGH = crate::Reg<mac_addr_4_high::MAC_ADDR_4_HIGH_SPEC>;
#[doc = "MAC Address4 High Register"]
pub mod mac_addr_4_high;
#[doc = "MAC_ADDR_4_LOW (rw) register accessor: an alias for `Reg<MAC_ADDR_4_LOW_SPEC>`"]
pub type MAC_ADDR_4_LOW = crate::Reg<mac_addr_4_low::MAC_ADDR_4_LOW_SPEC>;
#[doc = "MAC Address4 Low Register"]
pub mod mac_addr_4_low;
#[doc = "XMII_CSR (rw) register accessor: an alias for `Reg<XMII_CSR_SPEC>`"]
pub type XMII_CSR = crate::Reg<xmii_csr::XMII_CSR_SPEC>;
#[doc = "SGMII/RGMII/SMII Control and Status Register"]
pub mod xmii_csr;
#[doc = "WDOG_WTO (rw) register accessor: an alias for `Reg<WDOG_WTO_SPEC>`"]
pub type WDOG_WTO = crate::Reg<wdog_wto::WDOG_WTO_SPEC>;
#[doc = "Watchdog Timeout Register"]
pub mod wdog_wto;
#[doc = "GPIO (rw) register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "General Purpose IO Register"]
pub mod gpio;
#[doc = "MMC_CNTRL (rw) register accessor: an alias for `Reg<MMC_CNTRL_SPEC>`"]
pub type MMC_CNTRL = crate::Reg<mmc_cntrl::MMC_CNTRL_SPEC>;
#[doc = "MMC Control establishes the operating mode of MMC."]
pub mod mmc_cntrl;
#[doc = "MMC_INTR_RX (rw) register accessor: an alias for `Reg<MMC_INTR_RX_SPEC>`"]
pub type MMC_INTR_RX = crate::Reg<mmc_intr_rx::MMC_INTR_RX_SPEC>;
#[doc = "MMC Receive Interrupt maintains the interrupt generated from all of the receive statistic counters."]
pub mod mmc_intr_rx;
#[doc = "MMC_INTR_TX (rw) register accessor: an alias for `Reg<MMC_INTR_TX_SPEC>`"]
pub type MMC_INTR_TX = crate::Reg<mmc_intr_tx::MMC_INTR_TX_SPEC>;
#[doc = "MMC Transmit Interrupt maintains the interrupt generated from all of the transmit statistic counters"]
pub mod mmc_intr_tx;
#[doc = "MMC_INTR_MASK_RX (rw) register accessor: an alias for `Reg<MMC_INTR_MASK_RX_SPEC>`"]
pub type MMC_INTR_MASK_RX = crate::Reg<mmc_intr_mask_rx::MMC_INTR_MASK_RX_SPEC>;
#[doc = "MMC Receive Interrupt mask maintains the mask for the interrupt generated from all of the receive statistic counters"]
pub mod mmc_intr_mask_rx;
#[doc = "MMC_INTR_MASK_TX (rw) register accessor: an alias for `Reg<MMC_INTR_MASK_TX_SPEC>`"]
pub type MMC_INTR_MASK_TX = crate::Reg<mmc_intr_mask_tx::MMC_INTR_MASK_TX_SPEC>;
#[doc = "MMC Transmit Interrupt Mask"]
pub mod mmc_intr_mask_tx;
#[doc = "TXOCTETCOUNT_GB (rw) register accessor: an alias for `Reg<TXOCTETCOUNT_GB_SPEC>`"]
pub type TXOCTETCOUNT_GB = crate::Reg<txoctetcount_gb::TXOCTETCOUNT_GB_SPEC>;
#[doc = "Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
pub mod txoctetcount_gb;
#[doc = "TXFRAMECOUNT_GB (rw) register accessor: an alias for `Reg<TXFRAMECOUNT_GB_SPEC>`"]
pub type TXFRAMECOUNT_GB = crate::Reg<txframecount_gb::TXFRAMECOUNT_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted, exclusive of retried frames."]
pub mod txframecount_gb;
#[doc = "TXBROADCASTFRAMES_G (rw) register accessor: an alias for `Reg<TXBROADCASTFRAMES_G_SPEC>`"]
pub type TXBROADCASTFRAMES_G = crate::Reg<txbroadcastframes_g::TXBROADCASTFRAMES_G_SPEC>;
#[doc = "Number of good broadcast frames transmitted"]
pub mod txbroadcastframes_g;
#[doc = "TXMLTICASTFRAMES_G (rw) register accessor: an alias for `Reg<TXMLTICASTFRAMES_G_SPEC>`"]
pub type TXMLTICASTFRAMES_G = crate::Reg<txmlticastframes_g::TXMLTICASTFRAMES_G_SPEC>;
#[doc = "Number of good multicast frames transmitted"]
pub mod txmlticastframes_g;
#[doc = "TX64OCTETS_GB (rw) register accessor: an alias for `Reg<TX64OCTETS_GB_SPEC>`"]
pub type TX64OCTETS_GB = crate::Reg<tx64octets_gb::TX64OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted with length 64 bytes, exclusive of preamble and retried frames."]
pub mod tx64octets_gb;
#[doc = "TX65TO127OCTETS_GB (rw) register accessor: an alias for `Reg<TX65TO127OCTETS_GB_SPEC>`"]
pub type TX65TO127OCTETS_GB = crate::Reg<tx65to127octets_gb::TX65TO127OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted with length between 65 and 127 (inclusive) bytes, exclusive of preamble and retried frames."]
pub mod tx65to127octets_gb;
#[doc = "TX128TO255OCTETS_GB (rw) register accessor: an alias for `Reg<TX128TO255OCTETS_GB_SPEC>`"]
pub type TX128TO255OCTETS_GB = crate::Reg<tx128to255octets_gb::TX128TO255OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted with length between 128 and 255 (inclusive) bytes, exclusive of preamble and retried frames."]
pub mod tx128to255octets_gb;
#[doc = "TX256TO511OCTETS_GB (rw) register accessor: an alias for `Reg<TX256TO511OCTETS_GB_SPEC>`"]
pub type TX256TO511OCTETS_GB = crate::Reg<tx256to511octets_gb::TX256TO511OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted with length between 256 and 511 (inclusive) bytes, exclusive of preamble and retried frames."]
pub mod tx256to511octets_gb;
#[doc = "TX512TO1023OCTETS_GB (rw) register accessor: an alias for `Reg<TX512TO1023OCTETS_GB_SPEC>`"]
pub type TX512TO1023OCTETS_GB = crate::Reg<tx512to1023octets_gb::TX512TO1023OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted with length between 512 and 1,023 (inclusive) bytes, exclusive of preamble and retried frames."]
pub mod tx512to1023octets_gb;
#[doc = "TX1024TOMAXOCTETS_GB (rw) register accessor: an alias for `Reg<TX1024TOMAXOCTETS_GB_SPEC>`"]
pub type TX1024TOMAXOCTETS_GB = crate::Reg<tx1024tomaxoctets_gb::TX1024TOMAXOCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames transmitted with length between 1,024 and maxsize (inclusive) bytes, exclusive of preamble and retried frames."]
pub mod tx1024tomaxoctets_gb;
#[doc = "TXUNICASTFRAMES_GB (rw) register accessor: an alias for `Reg<TXUNICASTFRAMES_GB_SPEC>`"]
pub type TXUNICASTFRAMES_GB = crate::Reg<txunicastframes_gb::TXUNICASTFRAMES_GB_SPEC>;
#[doc = "Number of good and bad unicast frames transmitted."]
pub mod txunicastframes_gb;
#[doc = "TXMULTICASTFRAMES_GB (rw) register accessor: an alias for `Reg<TXMULTICASTFRAMES_GB_SPEC>`"]
pub type TXMULTICASTFRAMES_GB = crate::Reg<txmulticastframes_gb::TXMULTICASTFRAMES_GB_SPEC>;
#[doc = "Number of good and bad multicast frames transmitted."]
pub mod txmulticastframes_gb;
#[doc = "TXBROADCASTFRAMES_GB (rw) register accessor: an alias for `Reg<TXBROADCASTFRAMES_GB_SPEC>`"]
pub type TXBROADCASTFRAMES_GB = crate::Reg<txbroadcastframes_gb::TXBROADCASTFRAMES_GB_SPEC>;
#[doc = "Number of good and bad broadcast frames transmitted."]
pub mod txbroadcastframes_gb;
#[doc = "TXUNDERFLOWERROR (rw) register accessor: an alias for `Reg<TXUNDERFLOWERROR_SPEC>`"]
pub type TXUNDERFLOWERROR = crate::Reg<txunderflowerror::TXUNDERFLOWERROR_SPEC>;
#[doc = "Number of frames aborted because of frame underflow error."]
pub mod txunderflowerror;
#[doc = "TXSINGLECOL_G (rw) register accessor: an alias for `Reg<TXSINGLECOL_G_SPEC>`"]
pub type TXSINGLECOL_G = crate::Reg<txsinglecol_g::TXSINGLECOL_G_SPEC>;
#[doc = "Number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub mod txsinglecol_g;
#[doc = "TXMULTICOL_G (rw) register accessor: an alias for `Reg<TXMULTICOL_G_SPEC>`"]
pub type TXMULTICOL_G = crate::Reg<txmulticol_g::TXMULTICOL_G_SPEC>;
#[doc = "Number of successfully transmitted frames after multiple collisions in the half-duplex mode."]
pub mod txmulticol_g;
#[doc = "TXDEFERRED (rw) register accessor: an alias for `Reg<TXDEFERRED_SPEC>`"]
pub type TXDEFERRED = crate::Reg<txdeferred::TXDEFERRED_SPEC>;
#[doc = "Number of successfully transmitted frames after a deferral in the half-duplex mode."]
pub mod txdeferred;
#[doc = "TXLATECOL (rw) register accessor: an alias for `Reg<TXLATECOL_SPEC>`"]
pub type TXLATECOL = crate::Reg<txlatecol::TXLATECOL_SPEC>;
#[doc = "Number of frames aborted because of late collision error"]
pub mod txlatecol;
#[doc = "TXEXESSCOL (rw) register accessor: an alias for `Reg<TXEXESSCOL_SPEC>`"]
pub type TXEXESSCOL = crate::Reg<txexesscol::TXEXESSCOL_SPEC>;
#[doc = "Number of frames aborted because of excessive (16) collision errors"]
pub mod txexesscol;
#[doc = "TXCARRIERERROR (rw) register accessor: an alias for `Reg<TXCARRIERERROR_SPEC>`"]
pub type TXCARRIERERROR = crate::Reg<txcarriererror::TXCARRIERERROR_SPEC>;
#[doc = "Number of frames aborted because of carrier sense error (no carrier or loss of carrier)."]
pub mod txcarriererror;
#[doc = "TXOCTETCOUNT_G (rw) register accessor: an alias for `Reg<TXOCTETCOUNT_G_SPEC>`"]
pub type TXOCTETCOUNT_G = crate::Reg<txoctetcount_g::TXOCTETCOUNT_G_SPEC>;
#[doc = "Number of bytes transmitted, exclusive of preamble, only in good frames."]
pub mod txoctetcount_g;
#[doc = "TXFRAMECOUNT_G (rw) register accessor: an alias for `Reg<TXFRAMECOUNT_G_SPEC>`"]
pub type TXFRAMECOUNT_G = crate::Reg<txframecount_g::TXFRAMECOUNT_G_SPEC>;
#[doc = "Number of good frames transmitted"]
pub mod txframecount_g;
#[doc = "TXEXCESSDEF (rw) register accessor: an alias for `Reg<TXEXCESSDEF_SPEC>`"]
pub type TXEXCESSDEF = crate::Reg<txexcessdef::TXEXCESSDEF_SPEC>;
#[doc = "Number of frames aborted because of excessive deferral error (deferred for more than two max-sized frame times)."]
pub mod txexcessdef;
#[doc = "TXPAUSEFRAMES (rw) register accessor: an alias for `Reg<TXPAUSEFRAMES_SPEC>`"]
pub type TXPAUSEFRAMES = crate::Reg<txpauseframes::TXPAUSEFRAMES_SPEC>;
#[doc = "Number of good Pause frames transmitted"]
pub mod txpauseframes;
#[doc = "TXVLANFRAMES_G (rw) register accessor: an alias for `Reg<TXVLANFRAMES_G_SPEC>`"]
pub type TXVLANFRAMES_G = crate::Reg<txvlanframes_g::TXVLANFRAMES_G_SPEC>;
#[doc = "Number of good VLAN frames transmitted, exclusive of retried frames."]
pub mod txvlanframes_g;
#[doc = "TXOVERSIZE_G (rw) register accessor: an alias for `Reg<TXOVERSIZE_G_SPEC>`"]
pub type TXOVERSIZE_G = crate::Reg<txoversize_g::TXOVERSIZE_G_SPEC>;
#[doc = "Number of frames transmitted without errors and with length greater than the maxsize (1,518 or 1,522 bytes for VLAN tagged frames; 2000 bytes if enabled in Bit 27 of Register 0 (MAC Configuration Register))."]
pub mod txoversize_g;
#[doc = "RXFRAMECOUNT_GB (rw) register accessor: an alias for `Reg<RXFRAMECOUNT_GB_SPEC>`"]
pub type RXFRAMECOUNT_GB = crate::Reg<rxframecount_gb::RXFRAMECOUNT_GB_SPEC>;
#[doc = "Number of good and bad frames received"]
pub mod rxframecount_gb;
#[doc = "RXOCTETCOUNT_G (rw) register accessor: an alias for `Reg<RXOCTETCOUNT_G_SPEC>`"]
pub type RXOCTETCOUNT_G = crate::Reg<rxoctetcount_g::RXOCTETCOUNT_G_SPEC>;
#[doc = "Number of bytes received, exclusive of preamble, only in good frames."]
pub mod rxoctetcount_g;
#[doc = "RXOCTETCOUNT_GB (rw) register accessor: an alias for `Reg<RXOCTETCOUNT_GB_SPEC>`"]
pub type RXOCTETCOUNT_GB = crate::Reg<rxoctetcount_gb::RXOCTETCOUNT_GB_SPEC>;
#[doc = "Number of bytes received, exclusive of preamble, in good and bad frames."]
pub mod rxoctetcount_gb;
#[doc = "RXBROADCASTFRAMES_G (rw) register accessor: an alias for `Reg<RXBROADCASTFRAMES_G_SPEC>`"]
pub type RXBROADCASTFRAMES_G = crate::Reg<rxbroadcastframes_g::RXBROADCASTFRAMES_G_SPEC>;
#[doc = "Number of good broadcast frames received"]
pub mod rxbroadcastframes_g;
#[doc = "RXMULTICASTFRAMES_G (rw) register accessor: an alias for `Reg<RXMULTICASTFRAMES_G_SPEC>`"]
pub type RXMULTICASTFRAMES_G = crate::Reg<rxmulticastframes_g::RXMULTICASTFRAMES_G_SPEC>;
#[doc = "Number of good multicast frames received"]
pub mod rxmulticastframes_g;
#[doc = "RXCRCERROR (rw) register accessor: an alias for `Reg<RXCRCERROR_SPEC>`"]
pub type RXCRCERROR = crate::Reg<rxcrcerror::RXCRCERROR_SPEC>;
#[doc = "Number of frames received with CRC error"]
pub mod rxcrcerror;
#[doc = "RXALIGNMENTERROR (rw) register accessor: an alias for `Reg<RXALIGNMENTERROR_SPEC>`"]
pub type RXALIGNMENTERROR = crate::Reg<rxalignmenterror::RXALIGNMENTERROR_SPEC>;
#[doc = "Number of frames received with alignment (dribble) error. Valid only in 10/100 mode"]
pub mod rxalignmenterror;
#[doc = "RXRUNTERROR (rw) register accessor: an alias for `Reg<RXRUNTERROR_SPEC>`"]
pub type RXRUNTERROR = crate::Reg<rxrunterror::RXRUNTERROR_SPEC>;
#[doc = "Number of frames received with runt (<64 bytes and CRC error) error."]
pub mod rxrunterror;
#[doc = "RXJABBERERROR (rw) register accessor: an alias for `Reg<RXJABBERERROR_SPEC>`"]
pub type RXJABBERERROR = crate::Reg<rxjabbererror::RXJABBERERROR_SPEC>;
#[doc = "Number of giant frames received with length (including CRC) greater than 1,518 bytes (1,522 bytes for VLAN tagged) and with CRC error. If Jumbo Frame mode is enabled, then frames of length greater than 9,018 bytes (9,022 for VLAN tagged) are considered as giant frames."]
pub mod rxjabbererror;
#[doc = "RXUNDERSIZE_G (rw) register accessor: an alias for `Reg<RXUNDERSIZE_G_SPEC>`"]
pub type RXUNDERSIZE_G = crate::Reg<rxundersize_g::RXUNDERSIZE_G_SPEC>;
#[doc = "Number of frames received with length less than 64 bytes, without any errors."]
pub mod rxundersize_g;
#[doc = "RXOVERSIZE_G (rw) register accessor: an alias for `Reg<RXOVERSIZE_G_SPEC>`"]
pub type RXOVERSIZE_G = crate::Reg<rxoversize_g::RXOVERSIZE_G_SPEC>;
#[doc = "Number of frames received without errors, with length greater than the maxsize (1,518 or 1,522 for VLAN tagged frames; 2,000 bytes if enabled in Bit 27 of Register 0 (MAC Configuration Register))"]
pub mod rxoversize_g;
#[doc = "RX64OCTETS_GB (rw) register accessor: an alias for `Reg<RX64OCTETS_GB_SPEC>`"]
pub type RX64OCTETS_GB = crate::Reg<rx64octets_gb::RX64OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames received with length 64 bytes, exclusive of preamble."]
pub mod rx64octets_gb;
#[doc = "RX65TO127OCTETS_GB (rw) register accessor: an alias for `Reg<RX65TO127OCTETS_GB_SPEC>`"]
pub type RX65TO127OCTETS_GB = crate::Reg<rx65to127octets_gb::RX65TO127OCTETS_GB_SPEC>;
#[doc = "No description avaiable"]
pub mod rx65to127octets_gb;
#[doc = "RX128TO255OCTETS_GB (rw) register accessor: an alias for `Reg<RX128TO255OCTETS_GB_SPEC>`"]
pub type RX128TO255OCTETS_GB = crate::Reg<rx128to255octets_gb::RX128TO255OCTETS_GB_SPEC>;
#[doc = "No description avaiable"]
pub mod rx128to255octets_gb;
#[doc = "RX256TO511OCTETS_GB (rw) register accessor: an alias for `Reg<RX256TO511OCTETS_GB_SPEC>`"]
pub type RX256TO511OCTETS_GB = crate::Reg<rx256to511octets_gb::RX256TO511OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames received with length between 256 and 511 (inclusive) bytes, exclusive of preamble."]
pub mod rx256to511octets_gb;
#[doc = "RX512TO1023OCTETS_GB (rw) register accessor: an alias for `Reg<RX512TO1023OCTETS_GB_SPEC>`"]
pub type RX512TO1023OCTETS_GB = crate::Reg<rx512to1023octets_gb::RX512TO1023OCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames received with length between 512 and 1023 (inclusive) bytes, exclusive of preamble."]
pub mod rx512to1023octets_gb;
#[doc = "RX1024TOMAXOCTETS_GB (rw) register accessor: an alias for `Reg<RX1024TOMAXOCTETS_GB_SPEC>`"]
pub type RX1024TOMAXOCTETS_GB = crate::Reg<rx1024tomaxoctets_gb::RX1024TOMAXOCTETS_GB_SPEC>;
#[doc = "Number of good and bad frames received with length between 1024 and maxsize (inclusive) bytes, exclusive of preamble."]
pub mod rx1024tomaxoctets_gb;
#[doc = "RXUNICASTFRAMES_G (rw) register accessor: an alias for `Reg<RXUNICASTFRAMES_G_SPEC>`"]
pub type RXUNICASTFRAMES_G = crate::Reg<rxunicastframes_g::RXUNICASTFRAMES_G_SPEC>;
#[doc = "Number of received good unicast frames."]
pub mod rxunicastframes_g;
#[doc = "RXLENGTHERROR (rw) register accessor: an alias for `Reg<RXLENGTHERROR_SPEC>`"]
pub type RXLENGTHERROR = crate::Reg<rxlengtherror::RXLENGTHERROR_SPEC>;
#[doc = "Number of frames received with length error (Length type field ≠ frame size), for all frames with valid length field."]
pub mod rxlengtherror;
#[doc = "RXOUTOFRANGETYPE (rw) register accessor: an alias for `Reg<RXOUTOFRANGETYPE_SPEC>`"]
pub type RXOUTOFRANGETYPE = crate::Reg<rxoutofrangetype::RXOUTOFRANGETYPE_SPEC>;
#[doc = "Number of frames received with length field not equal to the valid frame size (greater than 1,500 but less than 1,536)."]
pub mod rxoutofrangetype;
#[doc = "RXPAUSEFRAMES (rw) register accessor: an alias for `Reg<RXPAUSEFRAMES_SPEC>`"]
pub type RXPAUSEFRAMES = crate::Reg<rxpauseframes::RXPAUSEFRAMES_SPEC>;
#[doc = "Number of good and valid Pause frames received."]
pub mod rxpauseframes;
#[doc = "RXFIFOOVERFLOW (rw) register accessor: an alias for `Reg<RXFIFOOVERFLOW_SPEC>`"]
pub type RXFIFOOVERFLOW = crate::Reg<rxfifooverflow::RXFIFOOVERFLOW_SPEC>;
#[doc = "Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration."]
pub mod rxfifooverflow;
#[doc = "RXVLANFRAMES_GB (rw) register accessor: an alias for `Reg<RXVLANFRAMES_GB_SPEC>`"]
pub type RXVLANFRAMES_GB = crate::Reg<rxvlanframes_gb::RXVLANFRAMES_GB_SPEC>;
#[doc = "Number of good and bad VLAN frames received."]
pub mod rxvlanframes_gb;
#[doc = "RXWATCHDOGERROR (rw) register accessor: an alias for `Reg<RXWATCHDOGERROR_SPEC>`"]
pub type RXWATCHDOGERROR = crate::Reg<rxwatchdogerror::RXWATCHDOGERROR_SPEC>;
#[doc = "Number of frames received with error because of watchdog timeout error (frames with a data load larger than 2,048 bytes or the value programmed in Register 55 (Watchdog Timeout Register))."]
pub mod rxwatchdogerror;
#[doc = "RXRCVERROR (rw) register accessor: an alias for `Reg<RXRCVERROR_SPEC>`"]
pub type RXRCVERROR = crate::Reg<rxrcverror::RXRCVERROR_SPEC>;
#[doc = "Number of frames received with Receive error or Frame Extension error on the GMII or MII interface."]
pub mod rxrcverror;
#[doc = "RXCTRLFRAMES_G (rw) register accessor: an alias for `Reg<RXCTRLFRAMES_G_SPEC>`"]
pub type RXCTRLFRAMES_G = crate::Reg<rxctrlframes_g::RXCTRLFRAMES_G_SPEC>;
#[doc = "Number of received good control frames"]
pub mod rxctrlframes_g;
#[doc = "MMC_IPC_INTR_MASK_RX (rw) register accessor: an alias for `Reg<MMC_IPC_INTR_MASK_RX_SPEC>`"]
pub type MMC_IPC_INTR_MASK_RX = crate::Reg<mmc_ipc_intr_mask_rx::MMC_IPC_INTR_MASK_RX_SPEC>;
#[doc = "MMC IPC Receive Checksum Offload Interrupt Mask maintains the mask for the interrupt generated from the receive IPC statistic counters."]
pub mod mmc_ipc_intr_mask_rx;
#[doc = "MMC_IPC_INTR_RX (rw) register accessor: an alias for `Reg<MMC_IPC_INTR_RX_SPEC>`"]
pub type MMC_IPC_INTR_RX = crate::Reg<mmc_ipc_intr_rx::MMC_IPC_INTR_RX_SPEC>;
#[doc = "MMC Receive Checksum Offload Interrupt maintains the interrupt that the receive IPC statistic counters generate. See Table 4-25 for further detail."]
pub mod mmc_ipc_intr_rx;
#[doc = "RXIPV4_GD_FMS (rw) register accessor: an alias for `Reg<RXIPV4_GD_FMS_SPEC>`"]
pub type RXIPV4_GD_FMS = crate::Reg<rxipv4_gd_fms::RXIPV4_GD_FMS_SPEC>;
#[doc = "Number of good IPv4 datagrams received with the TCP, UDP, or ICMP payload"]
pub mod rxipv4_gd_fms;
#[doc = "RXIPV4_HDRERR_FRMS (rw) register accessor: an alias for `Reg<RXIPV4_HDRERR_FRMS_SPEC>`"]
pub type RXIPV4_HDRERR_FRMS = crate::Reg<rxipv4_hdrerr_frms::RXIPV4_HDRERR_FRMS_SPEC>;
#[doc = "Number of IPv4 datagrams received with header (checksum, length, or version mismatch) errors"]
pub mod rxipv4_hdrerr_frms;
#[doc = "RXIPV4_NOPAY_FRMS (rw) register accessor: an alias for `Reg<RXIPV4_NOPAY_FRMS_SPEC>`"]
pub type RXIPV4_NOPAY_FRMS = crate::Reg<rxipv4_nopay_frms::RXIPV4_NOPAY_FRMS_SPEC>;
#[doc = "Number of IPv4 datagram frames received that did not have a TCP, UDP, or ICMP payload processed by the Checksum engine"]
pub mod rxipv4_nopay_frms;
#[doc = "RXIPV4_FRAG_FRMS (rw) register accessor: an alias for `Reg<RXIPV4_FRAG_FRMS_SPEC>`"]
pub type RXIPV4_FRAG_FRMS = crate::Reg<rxipv4_frag_frms::RXIPV4_FRAG_FRMS_SPEC>;
#[doc = "Number of good IPv4 datagrams with fragmentation"]
pub mod rxipv4_frag_frms;
#[doc = "RXIPV4_UDSBL_FRMS (rw) register accessor: an alias for `Reg<RXIPV4_UDSBL_FRMS_SPEC>`"]
pub type RXIPV4_UDSBL_FRMS = crate::Reg<rxipv4_udsbl_frms::RXIPV4_UDSBL_FRMS_SPEC>;
#[doc = "Number of good IPv4 datagrams received that had a UDP payload with checksum disabled"]
pub mod rxipv4_udsbl_frms;
#[doc = "RXIPV6_GD_FRMS (rw) register accessor: an alias for `Reg<RXIPV6_GD_FRMS_SPEC>`"]
pub type RXIPV6_GD_FRMS = crate::Reg<rxipv6_gd_frms::RXIPV6_GD_FRMS_SPEC>;
#[doc = "Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads"]
pub mod rxipv6_gd_frms;
#[doc = "RXIPV6_HDRERR_FRMS (rw) register accessor: an alias for `Reg<RXIPV6_HDRERR_FRMS_SPEC>`"]
pub type RXIPV6_HDRERR_FRMS = crate::Reg<rxipv6_hdrerr_frms::RXIPV6_HDRERR_FRMS_SPEC>;
#[doc = "Number of IPv6 datagrams received with header errors (length or version mismatch)"]
pub mod rxipv6_hdrerr_frms;
#[doc = "RXIPV6_NOPAY_FRMS (rw) register accessor: an alias for `Reg<RXIPV6_NOPAY_FRMS_SPEC>`"]
pub type RXIPV6_NOPAY_FRMS = crate::Reg<rxipv6_nopay_frms::RXIPV6_NOPAY_FRMS_SPEC>;
#[doc = "Number of IPv6 datagram frames received that did not have a TCP, UDP, or ICMP payload. This includes all IPv6 datagrams with fragmentation or security extension headers"]
pub mod rxipv6_nopay_frms;
#[doc = "RXUDP_GD_FRMS (rw) register accessor: an alias for `Reg<RXUDP_GD_FRMS_SPEC>`"]
pub type RXUDP_GD_FRMS = crate::Reg<rxudp_gd_frms::RXUDP_GD_FRMS_SPEC>;
#[doc = "Number of good IP datagrams with a good UDP payload. This counter is not updated when the rxipv4_udsbl_frms counter is incremented."]
pub mod rxudp_gd_frms;
#[doc = "RXUDP_ERR_FRMS (rw) register accessor: an alias for `Reg<RXUDP_ERR_FRMS_SPEC>`"]
pub type RXUDP_ERR_FRMS = crate::Reg<rxudp_err_frms::RXUDP_ERR_FRMS_SPEC>;
#[doc = "Number of good IP datagrams whose UDP payload has a checksum error"]
pub mod rxudp_err_frms;
#[doc = "RXTCP_GD_FRMS (rw) register accessor: an alias for `Reg<RXTCP_GD_FRMS_SPEC>`"]
pub type RXTCP_GD_FRMS = crate::Reg<rxtcp_gd_frms::RXTCP_GD_FRMS_SPEC>;
#[doc = "Number of good IP datagrams with a good TCP payload"]
pub mod rxtcp_gd_frms;
#[doc = "RXTCP_ERR_FRMS (rw) register accessor: an alias for `Reg<RXTCP_ERR_FRMS_SPEC>`"]
pub type RXTCP_ERR_FRMS = crate::Reg<rxtcp_err_frms::RXTCP_ERR_FRMS_SPEC>;
#[doc = "Number of good IP datagrams whose TCP payload has a checksum error"]
pub mod rxtcp_err_frms;
#[doc = "RXICMP_GD_FRMS (rw) register accessor: an alias for `Reg<RXICMP_GD_FRMS_SPEC>`"]
pub type RXICMP_GD_FRMS = crate::Reg<rxicmp_gd_frms::RXICMP_GD_FRMS_SPEC>;
#[doc = "Number of good IP datagrams with a good ICMP payload"]
pub mod rxicmp_gd_frms;
#[doc = "RXICMP_ERR_FRMS (rw) register accessor: an alias for `Reg<RXICMP_ERR_FRMS_SPEC>`"]
pub type RXICMP_ERR_FRMS = crate::Reg<rxicmp_err_frms::RXICMP_ERR_FRMS_SPEC>;
#[doc = "Number of good IP datagrams whose ICMP payload has a checksum error"]
pub mod rxicmp_err_frms;
#[doc = "RXIPV4_GD_OCTETS (rw) register accessor: an alias for `Reg<RXIPV4_GD_OCTETS_SPEC>`"]
pub type RXIPV4_GD_OCTETS = crate::Reg<rxipv4_gd_octets::RXIPV4_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in good IPv4 datagrams encapsulating TCP, UDP, or ICMP data. (Ethernet header, FCS, pad, or IP pad bytes are not included in this counter or in the octet counters listed below)."]
pub mod rxipv4_gd_octets;
#[doc = "RXIPV4_HDRERR_OCTETS (rw) register accessor: an alias for `Reg<RXIPV4_HDRERR_OCTETS_SPEC>`"]
pub type RXIPV4_HDRERR_OCTETS = crate::Reg<rxipv4_hdrerr_octets::RXIPV4_HDRERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in IPv4 datagrams with header errors (checksum, length, version mismatch). The value in the Length field of IPv4 header is used to update this counter."]
pub mod rxipv4_hdrerr_octets;
#[doc = "RXIPV4_NOPAY_OCTETS (rw) register accessor: an alias for `Reg<RXIPV4_NOPAY_OCTETS_SPEC>`"]
pub type RXIPV4_NOPAY_OCTETS = crate::Reg<rxipv4_nopay_octets::RXIPV4_NOPAY_OCTETS_SPEC>;
#[doc = "Number of bytes received in IPv4 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv4 header’s Length field is used to update this counter."]
pub mod rxipv4_nopay_octets;
#[doc = "RXIPV4_FRAG_OCTETS (rw) register accessor: an alias for `Reg<RXIPV4_FRAG_OCTETS_SPEC>`"]
pub type RXIPV4_FRAG_OCTETS = crate::Reg<rxipv4_frag_octets::RXIPV4_FRAG_OCTETS_SPEC>;
#[doc = "Number of bytes received in fragmented IPv4 datagrams. The value in the IPv4 header’s Length field is used to update this counter"]
pub mod rxipv4_frag_octets;
#[doc = "RXIPV4_UDSBL_OCTETS (rw) register accessor: an alias for `Reg<RXIPV4_UDSBL_OCTETS_SPEC>`"]
pub type RXIPV4_UDSBL_OCTETS = crate::Reg<rxipv4_udsbl_octets::RXIPV4_UDSBL_OCTETS_SPEC>;
#[doc = "Number of bytes received in a UDP segment that had the UDP checksum disabled. This counter does not count IP Header bytes."]
pub mod rxipv4_udsbl_octets;
#[doc = "RXIPV6_GD_OCTETS (rw) register accessor: an alias for `Reg<RXIPV6_GD_OCTETS_SPEC>`"]
pub type RXIPV6_GD_OCTETS = crate::Reg<rxipv6_gd_octets::RXIPV6_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in good IPv6 datagrams encapsulating TCP, UDP or ICMPv6 data"]
pub mod rxipv6_gd_octets;
#[doc = "RXIPV6_HDRERR_OCTETS (rw) register accessor: an alias for `Reg<RXIPV6_HDRERR_OCTETS_SPEC>`"]
pub type RXIPV6_HDRERR_OCTETS = crate::Reg<rxipv6_hdrerr_octets::RXIPV6_HDRERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header’s Length field is used to update this counter."]
pub mod rxipv6_hdrerr_octets;
#[doc = "RXIPV6_NOPAY_OCTETS (rw) register accessor: an alias for `Reg<RXIPV6_NOPAY_OCTETS_SPEC>`"]
pub type RXIPV6_NOPAY_OCTETS = crate::Reg<rxipv6_nopay_octets::RXIPV6_NOPAY_OCTETS_SPEC>;
#[doc = "Number of bytes received in IPv6 datagrams that did not have a TCP, UDP, or ICMP payload. The value in the IPv6 header’s Length field is used to update this counter."]
pub mod rxipv6_nopay_octets;
#[doc = "RXUDP_GD_OCTETS (rw) register accessor: an alias for `Reg<RXUDP_GD_OCTETS_SPEC>`"]
pub type RXUDP_GD_OCTETS = crate::Reg<rxudp_gd_octets::RXUDP_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in a good UDP segment. This counter (and the counters below) does not count IP header bytes."]
pub mod rxudp_gd_octets;
#[doc = "RXUDP_ERR_OCTETS (rw) register accessor: an alias for `Reg<RXUDP_ERR_OCTETS_SPEC>`"]
pub type RXUDP_ERR_OCTETS = crate::Reg<rxudp_err_octets::RXUDP_ERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in a UDP segment that had checksum errors"]
pub mod rxudp_err_octets;
#[doc = "RXTCP_GD_OCTETS (rw) register accessor: an alias for `Reg<RXTCP_GD_OCTETS_SPEC>`"]
pub type RXTCP_GD_OCTETS = crate::Reg<rxtcp_gd_octets::RXTCP_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in a good TCP segment"]
pub mod rxtcp_gd_octets;
#[doc = "RXTCP_ERR_OCTETS (rw) register accessor: an alias for `Reg<RXTCP_ERR_OCTETS_SPEC>`"]
pub type RXTCP_ERR_OCTETS = crate::Reg<rxtcp_err_octets::RXTCP_ERR_OCTETS_SPEC>;
#[doc = "Number of bytes received in a TCP segment with checksum errors"]
pub mod rxtcp_err_octets;
#[doc = "RXICMP_GD_OCTETS (rw) register accessor: an alias for `Reg<RXICMP_GD_OCTETS_SPEC>`"]
pub type RXICMP_GD_OCTETS = crate::Reg<rxicmp_gd_octets::RXICMP_GD_OCTETS_SPEC>;
#[doc = "Number of bytes received in a good ICMP segment"]
pub mod rxicmp_gd_octets;
#[doc = "L3_L4_CFG_0_L3_L4_CTRL (rw) register accessor: an alias for `Reg<L3_L4_CFG_0_L3_L4_CTRL_SPEC>`"]
pub type L3_L4_CFG_0_L3_L4_CTRL = crate::Reg<l3_l4_cfg_0_l3_l4_ctrl::L3_L4_CFG_0_L3_L4_CTRL_SPEC>;
#[doc = "Layer 3 and Layer 4 Control Register"]
pub mod l3_l4_cfg_0_l3_l4_ctrl;
#[doc = "L3_L4_CFG_0_L4_ADDR (rw) register accessor: an alias for `Reg<L3_L4_CFG_0_L4_ADDR_SPEC>`"]
pub type L3_L4_CFG_0_L4_ADDR = crate::Reg<l3_l4_cfg_0_l4_addr::L3_L4_CFG_0_L4_ADDR_SPEC>;
#[doc = "Layer 4 Address Register"]
pub mod l3_l4_cfg_0_l4_addr;
#[doc = "L3_L4_CFG_0_L3_ADDR_0 (rw) register accessor: an alias for `Reg<L3_L4_CFG_0_L3_ADDR_0_SPEC>`"]
pub type L3_L4_CFG_0_L3_ADDR_0 = crate::Reg<l3_l4_cfg_0_l3_addr_0::L3_L4_CFG_0_L3_ADDR_0_SPEC>;
#[doc = "Layer 3 Address 0 Register"]
pub mod l3_l4_cfg_0_l3_addr_0;
#[doc = "L3_L4_CFG_0_L3_ADDR_1 (rw) register accessor: an alias for `Reg<L3_L4_CFG_0_L3_ADDR_1_SPEC>`"]
pub type L3_L4_CFG_0_L3_ADDR_1 = crate::Reg<l3_l4_cfg_0_l3_addr_1::L3_L4_CFG_0_L3_ADDR_1_SPEC>;
#[doc = "Layer 3 Address 1 Register"]
pub mod l3_l4_cfg_0_l3_addr_1;
#[doc = "L3_L4_CFG_0_L3_ADDR_2 (rw) register accessor: an alias for `Reg<L3_L4_CFG_0_L3_ADDR_2_SPEC>`"]
pub type L3_L4_CFG_0_L3_ADDR_2 = crate::Reg<l3_l4_cfg_0_l3_addr_2::L3_L4_CFG_0_L3_ADDR_2_SPEC>;
#[doc = "Layer 3 Address 2 Register"]
pub mod l3_l4_cfg_0_l3_addr_2;
#[doc = "L3_L4_CFG_0_L3_ADDR_3 (rw) register accessor: an alias for `Reg<L3_L4_CFG_0_L3_ADDR_3_SPEC>`"]
pub type L3_L4_CFG_0_L3_ADDR_3 = crate::Reg<l3_l4_cfg_0_l3_addr_3::L3_L4_CFG_0_L3_ADDR_3_SPEC>;
#[doc = "Layer 3 Address 3 Register"]
pub mod l3_l4_cfg_0_l3_addr_3;
#[doc = "L3_L4_CFG_1_L3_L4_CTRL (rw) register accessor: an alias for `Reg<L3_L4_CFG_1_L3_L4_CTRL_SPEC>`"]
pub type L3_L4_CFG_1_L3_L4_CTRL = crate::Reg<l3_l4_cfg_1_l3_l4_ctrl::L3_L4_CFG_1_L3_L4_CTRL_SPEC>;
#[doc = "Layer 3 and Layer 4 Control Register"]
pub mod l3_l4_cfg_1_l3_l4_ctrl;
#[doc = "L3_L4_CFG_1_L4_ADDR (rw) register accessor: an alias for `Reg<L3_L4_CFG_1_L4_ADDR_SPEC>`"]
pub type L3_L4_CFG_1_L4_ADDR = crate::Reg<l3_l4_cfg_1_l4_addr::L3_L4_CFG_1_L4_ADDR_SPEC>;
#[doc = "Layer 4 Address Register"]
pub mod l3_l4_cfg_1_l4_addr;
#[doc = "L3_L4_CFG_1_L3_ADDR_0 (rw) register accessor: an alias for `Reg<L3_L4_CFG_1_L3_ADDR_0_SPEC>`"]
pub type L3_L4_CFG_1_L3_ADDR_0 = crate::Reg<l3_l4_cfg_1_l3_addr_0::L3_L4_CFG_1_L3_ADDR_0_SPEC>;
#[doc = "Layer 3 Address 0 Register"]
pub mod l3_l4_cfg_1_l3_addr_0;
#[doc = "L3_L4_CFG_1_L3_ADDR_1 (rw) register accessor: an alias for `Reg<L3_L4_CFG_1_L3_ADDR_1_SPEC>`"]
pub type L3_L4_CFG_1_L3_ADDR_1 = crate::Reg<l3_l4_cfg_1_l3_addr_1::L3_L4_CFG_1_L3_ADDR_1_SPEC>;
#[doc = "Layer 3 Address 1 Register"]
pub mod l3_l4_cfg_1_l3_addr_1;
#[doc = "L3_L4_CFG_1_L3_ADDR_2 (rw) register accessor: an alias for `Reg<L3_L4_CFG_1_L3_ADDR_2_SPEC>`"]
pub type L3_L4_CFG_1_L3_ADDR_2 = crate::Reg<l3_l4_cfg_1_l3_addr_2::L3_L4_CFG_1_L3_ADDR_2_SPEC>;
#[doc = "Layer 3 Address 2 Register"]
pub mod l3_l4_cfg_1_l3_addr_2;
#[doc = "L3_L4_CFG_1_L3_ADDR_3 (rw) register accessor: an alias for `Reg<L3_L4_CFG_1_L3_ADDR_3_SPEC>`"]
pub type L3_L4_CFG_1_L3_ADDR_3 = crate::Reg<l3_l4_cfg_1_l3_addr_3::L3_L4_CFG_1_L3_ADDR_3_SPEC>;
#[doc = "Layer 3 Address 3 Register"]
pub mod l3_l4_cfg_1_l3_addr_3;
#[doc = "L3_L4_CFG_2_L3_L4_CTRL (rw) register accessor: an alias for `Reg<L3_L4_CFG_2_L3_L4_CTRL_SPEC>`"]
pub type L3_L4_CFG_2_L3_L4_CTRL = crate::Reg<l3_l4_cfg_2_l3_l4_ctrl::L3_L4_CFG_2_L3_L4_CTRL_SPEC>;
#[doc = "Layer 3 and Layer 4 Control Register"]
pub mod l3_l4_cfg_2_l3_l4_ctrl;
#[doc = "L3_L4_CFG_2_L4_ADDR (rw) register accessor: an alias for `Reg<L3_L4_CFG_2_L4_ADDR_SPEC>`"]
pub type L3_L4_CFG_2_L4_ADDR = crate::Reg<l3_l4_cfg_2_l4_addr::L3_L4_CFG_2_L4_ADDR_SPEC>;
#[doc = "Layer 4 Address Register"]
pub mod l3_l4_cfg_2_l4_addr;
#[doc = "L3_L4_CFG_2_L3_ADDR_0 (rw) register accessor: an alias for `Reg<L3_L4_CFG_2_L3_ADDR_0_SPEC>`"]
pub type L3_L4_CFG_2_L3_ADDR_0 = crate::Reg<l3_l4_cfg_2_l3_addr_0::L3_L4_CFG_2_L3_ADDR_0_SPEC>;
#[doc = "Layer 3 Address 0 Register"]
pub mod l3_l4_cfg_2_l3_addr_0;
#[doc = "L3_L4_CFG_2_L3_ADDR_1 (rw) register accessor: an alias for `Reg<L3_L4_CFG_2_L3_ADDR_1_SPEC>`"]
pub type L3_L4_CFG_2_L3_ADDR_1 = crate::Reg<l3_l4_cfg_2_l3_addr_1::L3_L4_CFG_2_L3_ADDR_1_SPEC>;
#[doc = "Layer 3 Address 1 Register"]
pub mod l3_l4_cfg_2_l3_addr_1;
#[doc = "L3_L4_CFG_2_L3_ADDR_2 (rw) register accessor: an alias for `Reg<L3_L4_CFG_2_L3_ADDR_2_SPEC>`"]
pub type L3_L4_CFG_2_L3_ADDR_2 = crate::Reg<l3_l4_cfg_2_l3_addr_2::L3_L4_CFG_2_L3_ADDR_2_SPEC>;
#[doc = "Layer 3 Address 2 Register"]
pub mod l3_l4_cfg_2_l3_addr_2;
#[doc = "L3_L4_CFG_2_L3_ADDR_3 (rw) register accessor: an alias for `Reg<L3_L4_CFG_2_L3_ADDR_3_SPEC>`"]
pub type L3_L4_CFG_2_L3_ADDR_3 = crate::Reg<l3_l4_cfg_2_l3_addr_3::L3_L4_CFG_2_L3_ADDR_3_SPEC>;
#[doc = "Layer 3 Address 3 Register"]
pub mod l3_l4_cfg_2_l3_addr_3;
#[doc = "L3_L4_CFG_3_L3_L4_CTRL (rw) register accessor: an alias for `Reg<L3_L4_CFG_3_L3_L4_CTRL_SPEC>`"]
pub type L3_L4_CFG_3_L3_L4_CTRL = crate::Reg<l3_l4_cfg_3_l3_l4_ctrl::L3_L4_CFG_3_L3_L4_CTRL_SPEC>;
#[doc = "Layer 3 and Layer 4 Control Register"]
pub mod l3_l4_cfg_3_l3_l4_ctrl;
#[doc = "L3_L4_CFG_3_L4_ADDR (rw) register accessor: an alias for `Reg<L3_L4_CFG_3_L4_ADDR_SPEC>`"]
pub type L3_L4_CFG_3_L4_ADDR = crate::Reg<l3_l4_cfg_3_l4_addr::L3_L4_CFG_3_L4_ADDR_SPEC>;
#[doc = "Layer 4 Address Register"]
pub mod l3_l4_cfg_3_l4_addr;
#[doc = "L3_L4_CFG_3_L3_ADDR_0 (rw) register accessor: an alias for `Reg<L3_L4_CFG_3_L3_ADDR_0_SPEC>`"]
pub type L3_L4_CFG_3_L3_ADDR_0 = crate::Reg<l3_l4_cfg_3_l3_addr_0::L3_L4_CFG_3_L3_ADDR_0_SPEC>;
#[doc = "Layer 3 Address 0 Register"]
pub mod l3_l4_cfg_3_l3_addr_0;
#[doc = "L3_L4_CFG_3_L3_ADDR_1 (rw) register accessor: an alias for `Reg<L3_L4_CFG_3_L3_ADDR_1_SPEC>`"]
pub type L3_L4_CFG_3_L3_ADDR_1 = crate::Reg<l3_l4_cfg_3_l3_addr_1::L3_L4_CFG_3_L3_ADDR_1_SPEC>;
#[doc = "Layer 3 Address 1 Register"]
pub mod l3_l4_cfg_3_l3_addr_1;
#[doc = "L3_L4_CFG_3_L3_ADDR_2 (rw) register accessor: an alias for `Reg<L3_L4_CFG_3_L3_ADDR_2_SPEC>`"]
pub type L3_L4_CFG_3_L3_ADDR_2 = crate::Reg<l3_l4_cfg_3_l3_addr_2::L3_L4_CFG_3_L3_ADDR_2_SPEC>;
#[doc = "Layer 3 Address 2 Register"]
pub mod l3_l4_cfg_3_l3_addr_2;
#[doc = "L3_L4_CFG_3_L3_ADDR_3 (rw) register accessor: an alias for `Reg<L3_L4_CFG_3_L3_ADDR_3_SPEC>`"]
pub type L3_L4_CFG_3_L3_ADDR_3 = crate::Reg<l3_l4_cfg_3_l3_addr_3::L3_L4_CFG_3_L3_ADDR_3_SPEC>;
#[doc = "Layer 3 Address 3 Register"]
pub mod l3_l4_cfg_3_l3_addr_3;
#[doc = "HASH_TABLE_REGISTER0 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER0_SPEC>`"]
pub type HASH_TABLE_REGISTER0 = crate::Reg<hash_table_register0::HASH_TABLE_REGISTER0_SPEC>;
#[doc = "Hash Table Register 0"]
pub mod hash_table_register0;
#[doc = "HASH_TABLE_REGISTER1 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER1_SPEC>`"]
pub type HASH_TABLE_REGISTER1 = crate::Reg<hash_table_register1::HASH_TABLE_REGISTER1_SPEC>;
#[doc = "Hash Table Register 1"]
pub mod hash_table_register1;
#[doc = "HASH_TABLE_REGISTER2 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER2_SPEC>`"]
pub type HASH_TABLE_REGISTER2 = crate::Reg<hash_table_register2::HASH_TABLE_REGISTER2_SPEC>;
#[doc = "Hash Table Register 2"]
pub mod hash_table_register2;
#[doc = "HASH_TABLE_REGISTER3 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER3_SPEC>`"]
pub type HASH_TABLE_REGISTER3 = crate::Reg<hash_table_register3::HASH_TABLE_REGISTER3_SPEC>;
#[doc = "Hash Table Register 3"]
pub mod hash_table_register3;
#[doc = "HASH_TABLE_REGISTER4 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER4_SPEC>`"]
pub type HASH_TABLE_REGISTER4 = crate::Reg<hash_table_register4::HASH_TABLE_REGISTER4_SPEC>;
#[doc = "Hash Table Register 4"]
pub mod hash_table_register4;
#[doc = "HASH_TABLE_REGISTER5 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER5_SPEC>`"]
pub type HASH_TABLE_REGISTER5 = crate::Reg<hash_table_register5::HASH_TABLE_REGISTER5_SPEC>;
#[doc = "Hash Table Register 5"]
pub mod hash_table_register5;
#[doc = "HASH_TABLE_REGISTER6 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER6_SPEC>`"]
pub type HASH_TABLE_REGISTER6 = crate::Reg<hash_table_register6::HASH_TABLE_REGISTER6_SPEC>;
#[doc = "Hash Table Register 6"]
pub mod hash_table_register6;
#[doc = "HASH_TABLE_REGISTER7 (rw) register accessor: an alias for `Reg<HASH_TABLE_REGISTER7_SPEC>`"]
pub type HASH_TABLE_REGISTER7 = crate::Reg<hash_table_register7::HASH_TABLE_REGISTER7_SPEC>;
#[doc = "Hash Table Register 7"]
pub mod hash_table_register7;
#[doc = "VLAN_TAG_INC_RPL (rw) register accessor: an alias for `Reg<VLAN_TAG_INC_RPL_SPEC>`"]
pub type VLAN_TAG_INC_RPL = crate::Reg<vlan_tag_inc_rpl::VLAN_TAG_INC_RPL_SPEC>;
#[doc = "VLAN Tag Inclusion or Replacement Register"]
pub mod vlan_tag_inc_rpl;
#[doc = "VLAN_HASH (rw) register accessor: an alias for `Reg<VLAN_HASH_SPEC>`"]
pub type VLAN_HASH = crate::Reg<vlan_hash::VLAN_HASH_SPEC>;
#[doc = "VLAN Hash Table Register"]
pub mod vlan_hash;
#[doc = "TS_CTRL (rw) register accessor: an alias for `Reg<TS_CTRL_SPEC>`"]
pub type TS_CTRL = crate::Reg<ts_ctrl::TS_CTRL_SPEC>;
#[doc = "Timestamp Control Register"]
pub mod ts_ctrl;
#[doc = "SUB_SEC_INCR (rw) register accessor: an alias for `Reg<SUB_SEC_INCR_SPEC>`"]
pub type SUB_SEC_INCR = crate::Reg<sub_sec_incr::SUB_SEC_INCR_SPEC>;
#[doc = "Sub-Second Increment Register"]
pub mod sub_sec_incr;
#[doc = "SYST_SEC (rw) register accessor: an alias for `Reg<SYST_SEC_SPEC>`"]
pub type SYST_SEC = crate::Reg<syst_sec::SYST_SEC_SPEC>;
#[doc = "System Time - Seconds Register"]
pub mod syst_sec;
#[doc = "SYST_NSEC (rw) register accessor: an alias for `Reg<SYST_NSEC_SPEC>`"]
pub type SYST_NSEC = crate::Reg<syst_nsec::SYST_NSEC_SPEC>;
#[doc = "System Time - Nanoseconds Register"]
pub mod syst_nsec;
#[doc = "SYST_SEC_UPD (rw) register accessor: an alias for `Reg<SYST_SEC_UPD_SPEC>`"]
pub type SYST_SEC_UPD = crate::Reg<syst_sec_upd::SYST_SEC_UPD_SPEC>;
#[doc = "System Time - Seconds Update Register"]
pub mod syst_sec_upd;
#[doc = "SYST_NSEC_UPD (rw) register accessor: an alias for `Reg<SYST_NSEC_UPD_SPEC>`"]
pub type SYST_NSEC_UPD = crate::Reg<syst_nsec_upd::SYST_NSEC_UPD_SPEC>;
#[doc = "System Time - Nanoseconds Update Register"]
pub mod syst_nsec_upd;
#[doc = "TS_ADDEND (rw) register accessor: an alias for `Reg<TS_ADDEND_SPEC>`"]
pub type TS_ADDEND = crate::Reg<ts_addend::TS_ADDEND_SPEC>;
#[doc = "Timestamp Addend Register"]
pub mod ts_addend;
#[doc = "TGTTM_SEC (rw) register accessor: an alias for `Reg<TGTTM_SEC_SPEC>`"]
pub type TGTTM_SEC = crate::Reg<tgttm_sec::TGTTM_SEC_SPEC>;
#[doc = "Target Time Seconds Register"]
pub mod tgttm_sec;
#[doc = "TGTTM_NSEC (rw) register accessor: an alias for `Reg<TGTTM_NSEC_SPEC>`"]
pub type TGTTM_NSEC = crate::Reg<tgttm_nsec::TGTTM_NSEC_SPEC>;
#[doc = "Target Time Nanoseconds Register"]
pub mod tgttm_nsec;
#[doc = "SYSTM_H_SEC (rw) register accessor: an alias for `Reg<SYSTM_H_SEC_SPEC>`"]
pub type SYSTM_H_SEC = crate::Reg<systm_h_sec::SYSTM_H_SEC_SPEC>;
#[doc = "System Time - Higher Word Seconds Register"]
pub mod systm_h_sec;
#[doc = "TS_STATUS (r) register accessor: an alias for `Reg<TS_STATUS_SPEC>`"]
pub type TS_STATUS = crate::Reg<ts_status::TS_STATUS_SPEC>;
#[doc = "Timestamp Status Register"]
pub mod ts_status;
#[doc = "PPS_CTRL (rw) register accessor: an alias for `Reg<PPS_CTRL_SPEC>`"]
pub type PPS_CTRL = crate::Reg<pps_ctrl::PPS_CTRL_SPEC>;
#[doc = "PPS Control Register"]
pub mod pps_ctrl;
#[doc = "AUX_TS_NSEC (rw) register accessor: an alias for `Reg<AUX_TS_NSEC_SPEC>`"]
pub type AUX_TS_NSEC = crate::Reg<aux_ts_nsec::AUX_TS_NSEC_SPEC>;
#[doc = "Auxiliary Timestamp - Nanoseconds Register"]
pub mod aux_ts_nsec;
#[doc = "AUX_TS_SEC (rw) register accessor: an alias for `Reg<AUX_TS_SEC_SPEC>`"]
pub type AUX_TS_SEC = crate::Reg<aux_ts_sec::AUX_TS_SEC_SPEC>;
#[doc = "Auxiliary Timestamp - Seconds Register"]
pub mod aux_ts_sec;
#[doc = "PPS0_INTERVAL (rw) register accessor: an alias for `Reg<PPS0_INTERVAL_SPEC>`"]
pub type PPS0_INTERVAL = crate::Reg<pps0_interval::PPS0_INTERVAL_SPEC>;
#[doc = "PPS0 Interval Register"]
pub mod pps0_interval;
#[doc = "PPS0_WIDTH (rw) register accessor: an alias for `Reg<PPS0_WIDTH_SPEC>`"]
pub type PPS0_WIDTH = crate::Reg<pps0_width::PPS0_WIDTH_SPEC>;
#[doc = "PPS0 Width Register"]
pub mod pps0_width;
#[doc = "PPS_1_TGTTM_SEC (rw) register accessor: an alias for `Reg<PPS_1_TGTTM_SEC_SPEC>`"]
pub type PPS_1_TGTTM_SEC = crate::Reg<pps_1_tgttm_sec::PPS_1_TGTTM_SEC_SPEC>;
#[doc = "PPS Target Time Seconds Register"]
pub mod pps_1_tgttm_sec;
#[doc = "PPS_1_TGTTM_NSEC (rw) register accessor: an alias for `Reg<PPS_1_TGTTM_NSEC_SPEC>`"]
pub type PPS_1_TGTTM_NSEC = crate::Reg<pps_1_tgttm_nsec::PPS_1_TGTTM_NSEC_SPEC>;
#[doc = "PPS Target Time Nanoseconds Register"]
pub mod pps_1_tgttm_nsec;
#[doc = "PPS_1_INTERVAL (rw) register accessor: an alias for `Reg<PPS_1_INTERVAL_SPEC>`"]
pub type PPS_1_INTERVAL = crate::Reg<pps_1_interval::PPS_1_INTERVAL_SPEC>;
#[doc = "PPS Interval Register"]
pub mod pps_1_interval;
#[doc = "PPS_1_WIDTH (rw) register accessor: an alias for `Reg<PPS_1_WIDTH_SPEC>`"]
pub type PPS_1_WIDTH = crate::Reg<pps_1_width::PPS_1_WIDTH_SPEC>;
#[doc = "PPS Width Register"]
pub mod pps_1_width;
#[doc = "PPS_2_TGTTM_SEC (rw) register accessor: an alias for `Reg<PPS_2_TGTTM_SEC_SPEC>`"]
pub type PPS_2_TGTTM_SEC = crate::Reg<pps_2_tgttm_sec::PPS_2_TGTTM_SEC_SPEC>;
#[doc = "PPS2 Target Time Seconds Register"]
pub mod pps_2_tgttm_sec;
#[doc = "PPS_2_TGTTM_NSEC (rw) register accessor: an alias for `Reg<PPS_2_TGTTM_NSEC_SPEC>`"]
pub type PPS_2_TGTTM_NSEC = crate::Reg<pps_2_tgttm_nsec::PPS_2_TGTTM_NSEC_SPEC>;
#[doc = "PPS Target Time Nanoseconds Register"]
pub mod pps_2_tgttm_nsec;
#[doc = "PPS_2_INTERVAL (rw) register accessor: an alias for `Reg<PPS_2_INTERVAL_SPEC>`"]
pub type PPS_2_INTERVAL = crate::Reg<pps_2_interval::PPS_2_INTERVAL_SPEC>;
#[doc = "PPS Interval Register"]
pub mod pps_2_interval;
#[doc = "PPS_2_WIDTH (rw) register accessor: an alias for `Reg<PPS_2_WIDTH_SPEC>`"]
pub type PPS_2_WIDTH = crate::Reg<pps_2_width::PPS_2_WIDTH_SPEC>;
#[doc = "PPS Width Register"]
pub mod pps_2_width;
#[doc = "PPS_3_TGTTM_SEC (rw) register accessor: an alias for `Reg<PPS_3_TGTTM_SEC_SPEC>`"]
pub type PPS_3_TGTTM_SEC = crate::Reg<pps_3_tgttm_sec::PPS_3_TGTTM_SEC_SPEC>;
#[doc = "PPS3 Target Time Seconds Register"]
pub mod pps_3_tgttm_sec;
#[doc = "PPS_3_TGTTM_NSEC (rw) register accessor: an alias for `Reg<PPS_3_TGTTM_NSEC_SPEC>`"]
pub type PPS_3_TGTTM_NSEC = crate::Reg<pps_3_tgttm_nsec::PPS_3_TGTTM_NSEC_SPEC>;
#[doc = "PPS Target Time Nanoseconds Register"]
pub mod pps_3_tgttm_nsec;
#[doc = "PPS_3_INTERVAL (rw) register accessor: an alias for `Reg<PPS_3_INTERVAL_SPEC>`"]
pub type PPS_3_INTERVAL = crate::Reg<pps_3_interval::PPS_3_INTERVAL_SPEC>;
#[doc = "PPS Interval Register"]
pub mod pps_3_interval;
#[doc = "PPS_3_WIDTH (rw) register accessor: an alias for `Reg<PPS_3_WIDTH_SPEC>`"]
pub type PPS_3_WIDTH = crate::Reg<pps_3_width::PPS_3_WIDTH_SPEC>;
#[doc = "PPS Width Register"]
pub mod pps_3_width;
#[doc = "DMA_BUS_MODE (rw) register accessor: an alias for `Reg<DMA_BUS_MODE_SPEC>`"]
pub type DMA_BUS_MODE = crate::Reg<dma_bus_mode::DMA_BUS_MODE_SPEC>;
#[doc = "Bus Mode Register"]
pub mod dma_bus_mode;
#[doc = "DMA_TX_POLL_DEMAND (rw) register accessor: an alias for `Reg<DMA_TX_POLL_DEMAND_SPEC>`"]
pub type DMA_TX_POLL_DEMAND = crate::Reg<dma_tx_poll_demand::DMA_TX_POLL_DEMAND_SPEC>;
#[doc = "Transmit Poll Demand Register"]
pub mod dma_tx_poll_demand;
#[doc = "DMA_RX_POLL_DEMAND (rw) register accessor: an alias for `Reg<DMA_RX_POLL_DEMAND_SPEC>`"]
pub type DMA_RX_POLL_DEMAND = crate::Reg<dma_rx_poll_demand::DMA_RX_POLL_DEMAND_SPEC>;
#[doc = "Receive Poll Demand Register"]
pub mod dma_rx_poll_demand;
#[doc = "DMA_RX_DESC_LIST_ADDR (rw) register accessor: an alias for `Reg<DMA_RX_DESC_LIST_ADDR_SPEC>`"]
pub type DMA_RX_DESC_LIST_ADDR = crate::Reg<dma_rx_desc_list_addr::DMA_RX_DESC_LIST_ADDR_SPEC>;
#[doc = "Receive Descriptor List Address Register"]
pub mod dma_rx_desc_list_addr;
#[doc = "DMA_TX_DESC_LIST_ADDR (rw) register accessor: an alias for `Reg<DMA_TX_DESC_LIST_ADDR_SPEC>`"]
pub type DMA_TX_DESC_LIST_ADDR = crate::Reg<dma_tx_desc_list_addr::DMA_TX_DESC_LIST_ADDR_SPEC>;
#[doc = "Transmit Descriptor List Address Register"]
pub mod dma_tx_desc_list_addr;
#[doc = "DMA_STATUS (rw) register accessor: an alias for `Reg<DMA_STATUS_SPEC>`"]
pub type DMA_STATUS = crate::Reg<dma_status::DMA_STATUS_SPEC>;
#[doc = "Status Register"]
pub mod dma_status;
#[doc = "DMA_OP_MODE (rw) register accessor: an alias for `Reg<DMA_OP_MODE_SPEC>`"]
pub type DMA_OP_MODE = crate::Reg<dma_op_mode::DMA_OP_MODE_SPEC>;
#[doc = "Operation Mode Register"]
pub mod dma_op_mode;
#[doc = "DMA_INTR_EN (rw) register accessor: an alias for `Reg<DMA_INTR_EN_SPEC>`"]
pub type DMA_INTR_EN = crate::Reg<dma_intr_en::DMA_INTR_EN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod dma_intr_en;
#[doc = "DMA_MISS_OVF_CNT (rw) register accessor: an alias for `Reg<DMA_MISS_OVF_CNT_SPEC>`"]
pub type DMA_MISS_OVF_CNT = crate::Reg<dma_miss_ovf_cnt::DMA_MISS_OVF_CNT_SPEC>;
#[doc = "Missed Frame And Buffer Overflow Counter Register"]
pub mod dma_miss_ovf_cnt;
#[doc = "DMA_RX_INTR_WDOG (rw) register accessor: an alias for `Reg<DMA_RX_INTR_WDOG_SPEC>`"]
pub type DMA_RX_INTR_WDOG = crate::Reg<dma_rx_intr_wdog::DMA_RX_INTR_WDOG_SPEC>;
#[doc = "Receive Interrupt Watchdog Timer Register"]
pub mod dma_rx_intr_wdog;
#[doc = "DMA_AXI_MODE (rw) register accessor: an alias for `Reg<DMA_AXI_MODE_SPEC>`"]
pub type DMA_AXI_MODE = crate::Reg<dma_axi_mode::DMA_AXI_MODE_SPEC>;
#[doc = "AXI Bus Mode Register"]
pub mod dma_axi_mode;
#[doc = "DMA_BUS_STATUS (rw) register accessor: an alias for `Reg<DMA_BUS_STATUS_SPEC>`"]
pub type DMA_BUS_STATUS = crate::Reg<dma_bus_status::DMA_BUS_STATUS_SPEC>;
#[doc = "AHB or AXI Status Register"]
pub mod dma_bus_status;
#[doc = "DMA_CURR_HOST_TX_DESC (rw) register accessor: an alias for `Reg<DMA_CURR_HOST_TX_DESC_SPEC>`"]
pub type DMA_CURR_HOST_TX_DESC = crate::Reg<dma_curr_host_tx_desc::DMA_CURR_HOST_TX_DESC_SPEC>;
#[doc = "Current Host Transmit Descriptor Register"]
pub mod dma_curr_host_tx_desc;
#[doc = "DMA_CURR_HOST_RX_DESC (rw) register accessor: an alias for `Reg<DMA_CURR_HOST_RX_DESC_SPEC>`"]
pub type DMA_CURR_HOST_RX_DESC = crate::Reg<dma_curr_host_rx_desc::DMA_CURR_HOST_RX_DESC_SPEC>;
#[doc = "Current Host Receive Descriptor Register"]
pub mod dma_curr_host_rx_desc;
#[doc = "DMA_CURR_HOST_TX_BUF (rw) register accessor: an alias for `Reg<DMA_CURR_HOST_TX_BUF_SPEC>`"]
pub type DMA_CURR_HOST_TX_BUF = crate::Reg<dma_curr_host_tx_buf::DMA_CURR_HOST_TX_BUF_SPEC>;
#[doc = "Current Host Transmit Buffer Address Register"]
pub mod dma_curr_host_tx_buf;
#[doc = "DMA_CURR_HOST_RX_BUF (rw) register accessor: an alias for `Reg<DMA_CURR_HOST_RX_BUF_SPEC>`"]
pub type DMA_CURR_HOST_RX_BUF = crate::Reg<dma_curr_host_rx_buf::DMA_CURR_HOST_RX_BUF_SPEC>;
#[doc = "Current Host Receive Buffer Address Register"]
pub mod dma_curr_host_rx_buf;
#[doc = "DMA_HW_FEATURE (rw) register accessor: an alias for `Reg<DMA_HW_FEATURE_SPEC>`"]
pub type DMA_HW_FEATURE = crate::Reg<dma_hw_feature::DMA_HW_FEATURE_SPEC>;
#[doc = "HW Feature Register"]
pub mod dma_hw_feature;
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control Register 0"]
pub mod ctrl0;
#[doc = "CTRL2 (rw) register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control Register 1"]
pub mod ctrl2;
