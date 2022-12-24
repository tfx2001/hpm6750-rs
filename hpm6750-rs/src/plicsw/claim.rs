#[doc = "Register `CLAIM` reader"]
pub struct R(crate::R<CLAIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIM` writer"]
pub struct W(crate::W<CLAIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIM_SPEC>;
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
impl From<crate::W<CLAIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ID` reader - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
pub type INTERRUPT_ID_R = crate::BitReader<bool>;
#[doc = "Field `INTERRUPT_ID` writer - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
pub type INTERRUPT_ID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLAIM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<0> {
        INTERRUPT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Claim and complete.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claim](index.html) module"]
pub struct CLAIM_SPEC;
impl crate::RegisterSpec for CLAIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claim::R](R) reader structure"]
impl crate::Readable for CLAIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claim::W](W) writer structure"]
impl crate::Writable for CLAIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLAIM to value 0"]
impl crate::Resettable for CLAIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
