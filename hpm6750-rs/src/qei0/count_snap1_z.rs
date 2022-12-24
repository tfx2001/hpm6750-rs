#[doc = "Register `COUNT_SNAP1_Z` reader"]
pub struct R(crate::R<COUNT_SNAP1_Z_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_SNAP1_Z_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_SNAP1_Z_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_SNAP1_Z_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT_SNAP1_Z` writer"]
pub struct W(crate::W<COUNT_SNAP1_Z_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT_SNAP1_Z_SPEC>;
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
impl From<crate::W<COUNT_SNAP1_Z_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT_SNAP1_Z_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZCNT` reader - zcnt value"]
pub type ZCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ZCNT` writer - zcnt value"]
pub type ZCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNT_SNAP1_Z_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - zcnt value"]
    #[inline(always)]
    pub fn zcnt(&self) -> ZCNT_R {
        ZCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - zcnt value"]
    #[inline(always)]
    #[must_use]
    pub fn zcnt(&mut self) -> ZCNT_W<0> {
        ZCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Z snap register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_snap1_z](index.html) module"]
pub struct COUNT_SNAP1_Z_SPEC;
impl crate::RegisterSpec for COUNT_SNAP1_Z_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_snap1_z::R](R) reader structure"]
impl crate::Readable for COUNT_SNAP1_Z_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count_snap1_z::W](W) writer structure"]
impl crate::Writable for COUNT_SNAP1_Z_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT_SNAP1_Z to value 0"]
impl crate::Resettable for COUNT_SNAP1_Z_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
