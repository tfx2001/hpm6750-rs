#[doc = "Register `CHCTRL_CH1_LLPOINTERH` reader"]
pub struct R(crate::R<CHCTRL_CH1_LLPOINTERH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRL_CH1_LLPOINTERH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRL_CH1_LLPOINTERH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRL_CH1_LLPOINTERH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRL_CH1_LLPOINTERH` writer"]
pub struct W(crate::W<CHCTRL_CH1_LLPOINTERH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRL_CH1_LLPOINTERH_SPEC>;
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
impl From<crate::W<CHCTRL_CH1_LLPOINTERH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRL_CH1_LLPOINTERH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLPOINTERH` reader - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
pub type LLPOINTERH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LLPOINTERH` writer - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
pub type LLPOINTERH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRL_CH1_LLPOINTERH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    pub fn llpointerh(&self) -> LLPOINTERH_R {
        LLPOINTERH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn llpointerh(&mut self) -> LLPOINTERH_W<0> {
        LLPOINTERH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Linked List Pointer High Part Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrl_ch1_llpointerh](index.html) module"]
pub struct CHCTRL_CH1_LLPOINTERH_SPEC;
impl crate::RegisterSpec for CHCTRL_CH1_LLPOINTERH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrl_ch1_llpointerh::R](R) reader structure"]
impl crate::Readable for CHCTRL_CH1_LLPOINTERH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrl_ch1_llpointerh::W](W) writer structure"]
impl crate::Writable for CHCTRL_CH1_LLPOINTERH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRL_CH1_LLPOINTERH to value 0"]
impl crate::Resettable for CHCTRL_CH1_LLPOINTERH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
