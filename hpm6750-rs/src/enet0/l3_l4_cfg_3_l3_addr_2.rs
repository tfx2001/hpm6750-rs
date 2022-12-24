#[doc = "Register `L3_L4_CFG_3_L3_ADDR_2` reader"]
pub struct R(crate::R<L3_L4_CFG_3_L3_ADDR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L3_L4_CFG_3_L3_ADDR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L3_L4_CFG_3_L3_ADDR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L3_L4_CFG_3_L3_ADDR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L3_L4_CFG_3_L3_ADDR_2` writer"]
pub struct W(crate::W<L3_L4_CFG_3_L3_ADDR_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L3_L4_CFG_3_L3_ADDR_2_SPEC>;
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
impl From<crate::W<L3_L4_CFG_3_L3_ADDR_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L3_L4_CFG_3_L3_ADDR_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A20` reader - Layer 3 Address 2 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[95:64\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains value to be matched with Bits \\[95:64\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
pub type L3A20_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L3A20` writer - Layer 3 Address 2 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[95:64\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains value to be matched with Bits \\[95:64\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
pub type L3A20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L3_L4_CFG_3_L3_ADDR_2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[95:64\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains value to be matched with Bits \\[95:64\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field When Bit 0 (L3PEN0) and Bit 2 (L3SAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with Bits \\[95:64\\]
of the IP Source Address field in the IPv6 frames. When Bit 0 (L3PEN0) and Bit 4 (L3DAM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains value to be matched with Bits \\[95:64\\]
of the IP Destination Address field in the IPv6 frames. When Bit 0 (L3PEN0) is reset in Register 256 (Layer 3 and Layer 4 Control Register 0), this register is not used."]
    #[inline(always)]
    #[must_use]
    pub fn l3a20(&mut self) -> L3A20_W<0> {
        L3A20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 3 Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l3_l4_cfg_3_l3_addr_2](index.html) module"]
pub struct L3_L4_CFG_3_L3_ADDR_2_SPEC;
impl crate::RegisterSpec for L3_L4_CFG_3_L3_ADDR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l3_l4_cfg_3_l3_addr_2::R](R) reader structure"]
impl crate::Readable for L3_L4_CFG_3_L3_ADDR_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l3_l4_cfg_3_l3_addr_2::W](W) writer structure"]
impl crate::Writable for L3_L4_CFG_3_L3_ADDR_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L3_L4_CFG_3_L3_ADDR_2 to value 0"]
impl crate::Resettable for L3_L4_CFG_3_L3_ADDR_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
