#[doc = "Register `L3_L4_CFG_3_L4_ADDR` reader"]
pub struct R(crate::R<L3_L4_CFG_3_L4_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L3_L4_CFG_3_L4_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L3_L4_CFG_3_L4_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L3_L4_CFG_3_L4_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L3_L4_CFG_3_L4_ADDR` writer"]
pub struct W(crate::W<L3_L4_CFG_3_L4_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L3_L4_CFG_3_L4_ADDR_SPEC>;
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
impl From<crate::W<L3_L4_CFG_3_L4_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L3_L4_CFG_3_L4_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L4SP0` reader - Layer 4 Source Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames."]
pub type L4SP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L4SP0` writer - Layer 4 Source Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames."]
pub type L4SP0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L3_L4_CFG_3_L4_ADDR_SPEC, u16, u16, 16, O>;
#[doc = "Field `L4DP0` reader - Layer 4 Destination Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames."]
pub type L4DP0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L4DP0` writer - Layer 4 Destination Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames."]
pub type L4DP0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L3_L4_CFG_3_L4_ADDR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames."]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames."]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Layer 4 Source Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 frames."]
    #[inline(always)]
    #[must_use]
    pub fn l4sp0(&mut self) -> L4SP0_W<0> {
        L4SP0_W::new(self)
    }
    #[doc = "Bits 16:31 - Layer 4 Destination Port Number Field When Bit 16 (L4PEN0) is reset and Bit 20 (L4DPM0) is set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 frames. When Bit 16 (L4PEN0) and Bit 20 (L4DPM0) are set in Register 256 (Layer 3 and Layer 4 Control Register 0), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 frames."]
    #[inline(always)]
    #[must_use]
    pub fn l4dp0(&mut self) -> L4DP0_W<16> {
        L4DP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 4 Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l3_l4_cfg_3_l4_addr](index.html) module"]
pub struct L3_L4_CFG_3_L4_ADDR_SPEC;
impl crate::RegisterSpec for L3_L4_CFG_3_L4_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l3_l4_cfg_3_l4_addr::R](R) reader structure"]
impl crate::Readable for L3_L4_CFG_3_L4_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l3_l4_cfg_3_l4_addr::W](W) writer structure"]
impl crate::Writable for L3_L4_CFG_3_L4_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L3_L4_CFG_3_L4_ADDR to value 0"]
impl crate::Resettable for L3_L4_CFG_3_L4_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
