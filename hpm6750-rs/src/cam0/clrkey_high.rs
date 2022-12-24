#[doc = "Register `CLRKEY_HIGH` reader"]
pub struct R(crate::R<CLRKEY_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRKEY_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRKEY_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRKEY_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRKEY_HIGH` writer"]
pub struct W(crate::W<CLRKEY_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRKEY_HIGH_SPEC>;
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
impl From<crate::W<CLRKEY_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRKEY_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMIT` reader - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
pub type LIMIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LIMIT` writer - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLRKEY_HIGH_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<0> {
        LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Color Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrkey_high](index.html) module"]
pub struct CLRKEY_HIGH_SPEC;
impl crate::RegisterSpec for CLRKEY_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrkey_high::R](R) reader structure"]
impl crate::Readable for CLRKEY_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrkey_high::W](W) writer structure"]
impl crate::Writable for CLRKEY_HIGH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLRKEY_HIGH to value 0"]
impl crate::Resettable for CLRKEY_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
