#[doc = "Register `CLRKEY_LOW` reader"]
pub struct R(crate::R<CLRKEY_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRKEY_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRKEY_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRKEY_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRKEY_LOW` writer"]
pub struct W(crate::W<CLRKEY_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRKEY_LOW_SPEC>;
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
impl From<crate::W<CLRKEY_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRKEY_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMIT` reader - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
pub type LIMIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LIMIT` writer - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLRKEY_LOW_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    #[inline(always)]
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
#[doc = "Low Color Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrkey_low](index.html) module"]
pub struct CLRKEY_LOW_SPEC;
impl crate::RegisterSpec for CLRKEY_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrkey_low::R](R) reader structure"]
impl crate::Readable for CLRKEY_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrkey_low::W](W) writer structure"]
impl crate::Writable for CLRKEY_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRKEY_LOW to value 0"]
impl crate::Resettable for CLRKEY_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
