#[doc = "Register `CHCTRL_CH1_SRCADDRH` reader"]
pub struct R(crate::R<CHCTRL_CH1_SRCADDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRL_CH1_SRCADDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRL_CH1_SRCADDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRL_CH1_SRCADDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRL_CH1_SRCADDRH` writer"]
pub struct W(crate::W<CHCTRL_CH1_SRCADDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRL_CH1_SRCADDRH_SPEC>;
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
impl From<crate::W<CHCTRL_CH1_SRCADDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRL_CH1_SRCADDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCADDRH` reader - High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
pub type SRCADDRH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRCADDRH` writer - High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
pub type SRCADDRH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRL_CH1_SRCADDRH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    pub fn srcaddrh(&self) -> SRCADDRH_R {
        SRCADDRH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    pub fn srcaddrh(&mut self) -> SRCADDRH_W<0> {
        SRCADDRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Source Address High Part Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrl_ch1_srcaddrh](index.html) module"]
pub struct CHCTRL_CH1_SRCADDRH_SPEC;
impl crate::RegisterSpec for CHCTRL_CH1_SRCADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrl_ch1_srcaddrh::R](R) reader structure"]
impl crate::Readable for CHCTRL_CH1_SRCADDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrl_ch1_srcaddrh::W](W) writer structure"]
impl crate::Writable for CHCTRL_CH1_SRCADDRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTRL_CH1_SRCADDRH to value 0x01"]
impl crate::Resettable for CHCTRL_CH1_SRCADDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
