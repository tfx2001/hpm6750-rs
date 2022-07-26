#[doc = "Register `MACFF` reader"]
pub struct R(crate::R<MACFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFF` writer"]
pub struct W(crate::W<MACFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFF_SPEC>;
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
impl From<crate::W<MACFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter."]
pub type RA_R = crate::BitReader<bool>;
#[doc = "Field `RA` writer - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter."]
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `DNTU` reader - Drop non-TCP/UDP over IP Frames When set, this bit enables the MAC to drop the non-TCP or UDP over IP frames. The MAC forward only those frames that are processed by the Layer 4 filter. When reset, this bit enables the MAC to forward all non-TCP or UDP over IP frames."]
pub type DNTU_R = crate::BitReader<bool>;
#[doc = "Field `DNTU` writer - Drop non-TCP/UDP over IP Frames When set, this bit enables the MAC to drop the non-TCP or UDP over IP frames. The MAC forward only those frames that are processed by the Layer 4 filter. When reset, this bit enables the MAC to forward all non-TCP or UDP over IP frames."]
pub type DNTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `IPFE` reader - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields."]
pub type IPFE_R = crate::BitReader<bool>;
#[doc = "Field `IPFE` writer - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields."]
pub type IPFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `VTFE` reader - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison. When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag."]
pub type VTFE_R = crate::BitReader<bool>;
#[doc = "Field `VTFE` writer - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison. When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag."]
pub type VTFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits. When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter."]
pub type HPF_R = crate::BitReader<bool>;
#[doc = "Field `HPF` writer - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits. When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter."]
pub type HPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `SAF` reader - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the frame. When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` writer - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the frame. When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
pub type SAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `SAIF` reader - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
pub type SAIF_R = crate::BitReader<bool>;
#[doc = "Field `SAIF` writer - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
pub type SAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `PCF` reader - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast Pause frames). - 00: MAC filters all control frames from reaching the application. - 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. - 10: MAC forwards all control frames to application even if they fail the Address Filter. - 11: MAC forwards control frames that pass the Address Filter. The following conditions should be true for the Pause frames processing: - Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register 6 (Flow Control Register) to 1. - Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register 6 (Flow Control Register) is set. - Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001. Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the full-duplex mode and the RFE bit is enabled. Otherwise, the Pause frame filtering may be inconsistent. When Condition 1 is false, the Pause frames are considered as generic control frames. Therefore, to pass all control frames (including Pause frames) when the full-duplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 (as required by the application)."]
pub type PCF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCF` writer - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast Pause frames). - 00: MAC filters all control frames from reaching the application. - 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. - 10: MAC forwards all control frames to application even if they fail the Address Filter. - 11: MAC forwards control frames that pass the Address Filter. The following conditions should be true for the Pause frames processing: - Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register 6 (Flow Control Register) to 1. - Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register 6 (Flow Control Register) is set. - Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001. Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the full-duplex mode and the RFE bit is enabled. Otherwise, the Pause frame filtering may be inconsistent. When Condition 1 is false, the Pause frames are considered as generic control frames. Therefore, to pass all control frames (including Pause frames) when the full-duplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 (as required by the application)."]
pub type PCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACFF_SPEC, u8, u8, 2, O>;
#[doc = "Field `DBF` reader - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast frames."]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast frames."]
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `PM` reader - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed. When reset, filtering of multicast frame depends on HMC bit."]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed. When reset, filtering of multicast frame depends on HMC bit."]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset, normal filtering of frames is performed."]
pub type DAIF_R = crate::BitReader<bool>;
#[doc = "Field `DAIF` writer - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset, normal filtering of frames is performed."]
pub type DAIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `HMC` reader - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers."]
pub type HMC_R = crate::BitReader<bool>;
#[doc = "Field `HMC` writer - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers."]
pub type HMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `HUC` reader - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers."]
pub type HUC_R = crate::BitReader<bool>;
#[doc = "Field `HUC` writer - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers."]
pub type HUC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
#[doc = "Field `PR` reader - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set."]
pub type PR_R = crate::BitReader<bool>;
#[doc = "Field `PR` writer - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set."]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 21 - Drop non-TCP/UDP over IP Frames When set, this bit enables the MAC to drop the non-TCP or UDP over IP frames. The MAC forward only those frames that are processed by the Layer 4 filter. When reset, this bit enables the MAC to forward all non-TCP or UDP over IP frames."]
    #[inline(always)]
    pub fn dntu(&self) -> DNTU_R {
        DNTU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields."]
    #[inline(always)]
    pub fn ipfe(&self) -> IPFE_R {
        IPFE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 15 - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison. When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag."]
    #[inline(always)]
    pub fn vtfe(&self) -> VTFE_R {
        VTFE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits. When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter."]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the frame. When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast Pause frames). - 00: MAC filters all control frames from reaching the application. - 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. - 10: MAC forwards all control frames to application even if they fail the Address Filter. - 11: MAC forwards control frames that pass the Address Filter. The following conditions should be true for the Pause frames processing: - Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register 6 (Flow Control Register) to 1. - Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register 6 (Flow Control Register) is set. - Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001. Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the full-duplex mode and the RFE bit is enabled. Otherwise, the Pause frame filtering may be inconsistent. When Condition 1 is false, the Pause frames are considered as generic control frames. Therefore, to pass all control frames (including Pause frames) when the full-duplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 (as required by the application)."]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast frames."]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed. When reset, filtering of multicast frame depends on HMC bit."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset, normal filtering of frames is performed."]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers."]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers."]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Receive All When this bit is set, the MAC Receiver module passes all received frames, irrespective of whether they pass the address filter or not, to the Application. The result of the SA or DA filtering is updated (pass or fail) in the corresponding bits in the Receive Status Word. When this bit is reset, the Receiver module passes only those frames to the Application that pass the SA or DA address filter."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<31> {
        RA_W::new(self)
    }
    #[doc = "Bit 21 - Drop non-TCP/UDP over IP Frames When set, this bit enables the MAC to drop the non-TCP or UDP over IP frames. The MAC forward only those frames that are processed by the Layer 4 filter. When reset, this bit enables the MAC to forward all non-TCP or UDP over IP frames."]
    #[inline(always)]
    pub fn dntu(&mut self) -> DNTU_W<21> {
        DNTU_W::new(self)
    }
    #[doc = "Bit 20 - Layer 3 and Layer 4 Filter Enable When set, this bit enables the MAC to drop frames that do not match the enabled Layer 3 and Layer 4 filters. If Layer 3 or Layer 4 filters are not enabled for matching, this bit does not have any effect. When reset, the MAC forwards all frames irrespective of the match status of the Layer 3 and Layer 4 fields."]
    #[inline(always)]
    pub fn ipfe(&mut self) -> IPFE_W<20> {
        IPFE_W::new(self)
    }
    #[doc = "Bit 15 - VLAN Tag Filter Enable When set, this bit enables the MAC to drop VLAN tagged frames that do not match the VLAN Tag comparison. When reset, the MAC forwards all frames irrespective of the match status of the VLAN Tag."]
    #[inline(always)]
    pub fn vtfe(&mut self) -> VTFE_W<15> {
        VTFE_W::new(self)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter When this bit is set, it configures the address filter to pass a frame if it matches either the perfect filtering or the hash filtering as set by the HMC or HUC bits. When this bit is low and the HUC or HMC bit is set, the frame is passed only if it matches the Hash filter."]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<10> {
        HPF_W::new(self)
    }
    #[doc = "Bit 9 - Source Address Filter Enable When this bit is set, the MAC compares the SA field of the received frames with the values programmed in the enabled SA registers. If the comparison fails, the MAC drops the frame. When this bit is reset, the MAC forwards the received frame to the application with updated SAF bit of the Rx Status depending on the SA address comparison."]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<9> {
        SAF_W::new(self)
    }
    #[doc = "Bit 8 - SA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the SA address comparison. The frames whose SA matches the SA registers are marked as failing the SA Address filter. When this bit is reset, frames whose SA does not match the SA registers are marked as failing the SA Address filter."]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<8> {
        SAIF_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass Control Frames These bits control the forwarding of all control frames (including unicast and multicast Pause frames). - 00: MAC filters all control frames from reaching the application. - 01: MAC forwards all control frames except Pause frames to application even if they fail the Address filter. - 10: MAC forwards all control frames to application even if they fail the Address Filter. - 11: MAC forwards control frames that pass the Address Filter. The following conditions should be true for the Pause frames processing: - Condition 1: The MAC is in the full-duplex mode and flow control is enabled by setting Bit 2 (RFE) of Register 6 (Flow Control Register) to 1. - Condition 2: The destination address (DA) of the received frame matches the special multicast address or the MAC Address 0 when Bit 3 (UP) of the Register 6 (Flow Control Register) is set. - Condition 3: The Type field of the received frame is 0x8808 and the OPCODE field is 0x0001. Note: This field should be set to 01 only when the Condition 1 is true, that is, the MAC is programmed to operate in the full-duplex mode and the RFE bit is enabled. Otherwise, the Pause frame filtering may be inconsistent. When Condition 1 is false, the Pause frames are considered as generic control frames. Therefore, to pass all control frames (including Pause frames) when the full-duplex mode and flow control is not enabled, you should set the PCF field to 10 or 11 (as required by the application)."]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<6> {
        PCF_W::new(self)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames When this bit is set, the AFM module blocks all incoming broadcast frames. In addition, it overrides all other filter settings. When this bit is reset, the AFM module passes all received broadcast frames."]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W<5> {
        DBF_W::new(self)
    }
    #[doc = "Bit 4 - Pass All Multicast When set, this bit indicates that all received frames with a multicast destination address (first bit in the destination address field is '1') are passed. When reset, filtering of multicast frame depends on HMC bit."]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<4> {
        PM_W::new(self)
    }
    #[doc = "Bit 3 - DA Inverse Filtering When this bit is set, the Address Check block operates in inverse filtering mode for the DA address comparison for both unicast and multicast frames. When reset, normal filtering of frames is performed."]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<3> {
        DAIF_W::new(self)
    }
    #[doc = "Bit 2 - Hash Multicast When set, the MAC performs destination address filtering of received multicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for multicast frames, that is, it compares the DA field with the values programmed in DA registers."]
    #[inline(always)]
    pub fn hmc(&mut self) -> HMC_W<2> {
        HMC_W::new(self)
    }
    #[doc = "Bit 1 - Hash Unicast When set, the MAC performs destination address filtering of unicast frames according to the hash table. When reset, the MAC performs a perfect destination address filtering for unicast frames, that is, it compares the DA field with the values programmed in DA registers."]
    #[inline(always)]
    pub fn huc(&mut self) -> HUC_W<1> {
        HUC_W::new(self)
    }
    #[doc = "Bit 0 - Promiscuous Mode When this bit is set, the Address Filter module passes all incoming frames irrespective of the destination or source address. The SA or DA Filter Fails status bits of the Receive Status Word are always cleared when PR is set."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Frame Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macff](index.html) module"]
pub struct MACFF_SPEC;
impl crate::RegisterSpec for MACFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macff::R](R) reader structure"]
impl crate::Readable for MACFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macff::W](W) writer structure"]
impl crate::Writable for MACFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFF to value 0"]
impl crate::Resettable for MACFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
