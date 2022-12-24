#[doc = "Register `CQTCLR` reader"]
pub struct R(crate::R<CQTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQTCLR` writer"]
pub struct W(crate::W<CQTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQTCLR_SPEC>;
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
impl From<crate::W<CQTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCLR` reader - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is leared to verify that a clear operation was done."]
pub type TCLR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TCLR` writer - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is leared to verify that a clear operation was done."]
pub type TCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CQTCLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is leared to verify that a clear operation was done."]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is leared to verify that a clear operation was done."]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<0> {
        TCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqtclr](index.html) module"]
pub struct CQTCLR_SPEC;
impl crate::RegisterSpec for CQTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqtclr::R](R) reader structure"]
impl crate::Readable for CQTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqtclr::W](W) writer structure"]
impl crate::Writable for CQTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQTCLR to value 0"]
impl crate::Resettable for CQTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
