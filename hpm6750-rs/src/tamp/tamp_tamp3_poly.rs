#[doc = "Register `TAMP_TAMP3_POLY` reader"]
pub struct R(crate::R<TAMP_TAMP3_POLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_TAMP3_POLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_TAMP3_POLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_TAMP3_POLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_TAMP3_POLY` writer"]
pub struct W(crate::W<TAMP_TAMP3_POLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_TAMP3_POLY_SPEC>;
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
impl From<crate::W<TAMP_TAMP3_POLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_TAMP3_POLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLY` reader - tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
pub type POLY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `POLY` writer - tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
pub type POLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TAMP_TAMP3_POLY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> POLY_W<0> {
        POLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper3 Polynomial of LFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_tamp3_poly](index.html) module"]
pub struct TAMP_TAMP3_POLY_SPEC;
impl crate::RegisterSpec for TAMP_TAMP3_POLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_tamp3_poly::R](R) reader structure"]
impl crate::Readable for TAMP_TAMP3_POLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_tamp3_poly::W](W) writer structure"]
impl crate::Writable for TAMP_TAMP3_POLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_TAMP3_POLY to value 0"]
impl crate::Resettable for TAMP_TAMP3_POLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
