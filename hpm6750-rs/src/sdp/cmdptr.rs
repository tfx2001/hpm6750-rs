#[doc = "Register `CMDPTR` reader"]
pub struct R(crate::R<CMDPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDPTR` writer"]
pub struct W(crate::W<CMDPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDPTR_SPEC>;
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
impl From<crate::W<CMDPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDPTR` reader - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
pub type CMDPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMDPTR` writer - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
pub type CMDPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
    #[inline(always)]
    pub fn cmdptr(&self) -> CMDPTR_R {
        CMDPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
    #[inline(always)]
    #[must_use]
    pub fn cmdptr(&mut self) -> CMDPTR_W<0> {
        CMDPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdptr](index.html) module"]
pub struct CMDPTR_SPEC;
impl crate::RegisterSpec for CMDPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdptr::R](R) reader structure"]
impl crate::Readable for CMDPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdptr::W](W) writer structure"]
impl crate::Writable for CMDPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDPTR to value 0"]
impl crate::Resettable for CMDPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
