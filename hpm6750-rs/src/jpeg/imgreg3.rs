#[doc = "Register `IMGREG3` reader"]
pub struct R(crate::R<IMGREG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMGREG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMGREG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMGREG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMGREG3` writer"]
pub struct W(crate::W<IMGREG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMGREG3_SPEC>;
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
impl From<crate::W<IMGREG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMGREG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRST` reader - Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
pub type NRST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NRST` writer - Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
pub type NRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMGREG3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
    #[inline(always)]
    pub fn nrst(&self) -> NRST_R {
        NRST_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
    #[inline(always)]
    #[must_use]
    pub fn nrst(&mut self) -> NRST_W<0> {
        NRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Image Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imgreg3](index.html) module"]
pub struct IMGREG3_SPEC;
impl crate::RegisterSpec for IMGREG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imgreg3::R](R) reader structure"]
impl crate::Readable for IMGREG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imgreg3::W](W) writer structure"]
impl crate::Writable for IMGREG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMGREG3 to value 0"]
impl crate::Resettable for IMGREG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
