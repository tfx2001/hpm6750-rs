#[doc = "Register `DMA_STATUS` reader"]
pub struct R(crate::R<DMA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_STATUS` writer"]
pub struct W(crate::W<DMA_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_STATUS_SPEC>;
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
impl From<crate::W<DMA_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLPII` reader - GLPII: GMAC LPI Interrupt (for Channel 0) This bit indicates an interrupt event in the LPI logic of the MAC. To reset this bit to 1'b0, the software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source. Note: GLPII status is given only in Channel 0 DMA register and is applicable only when the Energy Efficient Ethernet feature is enabled. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high. -or- GTMSI: GMAC TMS Interrupt (for Channel 1 and Channel 2) This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac. To reset this bit, the software must read the corresponding registers (Channel Status Register) to get the exact cause of the interrupt and clear its source. Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high."]
pub type GLPII_R = crate::BitReader<bool>;
#[doc = "Field `GLPII` writer - GLPII: GMAC LPI Interrupt (for Channel 0) This bit indicates an interrupt event in the LPI logic of the MAC. To reset this bit to 1'b0, the software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source. Note: GLPII status is given only in Channel 0 DMA register and is applicable only when the Energy Efficient Ethernet feature is enabled. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high. -or- GTMSI: GMAC TMS Interrupt (for Channel 1 and Channel 2) This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac. To reset this bit, the software must read the corresponding registers (Channel Status Register) to get the exact cause of the interrupt and clear its source. Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high."]
pub type GLPII_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `TTI` reader - Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0. When this bit is high, the interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high. This bit is applicable only when the IEEE 1588 Timestamp feature is enabled. Otherwise, this bit is reserved."]
pub type TTI_R = crate::BitReader<bool>;
#[doc = "Field `TTI` writer - Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0. When this bit is high, the interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high. This bit is applicable only when the IEEE 1588 Timestamp feature is enabled. Otherwise, this bit is reserved."]
pub type TTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `GPI` reader - GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the Power Management feature is enabled. Otherwise, this bit is reserved. Note: The GPI and pmt_intr_o interrupts are generated in different clock domains."]
pub type GPI_R = crate::BitReader<bool>;
#[doc = "Field `GPI` writer - GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the Power Management feature is enabled. Otherwise, this bit is reserved. Note: The GPI and pmt_intr_o interrupts are generated in different clock domains."]
pub type GPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `GMI` reader - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the MAC Management Counters (MMC) are enabled. Otherwise, this bit is reserved."]
pub type GMI_R = crate::BitReader<bool>;
#[doc = "Field `GMI` writer - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the MAC Management Counters (MMC) are enabled. Otherwise, this bit is reserved."]
pub type GMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `GLI` reader - GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces (if present and enabled in your configuration): - PCS (TBI, RTBI, or SGMII): Link change or auto-negotiation complete event - SMII or RGMII: Link change event - General Purpose Input Status (GPIS): Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\]
of Register 14 (Interrupt Status Register) and then to clear the source of interrupt (which also clears the GLI interrupt), read any of the following corresponding registers: - PCS (TBI, RTBI, or SGMII): Register 49 (AN Status Register) - SMII or RGMII: Register 54 (SGMII/RGMII/SMII Control and Status Register) - General Purpose Input (GPI): Register 56 (General Purpose IO Register) The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high."]
pub type GLI_R = crate::BitReader<bool>;
#[doc = "Field `GLI` writer - GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces (if present and enabled in your configuration): - PCS (TBI, RTBI, or SGMII): Link change or auto-negotiation complete event - SMII or RGMII: Link change event - General Purpose Input Status (GPIS): Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\]
of Register 14 (Interrupt Status Register) and then to clear the source of interrupt (which also clears the GLI interrupt), read any of the following corresponding registers: - PCS (TBI, RTBI, or SGMII): Register 49 (AN Status Register) - SMII or RGMII: Register 54 (SGMII/RGMII/SMII Control and Status Register) - General Purpose Input (GPI): Register 56 (General Purpose IO Register) The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high."]
pub type GLI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `EB` reader - Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface. This field is valid only when Bit 13 (FBI) is set. This field does not generate an interrupt. - 0 0 0: Error during Rx DMA Write Data Transfer - 0 1 1: Error during Tx DMA Read Data Transfer - 1 0 0: Error during Rx DMA Descriptor Write Access - 1 0 1: Error during Tx DMA Descriptor Write Access - 1 1 0: Error during Rx DMA Descriptor Read Access - 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved."]
pub type EB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EB` writer - Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface. This field is valid only when Bit 13 (FBI) is set. This field does not generate an interrupt. - 0 0 0: Error during Rx DMA Write Data Transfer - 0 1 1: Error during Tx DMA Read Data Transfer - 1 0 0: Error during Rx DMA Descriptor Write Access - 1 0 1: Error during Tx DMA Descriptor Write Access - 1 1 0: Error during Rx DMA Descriptor Read Access - 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved."]
pub type EB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_STATUS_SPEC, u8, u8, 3, O>;
#[doc = "Field `TS` reader - Transmit Process State This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped; Reset or Stop Transmit Command issued - 3’b001: Running; Fetching Transmit Transfer Descriptor - 3’b010: Running; Waiting for status - 3’b011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO) - 3’b100: TIME_STAMP write state - 3’b101: Reserved for future use - 3’b110: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow - 3’b111: Running; Closing Transmit Descriptor"]
pub type TS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` writer - Transmit Process State This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped; Reset or Stop Transmit Command issued - 3’b001: Running; Fetching Transmit Transfer Descriptor - 3’b010: Running; Waiting for status - 3’b011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO) - 3’b100: TIME_STAMP write state - 3’b101: Reserved for future use - 3’b110: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow - 3’b111: Running; Closing Transmit Descriptor"]
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_STATUS_SPEC, u8, u8, 3, O>;
#[doc = "Field `RS` reader - Receive Process State This field indicates the Receive DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped: Reset or Stop Receive Command issued - 3’b001: Running: Fetching Receive Transfer Descriptor - 3’b010: Reserved for future use - 3’b011: Running: Waiting for receive packet - 3’b100: Suspended: Receive Descriptor Unavailable - 3’b101: Running: Closing Receive Descriptor - 3’b110: TIME_STAMP write state - 3’b111: Running: Transferring the receive packet data from receive buffer to host memory"]
pub type RS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RS` writer - Receive Process State This field indicates the Receive DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped: Reset or Stop Receive Command issued - 3’b001: Running: Fetching Receive Transfer Descriptor - 3’b010: Reserved for future use - 3’b011: Running: Waiting for receive packet - 3’b100: Suspended: Receive Descriptor Unavailable - 3’b101: Running: Closing Receive Descriptor - 3’b110: TIME_STAMP write state - 3’b111: Running: Transferring the receive packet data from receive buffer to host memory"]
pub type RS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_STATUS_SPEC, u8, u8, 3, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in Register 7) affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes NIS to be set, is cleared."]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in Register 7) affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes NIS to be set, is cleared."]
pub type NIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive FIFO Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive FIFO Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
pub type AIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `ERI` reader - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or Bit 6 (RI) of this register is set (whichever occurs earlier)."]
pub type ERI_R = crate::BitReader<bool>;
#[doc = "Field `ERI` writer - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or Bit 6 (RI) of this register is set (whichever occurs earlier)."]
pub type ERI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all of its bus accesses."]
pub type FBI_R = crate::BitReader<bool>;
#[doc = "Field `FBI` writer - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all of its bus accesses."]
pub type FBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `ETI` reader - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
pub type ETI_R = crate::BitReader<bool>;
#[doc = "Field `ETI` writer - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
pub type ETI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
pub type RWT_R = crate::BitReader<bool>;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
pub type RWT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `RPS` reader - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
pub type RPS_R = crate::BitReader<bool>;
#[doc = "Field `RPS` writer - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
pub type RPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `RU` reader - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
pub type RU_R = crate::BitReader<bool>;
#[doc = "Field `RU` writer - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
pub type RU_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `RI` reader - Receive Interrupt This bit indicates that the frame reception is complete. When reception is complete, the Bit 31 of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor, and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
pub type RI_R = crate::BitReader<bool>;
#[doc = "Field `RI` writer - Receive Interrupt This bit indicates that the frame reception is complete. When reception is complete, the Bit 31 of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor, and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
pub type RI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `UNF` reader - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
pub type UNF_R = crate::BitReader<bool>;
#[doc = "Field `UNF` writer - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
pub type UNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `OVF` reader - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]."]
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `OVF` writer - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]."]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 (10,240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
pub type TJT_R = crate::BitReader<bool>;
#[doc = "Field `TJT` writer - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 (10,240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
pub type TJT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\]
and then issue a Transmit Poll Demand command."]
pub type TU_R = crate::BitReader<bool>;
#[doc = "Field `TU` writer - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\]
and then issue a Transmit Poll Demand command."]
pub type TU_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `TPS` reader - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TPS_R = crate::BitReader<bool>;
#[doc = "Field `TPS` writer - Transmit Process Stopped This bit is set when the transmission is stopped."]
pub type TPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
#[doc = "Field `TI` reader - Transmit Interrupt This bit indicates that the frame transmission is complete. When transmission is complete, Bit 31 (OWN) of TDES0 is reset, and the specific frame status information is updated in the descriptor."]
pub type TI_R = crate::BitReader<bool>;
#[doc = "Field `TI` writer - Transmit Interrupt This bit indicates that the frame transmission is complete. When transmission is complete, Bit 31 (OWN) of TDES0 is reset, and the specific frame status information is updated in the descriptor."]
pub type TI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - GLPII: GMAC LPI Interrupt (for Channel 0) This bit indicates an interrupt event in the LPI logic of the MAC. To reset this bit to 1'b0, the software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source. Note: GLPII status is given only in Channel 0 DMA register and is applicable only when the Energy Efficient Ethernet feature is enabled. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high. -or- GTMSI: GMAC TMS Interrupt (for Channel 1 and Channel 2) This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac. To reset this bit, the software must read the corresponding registers (Channel Status Register) to get the exact cause of the interrupt and clear its source. Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high."]
    #[inline(always)]
    pub fn glpii(&self) -> GLPII_R {
        GLPII_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0. When this bit is high, the interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high. This bit is applicable only when the IEEE 1588 Timestamp feature is enabled. Otherwise, this bit is reserved."]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the Power Management feature is enabled. Otherwise, this bit is reserved. Note: The GPI and pmt_intr_o interrupts are generated in different clock domains."]
    #[inline(always)]
    pub fn gpi(&self) -> GPI_R {
        GPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the MAC Management Counters (MMC) are enabled. Otherwise, this bit is reserved."]
    #[inline(always)]
    pub fn gmi(&self) -> GMI_R {
        GMI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces (if present and enabled in your configuration): - PCS (TBI, RTBI, or SGMII): Link change or auto-negotiation complete event - SMII or RGMII: Link change event - General Purpose Input Status (GPIS): Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\]
of Register 14 (Interrupt Status Register) and then to clear the source of interrupt (which also clears the GLI interrupt), read any of the following corresponding registers: - PCS (TBI, RTBI, or SGMII): Register 49 (AN Status Register) - SMII or RGMII: Register 54 (SGMII/RGMII/SMII Control and Status Register) - General Purpose Input (GPI): Register 56 (General Purpose IO Register) The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high."]
    #[inline(always)]
    pub fn gli(&self) -> GLI_R {
        GLI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 23:25 - Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface. This field is valid only when Bit 13 (FBI) is set. This field does not generate an interrupt. - 0 0 0: Error during Rx DMA Write Data Transfer - 0 1 1: Error during Tx DMA Read Data Transfer - 1 0 0: Error during Rx DMA Descriptor Write Access - 1 0 1: Error during Tx DMA Descriptor Write Access - 1 1 0: Error during Rx DMA Descriptor Read Access - 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved."]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped; Reset or Stop Transmit Command issued - 3’b001: Running; Fetching Transmit Transfer Descriptor - 3’b010: Running; Waiting for status - 3’b011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO) - 3’b100: TIME_STAMP write state - 3’b101: Reserved for future use - 3’b110: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow - 3’b111: Running; Closing Transmit Descriptor"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Receive Process State This field indicates the Receive DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped: Reset or Stop Receive Command issued - 3’b001: Running: Fetching Receive Transfer Descriptor - 3’b010: Reserved for future use - 3’b011: Running: Waiting for receive packet - 3’b100: Suspended: Receive Descriptor Unavailable - 3’b101: Running: Closing Receive Descriptor - 3’b110: TIME_STAMP write state - 3’b111: Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in Register 7) affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes NIS to be set, is cleared."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive FIFO Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or Bit 6 (RI) of this register is set (whichever occurs earlier)."]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all of its bus accesses."]
    #[inline(always)]
    pub fn fbi(&self) -> FBI_R {
        FBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the frame reception is complete. When reception is complete, the Bit 31 of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor, and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 (10,240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\]
and then issue a Transmit Poll Demand command."]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the frame transmission is complete. When transmission is complete, Bit 31 (OWN) of TDES0 is reset, and the specific frame status information is updated in the descriptor."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - GLPII: GMAC LPI Interrupt (for Channel 0) This bit indicates an interrupt event in the LPI logic of the MAC. To reset this bit to 1'b0, the software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source. Note: GLPII status is given only in Channel 0 DMA register and is applicable only when the Energy Efficient Ethernet feature is enabled. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high. -or- GTMSI: GMAC TMS Interrupt (for Channel 1 and Channel 2) This bit indicates an interrupt event in the traffic manager and scheduler logic of DWC_gmac. To reset this bit, the software must read the corresponding registers (Channel Status Register) to get the exact cause of the interrupt and clear its source. Note: GTMSI status is given only in Channel 1 and Channel 2 DMA register when the AV feature is enabled and corresponding additional transmit channels are present. Otherwise, this bit is reserved. When this bit is high, the interrupt signal from the MAC (sbd_intr_o) is high."]
    #[inline(always)]
    pub fn glpii(&mut self) -> GLPII_W<30> {
        GLPII_W::new(self)
    }
    #[doc = "Bit 29 - Timestamp Trigger Interrupt This bit indicates an interrupt event in the Timestamp Generator block of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear its source to reset this bit to 1'b0. When this bit is high, the interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high. This bit is applicable only when the IEEE 1588 Timestamp feature is enabled. Otherwise, this bit is reserved."]
    #[inline(always)]
    pub fn tti(&mut self) -> TTI_W<29> {
        TTI_W::new(self)
    }
    #[doc = "Bit 28 - GMAC PMT Interrupt This bit indicates an interrupt event in the PMT module of the DWC_gmac. The software must read the PMT Control and Status Register in the MAC to get the exact cause of interrupt and clear its source to reset this bit to 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the Power Management feature is enabled. Otherwise, this bit is reserved. Note: The GPI and pmt_intr_o interrupts are generated in different clock domains."]
    #[inline(always)]
    pub fn gpi(&mut self) -> GPI_W<28> {
        GPI_W::new(self)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt This bit reflects an interrupt event in the MMC module of the DWC_gmac. The software must read the corresponding registers in the DWC_gmac to get the exact cause of the interrupt and clear the source of interrupt to make this bit as 1’b0. The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high. This bit is applicable only when the MAC Management Counters (MMC) are enabled. Otherwise, this bit is reserved."]
    #[inline(always)]
    pub fn gmi(&mut self) -> GMI_W<27> {
        GMI_W::new(self)
    }
    #[doc = "Bit 26 - GMAC Line Interface Interrupt When set, this bit reflects any of the following interrupt events in the DWC_gmac interfaces (if present and enabled in your configuration): - PCS (TBI, RTBI, or SGMII): Link change or auto-negotiation complete event - SMII or RGMII: Link change event - General Purpose Input Status (GPIS): Any LL or LH event on the gpi_i input ports To identify the exact cause of the interrupt, the software must first read Bit 11 and Bits\\[2:0\\]
of Register 14 (Interrupt Status Register) and then to clear the source of interrupt (which also clears the GLI interrupt), read any of the following corresponding registers: - PCS (TBI, RTBI, or SGMII): Register 49 (AN Status Register) - SMII or RGMII: Register 54 (SGMII/RGMII/SMII Control and Status Register) - General Purpose Input (GPI): Register 56 (General Purpose IO Register) The interrupt signal from the DWC_gmac subsystem (sbd_intr_o) is high when this bit is high."]
    #[inline(always)]
    pub fn gli(&mut self) -> GLI_W<26> {
        GLI_W::new(self)
    }
    #[doc = "Bits 23:25 - Error Bits This field indicates the type of error that caused a Bus Error, for example, error response on the AHB or AXI interface. This field is valid only when Bit 13 (FBI) is set. This field does not generate an interrupt. - 0 0 0: Error during Rx DMA Write Data Transfer - 0 1 1: Error during Tx DMA Read Data Transfer - 1 0 0: Error during Rx DMA Descriptor Write Access - 1 0 1: Error during Tx DMA Descriptor Write Access - 1 1 0: Error during Rx DMA Descriptor Read Access - 1 1 1: Error during Tx DMA Descriptor Read Access Note: 001 and 010 are reserved."]
    #[inline(always)]
    pub fn eb(&mut self) -> EB_W<23> {
        EB_W::new(self)
    }
    #[doc = "Bits 20:22 - Transmit Process State This field indicates the Transmit DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped; Reset or Stop Transmit Command issued - 3’b001: Running; Fetching Transmit Transfer Descriptor - 3’b010: Running; Waiting for status - 3’b011: Running; Reading Data from host memory buffer and queuing it to transmit buffer (Tx FIFO) - 3’b100: TIME_STAMP write state - 3’b101: Reserved for future use - 3’b110: Suspended; Transmit Descriptor Unavailable or Transmit Buffer Underflow - 3’b111: Running; Closing Transmit Descriptor"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W<20> {
        TS_W::new(self)
    }
    #[doc = "Bits 17:19 - Receive Process State This field indicates the Receive DMA FSM state. This field does not generate an interrupt. - 3’b000: Stopped: Reset or Stop Receive Command issued - 3’b001: Running: Fetching Receive Transfer Descriptor - 3’b010: Reserved for future use - 3’b011: Running: Waiting for receive packet - 3’b100: Suspended: Receive Descriptor Unavailable - 3’b101: Running: Closing Receive Descriptor - 3’b110: TIME_STAMP write state - 3’b111: Running: Transferring the receive packet data from receive buffer to host memory"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W<17> {
        RS_W::new(self)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary Normal Interrupt Summary bit value is the logical OR of the following bits when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[0\\]: Transmit Interrupt - Register 5\\[2\\]: Transmit Buffer Unavailable - Register 5\\[6\\]: Receive Interrupt - Register 5\\[14\\]: Early Receive Interrupt Only unmasked bits (interrupts for which interrupt enable is set in Register 7) affect the Normal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes NIS to be set, is cleared."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W<16> {
        NIS_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary Abnormal Interrupt Summary bit value is the logical OR of the following when the corresponding interrupt bits are enabled in Register 7 (Interrupt Enable Register): - Register 5\\[1\\]: Transmit Process Stopped - Register 5\\[3\\]: Transmit Jabber Timeout - Register 5\\[4\\]: Receive FIFO Overflow - Register 5\\[5\\]: Transmit Underflow - Register 5\\[7\\]: Receive Buffer Unavailable - Register 5\\[8\\]: Receive Process Stopped - Register 5\\[9\\]: Receive Watchdog Timeout - Register 5\\[10\\]: Early Transmit Interrupt - Register 5\\[13\\]: Fatal Bus Error Only unmasked bits affect the Abnormal Interrupt Summary bit. This is a sticky bit and must be cleared (by writing 1 to this bit) each time a corresponding bit, which causes AIS to be set, is cleared."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W<15> {
        AIS_W::new(self)
    }
    #[doc = "Bit 14 - Early Receive Interrupt This bit indicates that the DMA filled the first data buffer of the packet. This bit is cleared when the software writes 1 to this bit or Bit 6 (RI) of this register is set (whichever occurs earlier)."]
    #[inline(always)]
    pub fn eri(&mut self) -> ERI_W<14> {
        ERI_W::new(self)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt This bit indicates that a bus error occurred, as described in Bits \\[25:23\\]. When this bit is set, the corresponding DMA engine disables all of its bus accesses."]
    #[inline(always)]
    pub fn fbi(&mut self) -> FBI_W<13> {
        FBI_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt This bit indicates that the frame to be transmitted is fully transferred to the MTL Transmit FIFO."]
    #[inline(always)]
    pub fn eti(&mut self) -> ETI_W<10> {
        ETI_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout When set, this bit indicates that the Receive Watchdog Timer expired while receiving the current frame and the current frame is truncated after the watchdog timeout."]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped This bit is asserted when the Receive Process enters the Stopped state."]
    #[inline(always)]
    pub fn rps(&mut self) -> RPS_W<8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Receive List and the DMA cannot acquire it. The Receive Process is suspended. To resume processing Receive descriptors, the host should change the ownership of the descriptor and issue a Receive Poll Demand command. If no Receive Poll Demand is issued, the Receive Process resumes when the next recognized incoming frame is received. This bit is set only when the previous Receive Descriptor is owned by the DMA."]
    #[inline(always)]
    pub fn ru(&mut self) -> RU_W<7> {
        RU_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt This bit indicates that the frame reception is complete. When reception is complete, the Bit 31 of RDES1 (Disable Interrupt on Completion) is reset in the last Descriptor, and the specific frame status information is updated in the descriptor. The reception remains in the Running state."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W<6> {
        RI_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Underflow This bit indicates that the Transmit Buffer had an Underflow during frame transmission. Transmission is suspended and an Underflow Error TDES0\\[1\\]
is set."]
    #[inline(always)]
    pub fn unf(&mut self) -> UNF_W<5> {
        UNF_W::new(self)
    }
    #[doc = "Bit 4 - Receive Overflow This bit indicates that the Receive Buffer had an Overflow during frame reception. If the partial frame is transferred to the application, the overflow status is set in RDES0\\[11\\]."]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W<4> {
        OVF_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout This bit indicates that the Transmit Jabber Timer expired, which happens when the frame size exceeds 2,048 (10,240 bytes when the Jumbo frame is enabled). When the Jabber Timeout occurs, the transmission process is aborted and placed in the Stopped state. This causes the Transmit Jabber Timeout TDES0\\[14\\]
flag to assert."]
    #[inline(always)]
    pub fn tjt(&mut self) -> TJT_W<3> {
        TJT_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable This bit indicates that the host owns the Next Descriptor in the Transmit List and the DMA cannot acquire it. Transmission is suspended. Bits\\[22:20\\]
explain the Transmit Process state transitions. To resume processing Transmit descriptors, the host should change the ownership of the descriptor by setting TDES0\\[31\\]
and then issue a Transmit Poll Demand command."]
    #[inline(always)]
    pub fn tu(&mut self) -> TU_W<2> {
        TU_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped This bit is set when the transmission is stopped."]
    #[inline(always)]
    pub fn tps(&mut self) -> TPS_W<1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 0 - Transmit Interrupt This bit indicates that the frame transmission is complete. When transmission is complete, Bit 31 (OWN) of TDES0 is reset, and the specific frame status information is updated in the descriptor."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W<0> {
        TI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_status](index.html) module"]
pub struct DMA_STATUS_SPEC;
impl crate::RegisterSpec for DMA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_status::R](R) reader structure"]
impl crate::Readable for DMA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_status::W](W) writer structure"]
impl crate::Writable for DMA_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_STATUS to value 0"]
impl crate::Resettable for DMA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
