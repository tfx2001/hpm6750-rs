#[doc = "Register `GMII_ADDR` reader"]
pub struct R(crate::R<GMII_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMII_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMII_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMII_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GMII_ADDR` writer"]
pub struct W(crate::W<GMII_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GMII_ADDR_SPEC>;
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
impl From<crate::W<GMII_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GMII_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GB` reader - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5. During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress. Register 5 is invalid until this bit is cleared by the MAC. Therefore, Register 5 (GMII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present."]
pub type GB_R = crate::BitReader<bool>;
#[doc = "Field `GB` writer - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5. During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress. Register 5 is invalid until this bit is cleared by the MAC. Therefore, Register 5 (GMII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present."]
pub type GB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GMII_ADDR_SPEC, bool, O>;
#[doc = "Field `GW` reader - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register. If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register."]
pub type GW_R = crate::BitReader<bool>;
#[doc = "Field `GW` writer - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register. If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register."]
pub type GW_W<'a, const O: u8> = crate::BitWriter<'a, u32, GMII_ADDR_SPEC, bool, O>;
#[doc = "Field `CR` reader - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design. The CSR clock corresponding to different GMAC configurations is given in Table 9-2 on page 564. The suggested range of CSR clock frequency applicable for each value (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz–2.5 MHz. - 0000: The CSR clock frequency is 60–100 MHz and the MDC clock frequency is CSR clock/42. - 0001: The CSR clock frequency is 100–150 MHz and the MDC clock frequency is CSR clock/62. - 0010: The CSR clock frequency is 20–35 MHz and the MDC clock frequency is CSR clock/16. - 0011: The CSR clock frequency is 35–60 MHz and the MDC clock frequency is CSR clock/26. - 0100: The CSR clock frequency is 150–250 MHz and the MDC clock frequency is CSR clock/102. - 0101: The CSR clock frequency is 250–300 MHz and the MDC clock is CSR clock/124. - 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE Std 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the following values only if the interfacing chips support faster MDC clocks. - 1000: CSR clock/4 - 1001: CSR clock/6 - 1010: CSR clock/8 - 1011: CSR clock/10 - 1100: CSR clock/12 - 1101: CSR clock/14 - 1110: CSR clock/16 - 1111: CSR clock/18 These bits are not used for accessing RevMII. These bits are read-only if the RevMII interface is selected as single PHY interface."]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design. The CSR clock corresponding to different GMAC configurations is given in Table 9-2 on page 564. The suggested range of CSR clock frequency applicable for each value (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz–2.5 MHz. - 0000: The CSR clock frequency is 60–100 MHz and the MDC clock frequency is CSR clock/42. - 0001: The CSR clock frequency is 100–150 MHz and the MDC clock frequency is CSR clock/62. - 0010: The CSR clock frequency is 20–35 MHz and the MDC clock frequency is CSR clock/16. - 0011: The CSR clock frequency is 35–60 MHz and the MDC clock frequency is CSR clock/26. - 0100: The CSR clock frequency is 150–250 MHz and the MDC clock frequency is CSR clock/102. - 0101: The CSR clock frequency is 250–300 MHz and the MDC clock is CSR clock/124. - 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE Std 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the following values only if the interfacing chips support faster MDC clocks. - 1000: CSR clock/4 - 1001: CSR clock/6 - 1010: CSR clock/8 - 1011: CSR clock/10 - 1100: CSR clock/12 - 1101: CSR clock/14 - 1110: CSR clock/16 - 1111: CSR clock/18 These bits are not used for accessing RevMII. These bits are read-only if the RevMII interface is selected as single PHY interface."]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMII_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `GR` reader - GMII Register These bits select the desired GMII register in the selected PHY device. For RevMII, these bits select the desired CSR register in the RevMII Registers set."]
pub type GR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GR` writer - GMII Register These bits select the desired GMII register in the selected PHY device. For RevMII, these bits select the desired CSR register in the RevMII Registers set."]
pub type GR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMII_ADDR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed. For RevMII, this field gives the PHY Address of the RevMII module."]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed. For RevMII, this field gives the PHY Address of the RevMII module."]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GMII_ADDR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5. During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress. Register 5 is invalid until this bit is cleared by the MAC. Therefore, Register 5 (GMII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present."]
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register. If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register."]
    #[inline(always)]
    pub fn gw(&self) -> GW_R {
        GW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design. The CSR clock corresponding to different GMAC configurations is given in Table 9-2 on page 564. The suggested range of CSR clock frequency applicable for each value (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz–2.5 MHz. - 0000: The CSR clock frequency is 60–100 MHz and the MDC clock frequency is CSR clock/42. - 0001: The CSR clock frequency is 100–150 MHz and the MDC clock frequency is CSR clock/62. - 0010: The CSR clock frequency is 20–35 MHz and the MDC clock frequency is CSR clock/16. - 0011: The CSR clock frequency is 35–60 MHz and the MDC clock frequency is CSR clock/26. - 0100: The CSR clock frequency is 150–250 MHz and the MDC clock frequency is CSR clock/102. - 0101: The CSR clock frequency is 250–300 MHz and the MDC clock is CSR clock/124. - 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE Std 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the following values only if the interfacing chips support faster MDC clocks. - 1000: CSR clock/4 - 1001: CSR clock/6 - 1010: CSR clock/8 - 1011: CSR clock/10 - 1100: CSR clock/12 - 1101: CSR clock/14 - 1110: CSR clock/16 - 1111: CSR clock/18 These bits are not used for accessing RevMII. These bits are read-only if the RevMII interface is selected as single PHY interface."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - GMII Register These bits select the desired GMII register in the selected PHY device. For RevMII, these bits select the desired CSR register in the RevMII Registers set."]
    #[inline(always)]
    pub fn gr(&self) -> GR_R {
        GR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed. For RevMII, this field gives the PHY Address of the RevMII module."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - GMII Busy This bit should read logic 0 before writing to Register 4 and Register 5. During a PHY or RevMII register access, the software sets this bit to 1’b1 to indicate that a Read or Write access is in progress. Register 5 is invalid until this bit is cleared by the MAC. Therefore, Register 5 (GMII Data) should be kept valid until the MAC clears this bit during a PHY Write operation. Similarly for a read operation, the contents of Register 5 are not valid until this bit is cleared. The subsequent read or write operation should happen only after the previous operation is complete. Because there is no acknowledgment from the PHY to MAC after a read or write operation is completed, there is no change in the functionality of this bit even when the PHY is not present."]
    #[inline(always)]
    #[must_use]
    pub fn gb(&mut self) -> GB_W<0> {
        GB_W::new(self)
    }
    #[doc = "Bit 1 - GMII Write When set, this bit indicates to the PHY or RevMII that this is a Write operation using the GMII Data register. If this bit is not set, it indicates that this is a Read operation, that is, placing the data in the GMII Data register."]
    #[inline(always)]
    #[must_use]
    pub fn gw(&mut self) -> GW_W<1> {
        GW_W::new(self)
    }
    #[doc = "Bits 2:5 - CSR Clock Range The CSR Clock Range selection determines the frequency of the MDC clock according to the CSR clock frequency used in your design. The CSR clock corresponding to different GMAC configurations is given in Table 9-2 on page 564. The suggested range of CSR clock frequency applicable for each value (when Bit\\[5\\]
= 0) ensures that the MDC clock is approximately between the frequency range 1.0 MHz–2.5 MHz. - 0000: The CSR clock frequency is 60–100 MHz and the MDC clock frequency is CSR clock/42. - 0001: The CSR clock frequency is 100–150 MHz and the MDC clock frequency is CSR clock/62. - 0010: The CSR clock frequency is 20–35 MHz and the MDC clock frequency is CSR clock/16. - 0011: The CSR clock frequency is 35–60 MHz and the MDC clock frequency is CSR clock/26. - 0100: The CSR clock frequency is 150–250 MHz and the MDC clock frequency is CSR clock/102. - 0101: The CSR clock frequency is 250–300 MHz and the MDC clock is CSR clock/124. - 0110, 0111: Reserved When Bit 5 is set, you can achieve higher frequency of the MDC clock than the frequency limit of 2.5 MHz (specified in the IEEE Std 802.3) and program a clock divider of lower value. For example, when CSR clock is of 100 MHz frequency and you program these bits as 1010, then the resultant MDC clock is of 12.5 MHz which is outside the limit of IEEE 802.3 specified range. Program the following values only if the interfacing chips support faster MDC clocks. - 1000: CSR clock/4 - 1001: CSR clock/6 - 1010: CSR clock/8 - 1011: CSR clock/10 - 1100: CSR clock/12 - 1101: CSR clock/14 - 1110: CSR clock/16 - 1111: CSR clock/18 These bits are not used for accessing RevMII. These bits are read-only if the RevMII interface is selected as single PHY interface."]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<2> {
        CR_W::new(self)
    }
    #[doc = "Bits 6:10 - GMII Register These bits select the desired GMII register in the selected PHY device. For RevMII, these bits select the desired CSR register in the RevMII Registers set."]
    #[inline(always)]
    #[must_use]
    pub fn gr(&mut self) -> GR_W<6> {
        GR_W::new(self)
    }
    #[doc = "Bits 11:15 - Physical Layer Address This field indicates which of the 32 possible PHY devices are being accessed. For RevMII, this field gives the PHY Address of the RevMII module."]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<11> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GMII Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmii_addr](index.html) module"]
pub struct GMII_ADDR_SPEC;
impl crate::RegisterSpec for GMII_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmii_addr::R](R) reader structure"]
impl crate::Readable for GMII_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gmii_addr::W](W) writer structure"]
impl crate::Writable for GMII_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GMII_ADDR to value 0"]
impl crate::Resettable for GMII_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
