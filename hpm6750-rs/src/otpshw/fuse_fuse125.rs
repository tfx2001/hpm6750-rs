#[doc = "Register `FUSE_FUSE125` reader"]
pub struct R(crate::R<FUSE_FUSE125_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUSE_FUSE125_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUSE_FUSE125_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUSE_FUSE125_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUSE_FUSE125` writer"]
pub struct W(crate::W<FUSE_FUSE125_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUSE_FUSE125_SPEC>;
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
impl From<crate::W<FUSE_FUSE125_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUSE_FUSE125_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUSE` reader - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
pub type FUSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FUSE` writer - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
pub type FUSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUSE_FUSE125_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
    #[inline(always)]
    pub fn fuse(&self) -> FUSE_R {
        FUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fuse array, valid in PMIC part only read operation will read out value in fuse array write operation will update fuse array value(please make sure fuse is unlocked and 2.5V power is ready)"]
    #[inline(always)]
    #[must_use]
    pub fn fuse(&mut self) -> FUSE_W<0> {
        FUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fuse Array\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fuse_fuse125](index.html) module"]
pub struct FUSE_FUSE125_SPEC;
impl crate::RegisterSpec for FUSE_FUSE125_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fuse_fuse125::R](R) reader structure"]
impl crate::Readable for FUSE_FUSE125_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fuse_fuse125::W](W) writer structure"]
impl crate::Writable for FUSE_FUSE125_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUSE_FUSE125 to value 0"]
impl crate::Resettable for FUSE_FUSE125_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
