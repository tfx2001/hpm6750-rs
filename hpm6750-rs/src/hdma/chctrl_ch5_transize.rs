#[doc = "Register `CHCTRL_CH5_TRANSIZE` reader"]
pub struct R(crate::R<CHCTRL_CH5_TRANSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRL_CH5_TRANSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRL_CH5_TRANSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRL_CH5_TRANSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRL_CH5_TRANSIZE` writer"]
pub struct W(crate::W<CHCTRL_CH5_TRANSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRL_CH5_TRANSIZE_SPEC>;
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
impl From<crate::W<CHCTRL_CH5_TRANSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRL_CH5_TRANSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANSIZE` reader - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
pub type TRANSIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TRANSIZE` writer - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
pub type TRANSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRL_CH5_TRANSIZE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    #[inline(always)]
    pub fn transize(&self) -> TRANSIZE_R {
        TRANSIZE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    #[inline(always)]
    #[must_use]
    pub fn transize(&mut self) -> TRANSIZE_W<0> {
        TRANSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrl_ch5_transize](index.html) module"]
pub struct CHCTRL_CH5_TRANSIZE_SPEC;
impl crate::RegisterSpec for CHCTRL_CH5_TRANSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrl_ch5_transize::R](R) reader structure"]
impl crate::Readable for CHCTRL_CH5_TRANSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrl_ch5_transize::W](W) writer structure"]
impl crate::Writable for CHCTRL_CH5_TRANSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRL_CH5_TRANSIZE to value 0"]
impl crate::Resettable for CHCTRL_CH5_TRANSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
