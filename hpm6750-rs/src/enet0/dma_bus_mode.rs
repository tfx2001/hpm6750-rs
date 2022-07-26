#[doc = "Register `DMA_BUS_MODE` reader"]
pub struct R(crate::R<DMA_BUS_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_BUS_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_BUS_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_BUS_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_BUS_MODE` writer"]
pub struct W(crate::W<DMA_BUS_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_BUS_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_BUS_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_BUS_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIB` reader - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT (Retry, Split, or Losing bus grant), the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx. The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
pub type RIB_R = crate::BitReader<bool>;
#[doc = "Field `RIB` writer - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT (Retry, Split, or Losing bus grant), the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx. The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
pub type RIB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `PRWG` reader - Channel Priority Weights This field sets the priority weights for Channel 0 during the round-robin arbitration between the DMA channels for the system bus. - 00: The priority weight is 1. - 01: The priority weight is 2. - 10: The priority weight is 3. - 11: The priority weight is 4. This field is present in all DWC_gmac configurations except GMAC-AXI when you select the AV feature. Otherwise, this field is reserved and read-only (RO)."]
pub type PRWG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRWG` writer - Channel Priority Weights This field sets the priority weights for Channel 0 during the round-robin arbitration between the DMA channels for the system bus. - 00: The priority weight is 1. - 01: The priority weight is 2. - 10: The priority weight is 3. - 11: The priority weight is 4. This field is present in all DWC_gmac configurations except GMAC-AXI when you select the AV feature. Otherwise, this field is reserved and read-only (RO)."]
pub type PRWG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_BUS_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXPR` reader - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the system-side bus. In the GMAC-AXI configuration, this bit is reserved and read-only (RO)."]
pub type TXPR_R = crate::BitReader<bool>;
#[doc = "Field `TXPR` writer - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the system-side bus. In the GMAC-AXI configuration, this bit is reserved and read-only (RO)."]
pub type TXPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `MB` reader - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR (undefined burst), whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR (undefined burst), whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `AAL` reader - Address-Aligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits. If the FB bit is equal to 0, the first burst (accessing the start address of data buffer) is not aligned, but subsequent bursts are aligned to the address."]
pub type AAL_R = crate::BitReader<bool>;
#[doc = "Field `AAL` writer - Address-Aligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits. If the FB bit is equal to 0, the first burst (accessing the start address of data buffer) is not aligned, but subsequent bursts are aligned to the address."]
pub type AAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `PBLX8` reader - PBLx8 Mode When set high, this bit multiplies the programmed PBL value (Bits \\[22:17\\]
and Bits\\[13:8\\]) eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
pub type PBLX8_R = crate::BitReader<bool>;
#[doc = "Field `PBLX8` writer - PBLx8 Mode When set high, this bit multiplies the programmed PBL value (Bits \\[22:17\\]
and Bits\\[13:8\\]) eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
pub type PBLX8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `USP` reader - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\]
as PBL. The PBL value in Bits \\[13:8\\]
is applicable only to the Tx DMA operations. When reset to low, the PBL value in Bits \\[13:8\\]
is applicable for both DMA engines."]
pub type USP_R = crate::BitReader<bool>;
#[doc = "Field `USP` writer - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\]
as PBL. The PBL value in Bits \\[13:8\\]
is applicable only to the Tx DMA operations. When reset to low, the PBL value in Bits \\[13:8\\]
is applicable for both DMA engines."]
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `RPBL` reader - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write. The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus. You can program RPBL with values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP is set high."]
pub type RPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPBL` writer - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write. The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus. You can program RPBL with values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP is set high."]
pub type RPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_BUS_MODE_SPEC, u8, u8, 6, O>;
#[doc = "Field `FB` reader - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not. When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers. When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations."]
pub type FB_R = crate::BitReader<bool>;
#[doc = "Field `FB` writer - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not. When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers. When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations."]
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `PR` reader - Priority Ratio These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 (TXPR) is reset or set. - 00: The Priority Ratio is 1:1. - 01: The Priority Ratio is 2:1. - 10: The Priority Ratio is 3:1. - 11: The Priority Ratio is 4:1."]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - Priority Ratio These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 (TXPR) is reset or set. - 00: The Priority Ratio is 1:1. - 01: The Priority Ratio is 2:1. - 10: The Priority Ratio is 3:1. - 11: The Priority Ratio is 4:1."]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_BUS_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `PBL` reader - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set high, this PBL value is applicable only for Tx DMA transactions. If the number of beats to be transferred is more than 32, then perform the following steps: 1. Set the PBLx8 mode. 2. Set the PBL."]
pub type PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBL` writer - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set high, this PBL value is applicable only for Tx DMA transactions. If the number of beats to be transferred is more than 32, then perform the following steps: 1. Set the PBLx8 mode. 2. Set the PBL."]
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_BUS_MODE_SPEC, u8, u8, 6, O>;
#[doc = "Field `ATDS` reader - Alternate Descriptor Size When set, the size of the alternate descriptor (described in “Alternate or Enhanced Descriptors” on page 545) increases to 32 bytes (8 DWORDS). This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine (Type 2) is enabled in the receiver. The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine (Type 2) features are not enabled. In such case, you can use the 16 bytes descriptor to save 4 bytes of memory. This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: - Advanced Timestamp feature - IPC Full Checksum Offload Engine (Type 2) feature Otherwise, this bit is reserved and is read-only. When reset, the descriptor size reverts back to 4 DWORDs (16 bytes)."]
pub type ATDS_R = crate::BitReader<bool>;
#[doc = "Field `ATDS` writer - Alternate Descriptor Size When set, the size of the alternate descriptor (described in “Alternate or Enhanced Descriptors” on page 545) increases to 32 bytes (8 DWORDS). This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine (Type 2) is enabled in the receiver. The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine (Type 2) features are not enabled. In such case, you can use the 16 bytes descriptor to save 4 bytes of memory. This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: - Advanced Timestamp feature - IPC Full Checksum Offload Engine (Type 2) feature Otherwise, this bit is reserved and is read-only. When reset, the descriptor size reverts back to 4 DWORDs (16 bytes)."]
pub type ATDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `DSL` reader - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode."]
pub type DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSL` writer - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode."]
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_BUS_MODE_SPEC, u8, u8, 5, O>;
#[doc = "Field `DA` reader - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0. - 0: Weighted round-robin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\]
(PR) and priority weights specified in Bit 27 (TXPR). - 1: Fixed priority The transmit path has priority over receive path when Bit 27 (TXPR) is set. Otherwise, receive path has priority over the transmit path."]
pub type DA_R = crate::BitReader<bool>;
#[doc = "Field `DA` writer - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0. - 0: Weighted round-robin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\]
(PR) and priority weights specified in Bit 27 (TXPR). - 1: Fixed priority The transmit path has priority over receive path when Bit 27 (TXPR) is set. Otherwise, receive path has priority over the transmit path."]
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
#[doc = "Field `SWR` reader - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains. Before reprogramming any register of the DWC_gmac, you should read a zero (0) value in this bit. Note: - The Software reset function is driven only by this bit. Bit 0 of Register 64 (Channel 1 Bus Mode Register) or Register 128 (Channel 2 Bus Mode Register) has no impact on the Software reset function. - The reset operation is completed only when all resets in all active clock domains are de-asserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for the software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
pub type SWR_R = crate::BitReader<bool>;
#[doc = "Field `SWR` writer - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains. Before reprogramming any register of the DWC_gmac, you should read a zero (0) value in this bit. Note: - The Software reset function is driven only by this bit. Bit 0 of Register 64 (Channel 1 Bus Mode Register) or Register 128 (Channel 2 Bus Mode Register) has no impact on the Software reset function. - The reset operation is completed only when all resets in all active clock domains are de-asserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for the software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT (Retry, Split, or Losing bus grant), the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx. The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
    #[inline(always)]
    pub fn rib(&self) -> RIB_R {
        RIB_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights This field sets the priority weights for Channel 0 during the round-robin arbitration between the DMA channels for the system bus. - 00: The priority weight is 1. - 01: The priority weight is 2. - 10: The priority weight is 3. - 11: The priority weight is 4. This field is present in all DWC_gmac configurations except GMAC-AXI when you select the AV feature. Otherwise, this field is reserved and read-only (RO)."]
    #[inline(always)]
    pub fn prwg(&self) -> PRWG_R {
        PRWG_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 27 - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the system-side bus. In the GMAC-AXI configuration, this bit is reserved and read-only (RO)."]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR (undefined burst), whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-Aligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits. If the FB bit is equal to 0, the first burst (accessing the start address of data buffer) is not aligned, but subsequent bursts are aligned to the address."]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - PBLx8 Mode When set high, this bit multiplies the programmed PBL value (Bits \\[22:17\\]
and Bits\\[13:8\\]) eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\]
as PBL. The PBL value in Bits \\[13:8\\]
is applicable only to the Tx DMA operations. When reset to low, the PBL value in Bits \\[13:8\\]
is applicable for both DMA engines."]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write. The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus. You can program RPBL with values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP is set high."]
    #[inline(always)]
    pub fn rpbl(&self) -> RPBL_R {
        RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not. When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers. When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Priority Ratio These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 (TXPR) is reset or set. - 00: The Priority Ratio is 1:1. - 01: The Priority Ratio is 2:1. - 10: The Priority Ratio is 3:1. - 11: The Priority Ratio is 4:1."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set high, this PBL value is applicable only for Tx DMA transactions. If the number of beats to be transferred is more than 32, then perform the following steps: 1. Set the PBLx8 mode. 2. Set the PBL."]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size When set, the size of the alternate descriptor (described in “Alternate or Enhanced Descriptors” on page 545) increases to 32 bytes (8 DWORDS). This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine (Type 2) is enabled in the receiver. The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine (Type 2) features are not enabled. In such case, you can use the 16 bytes descriptor to save 4 bytes of memory. This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: - Advanced Timestamp feature - IPC Full Checksum Offload Engine (Type 2) feature Otherwise, this bit is reserved and is read-only. When reset, the descriptor size reverts back to 4 DWORDs (16 bytes)."]
    #[inline(always)]
    pub fn atds(&self) -> ATDS_R {
        ATDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode."]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0. - 0: Weighted round-robin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\]
(PR) and priority weights specified in Bit 27 (TXPR). - 1: Fixed priority The transmit path has priority over receive path when Bit 27 (TXPR) is set. Otherwise, receive path has priority over the transmit path."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains. Before reprogramming any register of the DWC_gmac, you should read a zero (0) value in this bit. Note: - The Software reset function is driven only by this bit. Bit 0 of Register 64 (Channel 1 Bus Mode Register) or Register 128 (Channel 2 Bus Mode Register) has no impact on the Software reset function. - The reset operation is completed only when all resets in all active clock domains are de-asserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for the software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Rebuild INCRx Burst When this bit is set high and the AHB master gets an EBT (Retry, Split, or Losing bus grant), the AHB master interface rebuilds the pending beats of any burst transfer initiated with INCRx. The AHB master interface rebuilds the beats with a combination of specified bursts with INCRx and SINGLE. By default, the AHB master interface rebuilds pending beats of an EBT with an unspecified (INCR) burst."]
    #[inline(always)]
    pub fn rib(&mut self) -> RIB_W<31> {
        RIB_W::new(self)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights This field sets the priority weights for Channel 0 during the round-robin arbitration between the DMA channels for the system bus. - 00: The priority weight is 1. - 01: The priority weight is 2. - 10: The priority weight is 3. - 11: The priority weight is 4. This field is present in all DWC_gmac configurations except GMAC-AXI when you select the AV feature. Otherwise, this field is reserved and read-only (RO)."]
    #[inline(always)]
    pub fn prwg(&mut self) -> PRWG_W<28> {
        PRWG_W::new(self)
    }
    #[doc = "Bit 27 - Transmit Priority When set, this bit indicates that the transmit DMA has higher priority than the receive DMA during arbitration for the system-side bus. In the GMAC-AXI configuration, this bit is reserved and read-only (RO)."]
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<27> {
        TXPR_W::new(self)
    }
    #[doc = "Bit 26 - Mixed Burst When this bit is set high and the FB bit is low, the AHB master interface starts all bursts of length more than 16 with INCR (undefined burst), whereas it reverts to fixed burst transfers (INCRx and SINGLE) for burst length of 16 and less."]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<26> {
        MB_W::new(self)
    }
    #[doc = "Bit 25 - Address-Aligned Beats When this bit is set high and the FB bit is equal to 1, the AHB or AXI interface generates all bursts aligned to the start address LS bits. If the FB bit is equal to 0, the first burst (accessing the start address of data buffer) is not aligned, but subsequent bursts are aligned to the address."]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<25> {
        AAL_W::new(self)
    }
    #[doc = "Bit 24 - PBLx8 Mode When set high, this bit multiplies the programmed PBL value (Bits \\[22:17\\]
and Bits\\[13:8\\]) eight times. Therefore, the DMA transfers the data in 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<24> {
        PBLX8_W::new(self)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, this bit configures the Rx DMA to use the value configured in Bits \\[22:17\\]
as PBL. The PBL value in Bits \\[13:8\\]
is applicable only to the Tx DMA operations. When reset to low, the PBL value in Bits \\[13:8\\]
is applicable for both DMA engines."]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<23> {
        USP_W::new(self)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL This field indicates the maximum number of beats to be transferred in one Rx DMA transaction. This is the maximum value that is used in a single block Read or Write. The Rx DMA always attempts to burst as specified in the RPBL bit each time it starts a Burst transfer on the host bus. You can program RPBL with values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. This field is valid and applicable only when USP is set high."]
    #[inline(always)]
    pub fn rpbl(&mut self) -> RPBL_W<17> {
        RPBL_W::new(self)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AHB or AXI master interface performs fixed burst transfers or not. When set, the AHB interface uses only SINGLE, INCR4, INCR8, or INCR16 during start of the normal burst transfers. When reset, the AHB or AXI interface uses SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<16> {
        FB_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority Ratio These bits control the priority ratio in the weighted round-robin arbitration between the Rx DMA and Tx DMA. These bits are valid only when Bit 1 (DA) is reset. The priority ratio is Rx:Tx or Tx:Rx depending on whether Bit 27 (TXPR) is reset or set. - 00: The Priority Ratio is 1:1. - 01: The Priority Ratio is 2:1. - 10: The Priority Ratio is 3:1. - 11: The Priority Ratio is 4:1."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<14> {
        PR_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This is the maximum value that is used in a single block Read or Write. The DMA always attempts to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value results in undefined behavior. When USP is set high, this PBL value is applicable only for Tx DMA transactions. If the number of beats to be transferred is more than 32, then perform the following steps: 1. Set the PBLx8 mode. 2. Set the PBL."]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size When set, the size of the alternate descriptor (described in “Alternate or Enhanced Descriptors” on page 545) increases to 32 bytes (8 DWORDS). This is required when the Advanced Timestamp feature or the IPC Full Checksum Offload Engine (Type 2) is enabled in the receiver. The enhanced descriptor is not required if the Advanced Timestamp and IPC Full Checksum Offload Engine (Type 2) features are not enabled. In such case, you can use the 16 bytes descriptor to save 4 bytes of memory. This bit is present only when you select the Alternate Descriptor feature and any one of the following features during core configuration: - Advanced Timestamp feature - IPC Full Checksum Offload Engine (Type 2) feature Otherwise, this bit is reserved and is read-only. When reset, the descriptor size reverts back to 4 DWORDs (16 bytes)."]
    #[inline(always)]
    pub fn atds(&mut self) -> ATDS_W<7> {
        ATDS_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of Word, Dword, or Lword (depending on the 32-bit, 64-bit, or 128-bit bus) to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When the DSL value is equal to zero, the descriptor table is taken as contiguous by the DMA in Ring mode."]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<2> {
        DSL_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme This bit specifies the arbitration scheme between the transmit and receive paths of Channel 0. - 0: Weighted round-robin with Rx:Tx or Tx:Rx The priority between the paths is according to the priority specified in Bits \\[15:14\\]
(PR) and priority weights specified in Bit 27 (TXPR). - 1: Fixed priority The transmit path has priority over receive path when Bit 27 (TXPR) is set. Otherwise, receive path has priority over the transmit path."]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets the logic and all internal registers of the MAC. It is cleared automatically after the reset operation is complete in all of the DWC_gmac clock domains. Before reprogramming any register of the DWC_gmac, you should read a zero (0) value in this bit. Note: - The Software reset function is driven only by this bit. Bit 0 of Register 64 (Channel 1 Bus Mode Register) or Register 128 (Channel 2 Bus Mode Register) has no impact on the Software reset function. - The reset operation is completed only when all resets in all active clock domains are de-asserted. Therefore, it is essential that all PHY inputs clocks (applicable for the selected PHY interface) are present for the software reset completion. The time to complete the software reset operation depends on the frequency of the slowest active clock."]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<0> {
        SWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_bus_mode](index.html) module"]
pub struct DMA_BUS_MODE_SPEC;
impl crate::RegisterSpec for DMA_BUS_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_bus_mode::R](R) reader structure"]
impl crate::Readable for DMA_BUS_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_bus_mode::W](W) writer structure"]
impl crate::Writable for DMA_BUS_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_BUS_MODE to value 0"]
impl crate::Resettable for DMA_BUS_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
