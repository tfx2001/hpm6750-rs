#[doc = "Register `DMA_OP_MODE` reader"]
pub struct R(crate::R<DMA_OP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OP_MODE` writer"]
pub struct W(crate::W<DMA_OP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OP_MODE_SPEC>;
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
impl From<crate::W<DMA_OP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT` reader - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors only in the encapsulated payload. When this bit is reset, all error frames are dropped if the FEF bit is reset. If the IPC Full Checksum Offload Engine (Type 2) is disabled, this bit is reserved (RO with value 1'b0)."]
pub type DT_R = crate::BitReader<bool>;
#[doc = "Field `DT` writer - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors only in the encapsulated payload. When this bit is reset, all error frames are dropped if the FEF bit is reset. If the IPC Full Checksum Offload Engine (Type 2) is disabled, this bit is reserved (RO with value 1'b0)."]
pub type DT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RSF` reader - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits. When this bit is reset, the Rx FIFO operates in the cut-through mode, subject to the threshold specified by the RTC bits."]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits. When this bit is reset, the Rx FIFO operates in the cut-through mode, subject to the threshold specified by the RTC bits."]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `DFF` reader - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset. (See “Receive Process Suspended” on page 83.)"]
pub type DFF_R = crate::BitReader<bool>;
#[doc = "Field `DFF` writer - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset. (See “Receive Process Suspended” on page 83.)"]
pub type DFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RFA_2` reader - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for activating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit), along with the RFA (Bits \\[10:9\\]), gives the following thresholds for activating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
pub type RFA_2_R = crate::BitReader<bool>;
#[doc = "Field `RFA_2` writer - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for activating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit), along with the RFA (Bits \\[10:9\\]), gives the following thresholds for activating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
pub type RFA_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RFD_2` reader - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for deactivating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit) along with the RFD (Bits \\[12:11\\]) gives the following thresholds for deactivating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
pub type RFD_2_R = crate::BitReader<bool>;
#[doc = "Field `RFD_2` writer - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for deactivating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit) along with the RFD (Bits \\[12:11\\]) gives the following thresholds for deactivating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
pub type RFD_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `TSF` reader - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Bits \\[16:14\\]
are ignored. This bit should be changed only when the transmission is stopped."]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Bits \\[16:14\\]
are ignored. This bit should be changed only when the transmission is stopped."]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `FTF` reader - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt frame transmission."]
pub type FTF_R = crate::BitReader<bool>;
#[doc = "Field `FTF` writer - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt frame transmission."]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `TTC` reader - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when Bit 21 (TSF) is reset. - 000: 64 - 001: 128 - 010: 192 - 011: 256 - 100: 40 - 101: 32 - 110: 24 - 111: 16"]
pub type TTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` writer - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when Bit 21 (TSF) is reset. - 000: 64 - 001: 128 - 010: 192 - 011: 256 - 100: 40 - 101: 32 - 110: 24 - 111: 16"]
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_OP_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 (Transmit Descriptor List Address Register), or from the position retained when transmission was stopped previously. If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 (Transmit Buffer Unavailable) of Register 5 (Status Register) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting Register 4 (Transmit Descriptor List Address Register), then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted. To change the list address, you need to program Register 4 (Transmit Descriptor List Address Register) with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state."]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 (Transmit Descriptor List Address Register), or from the position retained when transmission was stopped previously. If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 (Transmit Buffer Unavailable) of Register 5 (Status Register) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting Register 4 (Transmit Descriptor List Address Register), then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted. To change the list address, you need to program Register 4 (Transmit Descriptor List Address Register) with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state."]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RFD` reader - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill-level of Rx FIFO) at which the flow control is de-asserted after activation. - 00: Full minus 1 KB, that is, FULL — 1 KB - 01: Full minus 2 KB, that is, FULL — 2 KB - 10: Full minus 3 KB, that is, FULL — 3 KB - 11: Full minus 4 KB, that is, FULL — 4 KB The de-assertion is effective only after flow control is asserted. If the Rx FIFO is 8 KB or more, an additional Bit (RFD_2) is used for more threshold levels as described in Bit 22. These bits are reserved and read-only when the Rx FIFO depth is less than 4 KB."]
pub type RFD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFD` writer - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill-level of Rx FIFO) at which the flow control is de-asserted after activation. - 00: Full minus 1 KB, that is, FULL — 1 KB - 01: Full minus 2 KB, that is, FULL — 2 KB - 10: Full minus 3 KB, that is, FULL — 3 KB - 11: Full minus 4 KB, that is, FULL — 4 KB The de-assertion is effective only after flow control is asserted. If the Rx FIFO is 8 KB or more, an additional Bit (RFD_2) is used for more threshold levels as described in Bit 22. These bits are reserved and read-only when the Rx FIFO depth is less than 4 KB."]
pub type RFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_OP_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `RFA` reader - Threshold for Activating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill level of Rx FIFO) at which the flow control is activated. - 00: Full minus 1 KB, that is, FULL—1KB. - 01: Full minus 2 KB, that is, FULL—2KB. - 10: Full minus 3 KB, that is, FULL—3KB. - 11: Full minus 4 KB, that is, FULL—4KB. These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 (EFC) is set high. If the Rx FIFO is 8 KB or more, an additional Bit (RFA_2) is used for more threshold levels as described in Bit 23. These bits are reserved and read-only when the depth of Rx FIFO is less than 4 KB. Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11. The value 2'b11 means flow control on FIFO empty condition"]
pub type RFA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFA` writer - Threshold for Activating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill level of Rx FIFO) at which the flow control is activated. - 00: Full minus 1 KB, that is, FULL—1KB. - 01: Full minus 2 KB, that is, FULL—2KB. - 10: Full minus 3 KB, that is, FULL—3KB. - 11: Full minus 4 KB, that is, FULL—4KB. These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 (EFC) is set high. If the Rx FIFO is 8 KB or more, an additional Bit (RFA_2) is used for more threshold levels as described in Bit 23. These bits are reserved and read-only when the depth of Rx FIFO is less than 4 KB. Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11. The value 2'b11 means flow control on FIFO empty condition"]
pub type RFA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_OP_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `EFC` reader - Enable HW Flow Control When this bit is set, the flow control signal operation based on the fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled. This bit is not used (reserved and always reset) when the Rx FIFO is less than 4 KB."]
pub type EFC_R = crate::BitReader<bool>;
#[doc = "Field `EFC` writer - Enable HW Flow Control When this bit is set, the flow control signal operation based on the fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled. This bit is not used (reserved and always reset) when the Rx FIFO is less than 4 KB."]
pub type EFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `FEF` reader - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow). However, if the start byte (write) pointer of a frame is already transferred to the read controller side (in Threshold mode), then the frame is not dropped. In the GMAC-MTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus. When the FEF bit is set, all frames except runt error frames are forwarded to the DMA. If the Bit 25 (RSF) is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting. However, if the Bit 25 (RSF) is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA. Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status (in Table 8-6 or Table 8-23) in the following configurations: - The IP checksum engine (Type 1) and full checksum offload engine (Type 2) are not selected. - The advanced timestamp feature is not selected but the extended status is selected. The extended status is available with the following features: - L3-L4 filter in GMAC-CORE or GMAC-MTL configurations - Full checksum offload engine (Type 2) with enhanced descriptor format in the GMAC-DMA, GMAC-AHB, or GMAC-AXI configurations."]
pub type FEF_R = crate::BitReader<bool>;
#[doc = "Field `FEF` writer - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow). However, if the start byte (write) pointer of a frame is already transferred to the read controller side (in Threshold mode), then the frame is not dropped. In the GMAC-MTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus. When the FEF bit is set, all frames except runt error frames are forwarded to the DMA. If the Bit 25 (RSF) is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting. However, if the Bit 25 (RSF) is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA. Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status (in Table 8-6 or Table 8-23) in the following configurations: - The IP checksum engine (Type 1) and full checksum offload engine (Type 2) are not selected. - The advanced timestamp feature is not selected but the extended status is selected. The extended status is available with the following features: - L3-L4 filter in GMAC-CORE or GMAC-MTL configurations - Full checksum offload engine (Type 2) with enhanced descriptor format in the GMAC-DMA, GMAC-AHB, or GMAC-AXI configurations."]
pub type FEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `FUF` reader - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames (that is, frames with no Error and length less than 64 bytes) including pad-bytes and CRC. When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01."]
pub type FUF_R = crate::BitReader<bool>;
#[doc = "Field `FUF` writer - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames (that is, frames with no Error and length less than 64 bytes) including pad-bytes and CRC. When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01."]
pub type FUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `DGF` reader - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit. When reset, the MAC does not drop the giant frames in the Rx FIFO. Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: - Configurations in which IP Checksum Offload (Type 1) is selected in Rx - Configurations in which the IPC Full Checksum Offload Engine (Type 2) is selected in Rx with normal descriptor format - Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used (reserved and always reset)."]
pub type DGF_R = crate::BitReader<bool>;
#[doc = "Field `DGF` writer - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit. When reset, the MAC does not drop the giant frames in the Rx FIFO. Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: - Configurations in which IP Checksum Offload (Type 1) is selected in Rx - Configurations in which the IPC Full Checksum Offload Engine (Type 2) is selected in Rx with normal descriptor format - Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used (reserved and always reset)."]
pub type DGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `RTC` reader - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with length less than the threshold are automatically transferred. The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1. - 00: 64 - 01: 32 - 10: 96 - 11: 128"]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC` writer - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with length less than the threshold are automatically transferred. The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1. - 00: 64 - 01: 32 - 10: 96 - 11: 128"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_OP_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSF` reader - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
#[doc = "Field `SR` reader - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames. The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 (Receive Descriptor List Address Register) or the position retained when the Receive process was previously stopped. If the DMA does not own the descriptor, reception is suspended and Bit 7 (Receive Buffer Unavailable) of Register 5 (Status Register) is set. The Start Receive command is effective only when the reception has stopped. If the command is issued before setting Register 3 (Receive Descriptor List Address Register), the DMA behavior is unpredictable. When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames. The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 (Receive Descriptor List Address Register) or the position retained when the Receive process was previously stopped. If the DMA does not own the descriptor, reception is suspended and Bit 7 (Receive Buffer Unavailable) of Register 5 (Status Register) is set. The Start Receive command is effective only when the reception has stopped. If the command is issued before setting Register 3 (Receive Descriptor List Address Register), the DMA behavior is unpredictable. When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_OP_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors only in the encapsulated payload. When this bit is reset, all error frames are dropped if the FEF bit is reset. If the IPC Full Checksum Offload Engine (Type 2) is disabled, this bit is reserved (RO with value 1'b0)."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits. When this bit is reset, the Rx FIFO operates in the cut-through mode, subject to the threshold specified by the RTC bits."]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset. (See “Receive Process Suspended” on page 83.)"]
    #[inline(always)]
    pub fn dff(&self) -> DFF_R {
        DFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for activating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit), along with the RFA (Bits \\[10:9\\]), gives the following thresholds for activating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
    #[inline(always)]
    pub fn rfa_2(&self) -> RFA_2_R {
        RFA_2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for deactivating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit) along with the RFD (Bits \\[12:11\\]) gives the following thresholds for deactivating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
    #[inline(always)]
    pub fn rfd_2(&self) -> RFD_2_R {
        RFD_2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Bits \\[16:14\\]
are ignored. This bit should be changed only when the transmission is stopped."]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt frame transmission."]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when Bit 21 (TSF) is reset. - 000: 64 - 001: 128 - 010: 192 - 011: 256 - 100: 40 - 101: 32 - 110: 24 - 111: 16"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 (Transmit Descriptor List Address Register), or from the position retained when transmission was stopped previously. If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 (Transmit Buffer Unavailable) of Register 5 (Status Register) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting Register 4 (Transmit Descriptor List Address Register), then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted. To change the list address, you need to program Register 4 (Transmit Descriptor List Address Register) with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill-level of Rx FIFO) at which the flow control is de-asserted after activation. - 00: Full minus 1 KB, that is, FULL — 1 KB - 01: Full minus 2 KB, that is, FULL — 2 KB - 10: Full minus 3 KB, that is, FULL — 3 KB - 11: Full minus 4 KB, that is, FULL — 4 KB The de-assertion is effective only after flow control is asserted. If the Rx FIFO is 8 KB or more, an additional Bit (RFD_2) is used for more threshold levels as described in Bit 22. These bits are reserved and read-only when the Rx FIFO depth is less than 4 KB."]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Threshold for Activating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill level of Rx FIFO) at which the flow control is activated. - 00: Full minus 1 KB, that is, FULL—1KB. - 01: Full minus 2 KB, that is, FULL—2KB. - 10: Full minus 3 KB, that is, FULL—3KB. - 11: Full minus 4 KB, that is, FULL—4KB. These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 (EFC) is set high. If the Rx FIFO is 8 KB or more, an additional Bit (RFA_2) is used for more threshold levels as described in Bit 23. These bits are reserved and read-only when the depth of Rx FIFO is less than 4 KB. Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11. The value 2'b11 means flow control on FIFO empty condition"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable HW Flow Control When this bit is set, the flow control signal operation based on the fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled. This bit is not used (reserved and always reset) when the Rx FIFO is less than 4 KB."]
    #[inline(always)]
    pub fn efc(&self) -> EFC_R {
        EFC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow). However, if the start byte (write) pointer of a frame is already transferred to the read controller side (in Threshold mode), then the frame is not dropped. In the GMAC-MTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus. When the FEF bit is set, all frames except runt error frames are forwarded to the DMA. If the Bit 25 (RSF) is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting. However, if the Bit 25 (RSF) is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA. Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status (in Table 8-6 or Table 8-23) in the following configurations: - The IP checksum engine (Type 1) and full checksum offload engine (Type 2) are not selected. - The advanced timestamp feature is not selected but the extended status is selected. The extended status is available with the following features: - L3-L4 filter in GMAC-CORE or GMAC-MTL configurations - Full checksum offload engine (Type 2) with enhanced descriptor format in the GMAC-DMA, GMAC-AHB, or GMAC-AXI configurations."]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames (that is, frames with no Error and length less than 64 bytes) including pad-bytes and CRC. When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01."]
    #[inline(always)]
    pub fn fuf(&self) -> FUF_R {
        FUF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit. When reset, the MAC does not drop the giant frames in the Rx FIFO. Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: - Configurations in which IP Checksum Offload (Type 1) is selected in Rx - Configurations in which the IPC Full Checksum Offload Engine (Type 2) is selected in Rx with normal descriptor format - Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used (reserved and always reset)."]
    #[inline(always)]
    pub fn dgf(&self) -> DGF_R {
        DGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with length less than the threshold are automatically transferred. The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1. - 00: 64 - 01: 32 - 10: 96 - 11: 128"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 2 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames. The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 (Receive Descriptor List Address Register) or the position retained when the Receive process was previously stopped. If the DMA does not own the descriptor, reception is suspended and Bit 7 (Receive Buffer Unavailable) of Register 5 (Status Register) is set. The Start Receive command is effective only when the reception has stopped. If the command is issued before setting Register 3 (Receive Descriptor List Address Register), the DMA behavior is unpredictable. When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Disable Dropping of TCP/IP Checksum Error Frames When this bit is set, the MAC does not drop the frames which only have errors detected by the Receive Checksum Offload engine. Such frames do not have any errors (including FCS error) in the Ethernet frame received by the MAC but have errors only in the encapsulated payload. When this bit is reset, all error frames are dropped if the FEF bit is reset. If the IPC Full Checksum Offload Engine (Type 2) is disabled, this bit is reserved (RO with value 1'b0)."]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<28> {
        DT_W::new(self)
    }
    #[doc = "Bit 25 - Receive Store and Forward When this bit is set, the MTL reads a frame from the Rx FIFO only after the complete frame has been written to it, ignoring the RTC bits. When this bit is reset, the Rx FIFO operates in the cut-through mode, subject to the threshold specified by the RTC bits."]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<25> {
        RSF_W::new(self)
    }
    #[doc = "Bit 24 - Disable Flushing of Received Frames When this bit is set, the Rx DMA does not flush any frames because of the unavailability of receive descriptors or buffers as it does normally when this bit is reset. (See “Receive Process Suspended” on page 83.)"]
    #[inline(always)]
    pub fn dff(&mut self) -> DFF_W<24> {
        DFF_W::new(self)
    }
    #[doc = "Bit 23 - MSB of Threshold for Activating Flow Control If the DWC_gmac is configured for an Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for activating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit), along with the RFA (Bits \\[10:9\\]), gives the following thresholds for activating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
    #[inline(always)]
    pub fn rfa_2(&mut self) -> RFA_2_W<23> {
        RFA_2_W::new(self)
    }
    #[doc = "Bit 22 - MSB of Threshold for Deactivating Flow Control If the DWC_gmac is configured for Rx FIFO size of 8 KB or more, this bit (when set) provides additional threshold levels for deactivating the flow control in both half-duplex and full-duplex modes. This bit (as Most Significant Bit) along with the RFD (Bits \\[12:11\\]) gives the following thresholds for deactivating flow control: - 100: Full minus 5 KB, that is, FULL — 5 KB - 101: Full minus 6 KB, that is, FULL — 6 KB - 110: Full minus 7 KB, that is, FULL — 7 KB - 111: Reserved This bit is reserved (and RO) if the Rx FIFO is 4 KB or less deep."]
    #[inline(always)]
    pub fn rfd_2(&mut self) -> RFD_2_W<22> {
        RFD_2_W::new(self)
    }
    #[doc = "Bit 21 - Transmit Store and Forward When this bit is set, transmission starts when a full frame resides in the MTL Transmit FIFO. When this bit is set, the TTC values specified in Bits \\[16:14\\]
are ignored. This bit should be changed only when the transmission is stopped."]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<21> {
        TSF_W::new(self)
    }
    #[doc = "Bit 20 - Flush Transmit FIFO When this bit is set, the transmit FIFO controller logic is reset to its default values and thus all data in the Tx FIFO is lost or flushed. This bit is cleared internally when the flushing operation is complete. The Operation Mode register should not be written to until this bit is cleared. The data which is already accepted by the MAC transmitter is not flushed. It is scheduled for transmission and results in underflow and runt frame transmission."]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<20> {
        FTF_W::new(self)
    }
    #[doc = "Bits 14:16 - Transmit Threshold Control These bits control the threshold level of the MTL Transmit FIFO. Transmission starts when the frame size within the MTL Transmit FIFO is larger than the threshold. In addition, full frames with a length less than the threshold are also transmitted. These bits are used only when Bit 21 (TSF) is reset. - 000: 64 - 001: 128 - 010: 192 - 011: 256 - 100: 40 - 101: 32 - 110: 24 - 111: 16"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<14> {
        TTC_W::new(self)
    }
    #[doc = "Bit 13 - Start or Stop Transmission Command When this bit is set, transmission is placed in the Running state, and the DMA checks the Transmit List at the current position for a frame to be transmitted. Descriptor acquisition is attempted either from the current position in the list, which is the Transmit List Base Address set by Register 4 (Transmit Descriptor List Address Register), or from the position retained when transmission was stopped previously. If the DMA does not own the current descriptor, transmission enters the Suspended state and Bit 2 (Transmit Buffer Unavailable) of Register 5 (Status Register) is set. The Start Transmission command is effective only when transmission is stopped. If the command is issued before setting Register 4 (Transmit Descriptor List Address Register), then the DMA behavior is unpredictable. When this bit is reset, the transmission process is placed in the Stopped state after completing the transmission of the current frame. The Next Descriptor position in the Transmit List is saved, and it becomes the current position when transmission is restarted. To change the list address, you need to program Register 4 (Transmit Descriptor List Address Register) with a new value when this bit is reset. The new value is considered when this bit is set again. The stop transmission command is effective only when the transmission of the current frame is complete or the transmission is in the Suspended state."]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<13> {
        ST_W::new(self)
    }
    #[doc = "Bits 11:12 - Threshold for Deactivating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill-level of Rx FIFO) at which the flow control is de-asserted after activation. - 00: Full minus 1 KB, that is, FULL — 1 KB - 01: Full minus 2 KB, that is, FULL — 2 KB - 10: Full minus 3 KB, that is, FULL — 3 KB - 11: Full minus 4 KB, that is, FULL — 4 KB The de-assertion is effective only after flow control is asserted. If the Rx FIFO is 8 KB or more, an additional Bit (RFD_2) is used for more threshold levels as described in Bit 22. These bits are reserved and read-only when the Rx FIFO depth is less than 4 KB."]
    #[inline(always)]
    pub fn rfd(&mut self) -> RFD_W<11> {
        RFD_W::new(self)
    }
    #[doc = "Bits 9:10 - Threshold for Activating Flow Control (in half-duplex and full-duplex modes) These bits control the threshold (Fill level of Rx FIFO) at which the flow control is activated. - 00: Full minus 1 KB, that is, FULL—1KB. - 01: Full minus 2 KB, that is, FULL—2KB. - 10: Full minus 3 KB, that is, FULL—3KB. - 11: Full minus 4 KB, that is, FULL—4KB. These values are applicable only to Rx FIFOs of 4 KB or more and when Bit 8 (EFC) is set high. If the Rx FIFO is 8 KB or more, an additional Bit (RFA_2) is used for more threshold levels as described in Bit 23. These bits are reserved and read-only when the depth of Rx FIFO is less than 4 KB. Note: When FIFO size is exactly 4 KB, although the DWC_gmac allows you to program the value of these bits to 11, the software should not program these bits to 2'b11. The value 2'b11 means flow control on FIFO empty condition"]
    #[inline(always)]
    pub fn rfa(&mut self) -> RFA_W<9> {
        RFA_W::new(self)
    }
    #[doc = "Bit 8 - Enable HW Flow Control When this bit is set, the flow control signal operation based on the fill-level of Rx FIFO is enabled. When reset, the flow control operation is disabled. This bit is not used (reserved and always reset) when the Rx FIFO is less than 4 KB."]
    #[inline(always)]
    pub fn efc(&mut self) -> EFC_W<8> {
        EFC_W::new(self)
    }
    #[doc = "Bit 7 - Forward Error Frames When this bit is reset, the Rx FIFO drops frames with error status (CRC error, collision error, GMII_ER, giant frame, watchdog timeout, or overflow). However, if the start byte (write) pointer of a frame is already transferred to the read controller side (in Threshold mode), then the frame is not dropped. In the GMAC-MTL configuration in which the Frame Length FIFO is also enabled during core configuration, the Rx FIFO drops the error frames if that frame's start byte is not transferred (output) on the ARI bus. When the FEF bit is set, all frames except runt error frames are forwarded to the DMA. If the Bit 25 (RSF) is set and the Rx FIFO overflows when a partial frame is written, then the frame is dropped irrespective of the FEF bit setting. However, if the Bit 25 (RSF) is reset and the Rx FIFO overflows when a partial frame is written, then a partial frame may be forwarded to the DMA. Note: When FEF bit is reset, the giant frames are dropped if the giant frame status is given in Rx Status (in Table 8-6 or Table 8-23) in the following configurations: - The IP checksum engine (Type 1) and full checksum offload engine (Type 2) are not selected. - The advanced timestamp feature is not selected but the extended status is selected. The extended status is available with the following features: - L3-L4 filter in GMAC-CORE or GMAC-MTL configurations - Full checksum offload engine (Type 2) with enhanced descriptor format in the GMAC-DMA, GMAC-AHB, or GMAC-AXI configurations."]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<7> {
        FEF_W::new(self)
    }
    #[doc = "Bit 6 - Forward Undersized Good Frames When set, the Rx FIFO forwards Undersized frames (that is, frames with no Error and length less than 64 bytes) including pad-bytes and CRC. When reset, the Rx FIFO drops all frames of less than 64 bytes, unless a frame is already transferred because of the lower value of Receive Threshold, for example, RTC = 01."]
    #[inline(always)]
    pub fn fuf(&mut self) -> FUF_W<6> {
        FUF_W::new(self)
    }
    #[doc = "Bit 5 - Drop Giant Frames When set, the MAC drops the received giant frames in the Rx FIFO, that is, frames that are larger than the computed giant frame limit. When reset, the MAC does not drop the giant frames in the Rx FIFO. Note: This bit is available in the following configurations in which the giant frame status is not provided in Rx status and giant frames are not dropped by default: - Configurations in which IP Checksum Offload (Type 1) is selected in Rx - Configurations in which the IPC Full Checksum Offload Engine (Type 2) is selected in Rx with normal descriptor format - Configurations in which the Advanced Timestamp feature is selected In all other configurations, this bit is not used (reserved and always reset)."]
    #[inline(always)]
    pub fn dgf(&mut self) -> DGF_W<5> {
        DGF_W::new(self)
    }
    #[doc = "Bits 3:4 - Receive Threshold Control These two bits control the threshold level of the MTL Receive FIFO. Transfer (request) to DMA starts when the frame size within the MTL Receive FIFO is larger than the threshold. In addition, full frames with length less than the threshold are automatically transferred. The value of 11 is not applicable if the configured Receive FIFO size is 128 bytes. These bits are valid only when the RSF bit is zero, and are ignored when the RSF bit is set to 1. - 00: 64 - 01: 32 - 10: 96 - 11: 128"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<3> {
        RTC_W::new(self)
    }
    #[doc = "Bit 2 - Operate on Second Frame When this bit is set, it instructs the DMA to process the second frame of the Transmit data even before the status for the first frame is obtained."]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<2> {
        OSF_W::new(self)
    }
    #[doc = "Bit 1 - Start or Stop Receive When this bit is set, the Receive process is placed in the Running state. The DMA attempts to acquire the descriptor from the Receive list and processes the incoming frames. The descriptor acquisition is attempted from the current position in the list, which is the address set by the Register 3 (Receive Descriptor List Address Register) or the position retained when the Receive process was previously stopped. If the DMA does not own the descriptor, reception is suspended and Bit 7 (Receive Buffer Unavailable) of Register 5 (Status Register) is set. The Start Receive command is effective only when the reception has stopped. If the command is issued before setting Register 3 (Receive Descriptor List Address Register), the DMA behavior is unpredictable. When this bit is cleared, the Rx DMA operation is stopped after the transfer of the current frame. The next descriptor position in the Receive list is saved and becomes the current position after the Receive process is restarted. The Stop Receive command is effective only when the Receive process is in either the Running (waiting for receive packet) or in the Suspended state."]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<1> {
        SR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operation Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_op_mode](index.html) module"]
pub struct DMA_OP_MODE_SPEC;
impl crate::RegisterSpec for DMA_OP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_op_mode::R](R) reader structure"]
impl crate::Readable for DMA_OP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_op_mode::W](W) writer structure"]
impl crate::Writable for DMA_OP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_OP_MODE to value 0"]
impl crate::Resettable for DMA_OP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
