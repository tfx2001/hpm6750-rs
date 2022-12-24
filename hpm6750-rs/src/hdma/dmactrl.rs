#[doc = "Register `DMACTRL` writer"]
pub struct W(crate::W<DMACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTRL_SPEC>;
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
impl From<crate::W<DMACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` writer - Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactrl](index.html) module"]
pub struct DMACTRL_SPEC;
impl crate::RegisterSpec for DMACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmactrl::W](W) writer structure"]
impl crate::Writable for DMACTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DMACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
