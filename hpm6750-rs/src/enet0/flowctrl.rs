#[doc = "Register `FLOWCTRL` reader"]
pub struct R(crate::R<FLOWCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOWCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOWCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOWCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOWCTRL` writer"]
pub struct W(crate::W<FLOWCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOWCTRL_SPEC>;
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
impl From<crate::W<FLOWCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOWCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PT` reader - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the (G)MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
pub type PT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PT` writer - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the (G)MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
pub type PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOWCTRL_SPEC, u16, u16, 16, O>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer (MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i). When this bit is reset, normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
pub type DZPQ_R = crate::BitReader<bool>;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer (MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i). When this bit is reset, normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
pub type DZPQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOWCTRL_SPEC, bool, O>;
#[doc = "Field `PLT` reader - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is checked for automatic retransmission of the Pause frame. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 (256 – 28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: - 00: The threshold is Pause time minus 4 slot times (PT – 4 slot times). - 01: The threshold is Pause time minus 28 slot times (PT – 28 slot times). - 10: The threshold is Pause time minus 144 slot times (PT – 144 slot times). - 11: The threshold is Pause time minus 256 slot times (PT – 256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the GMII or MII interface."]
pub type PLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLT` writer - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is checked for automatic retransmission of the Pause frame. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 (256 – 28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: - 00: The threshold is Pause time minus 4 slot times (PT – 4 slot times). - 01: The threshold is Pause time minus 28 slot times (PT – 28 slot times). - 10: The threshold is Pause time minus 144 slot times (PT – 144 slot times). - 11: The threshold is Pause time minus 256 slot times (PT – 256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the GMII or MII interface."]
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLOWCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set, the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register. When this bit is reset, the MAC only detects Pause frames with unique multicast address."]
pub type UP_R = crate::BitReader<bool>;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set, the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register. When this bit is reset, the MAC only detects Pause frames with unique multicast address."]
pub type UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOWCTRL_SPEC, bool, O>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset, the decode function of the Pause frame is disabled."]
pub type RFE_R = crate::BitReader<bool>;
#[doc = "Field `RFE` writer - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset, the decode function of the Pause frame is disabled."]
pub type RFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOWCTRL_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable In the full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames. In the half-duplex mode, when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable In the full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames. In the half-duplex mode, when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOWCTRL_SPEC, bool, O>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFE bit is set. In the full-duplex mode, this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame, the Application must set this bit to 1'b1. During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission, the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode, when this bit is set (and TFE is set), then backpressure is asserted by the MAC. During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function. When the MAC is configured for the full-duplex mode, the BPA is automatically disabled."]
pub type FCB_BPA_R = crate::BitReader<bool>;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFE bit is set. In the full-duplex mode, this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame, the Application must set this bit to 1'b1. During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission, the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode, when this bit is set (and TFE is set), then backpressure is asserted by the MAC. During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function. When the MAC is configured for the full-duplex mode, the BPA is automatically disabled."]
pub type FCB_BPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLOWCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:31 - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the (G)MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer (MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i). When this bit is reset, normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is checked for automatic retransmission of the Pause frame. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 (256 – 28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: - 00: The threshold is Pause time minus 4 slot times (PT – 4 slot times). - 01: The threshold is Pause time minus 28 slot times (PT – 28 slot times). - 10: The threshold is Pause time minus 144 slot times (PT – 144 slot times). - 11: The threshold is Pause time minus 256 slot times (PT – 256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the GMII or MII interface."]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set, the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register. When this bit is reset, the MAC only detects Pause frames with unique multicast address."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset, the decode function of the Pause frame is disabled."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In the full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames. In the half-duplex mode, when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFE bit is set. In the full-duplex mode, this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame, the Application must set this bit to 1'b1. During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission, the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode, when this bit is set (and TFE is set), then backpressure is asserted by the MAC. During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function. When the MAC is configured for the full-duplex mode, the BPA is automatically disabled."]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Pause Time This field holds the value to be used in the Pause Time field in the transmit control frame. If the Pause Time bits is configured to be double-synchronized to the (G)MII clock domain, then consecutive writes to this register should be performed only after at least four clock cycles in the destination clock domain."]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When this bit is set, it disables the automatic generation of the Zero-Quanta Pause frames on the de-assertion of the flow-control signal from the FIFO layer (MTL or external sideband flow control signal sbd_flowctrl_i/mti_flowctrl_i). When this bit is reset, normal operation with automatic Zero-Quanta Pause frame generation is enabled."]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W<7> {
        DZPQ_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold This field configures the threshold of the Pause timer at which the input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is checked for automatic retransmission of the Pause frame. The threshold values should be always less than the Pause Time configured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-times), and PLT = 01, then a second Pause frame is automatically transmitted if the mti_flowctrl_i signal is asserted at 228 (256 – 28) slot times after the first Pause frame is transmitted. The following list provides the threshold values for different values: - 00: The threshold is Pause time minus 4 slot times (PT – 4 slot times). - 01: The threshold is Pause time minus 28 slot times (PT – 28 slot times). - 10: The threshold is Pause time minus 144 slot times (PT – 144 slot times). - 11: The threshold is Pause time minus 256 slot times (PT – 256 slot times). The slot time is defined as the time taken to transmit 512 bits (64 bytes) on the GMII or MII interface."]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect A pause frame is processed when it has the unique multicast address specified in the IEEE Std 802.3. When this bit is set, the MAC can also detect Pause frames with unicast address of the station. This unicast address should be as specified in the MAC Address0 High Register and MAC Address0 Low Register. When this bit is reset, the MAC only detects Pause frames with unique multicast address."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W<3> {
        UP_W::new(self)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable When this bit is set, the MAC decodes the received Pause frame and disables its transmitter for a specified (Pause) time. When this bit is reset, the decode function of the Pause frame is disabled."]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W<2> {
        RFE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In the full-duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames. When this bit is reset, the flow control operation in the MAC is disabled, and the MAC does not transmit any Pause frames. In the half-duplex mode, when this bit is set, the MAC enables the backpressure operation. When this bit is reset, the backpressure feature is disabled."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<1> {
        TFE_W::new(self)
    }
    #[doc = "Bit 0 - Flow Control Busy or Backpressure Activate This bit initiates a Pause frame in the full-duplex mode and activates the backpressure function in the half-duplex mode if the TFE bit is set. In the full-duplex mode, this bit should be read as 1'b0 before writing to the Flow Control register. To initiate a Pause frame, the Application must set this bit to 1'b1. During a transfer of the Control Frame, this bit continues to be set to signify that a frame transmission is in progress. After the completion of Pause frame transmission, the MAC resets this bit to 1'b0. The Flow Control register should not be written to until this bit is cleared. In the half-duplex mode, when this bit is set (and TFE is set), then backpressure is asserted by the MAC. During backpressure, when the MAC receives a new frame, the transmitter starts sending a JAM pattern resulting in a collision. This control register bit is logically ORed with the mti_flowctrl_i input signal for the backpressure function. When the MAC is configured for the full-duplex mode, the BPA is automatically disabled."]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<0> {
        FCB_BPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flowctrl](index.html) module"]
pub struct FLOWCTRL_SPEC;
impl crate::RegisterSpec for FLOWCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flowctrl::R](R) reader structure"]
impl crate::Readable for FLOWCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flowctrl::W](W) writer structure"]
impl crate::Writable for FLOWCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOWCTRL to value 0"]
impl crate::Resettable for FLOWCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
