#[doc = "Register `PHIDX` reader"]
pub struct R(crate::R<PHIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHIDX` writer"]
pub struct W(crate::W<PHIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHIDX_SPEC>;
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
impl From<crate::W<PHIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHIDX` reader - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
pub type PHIDX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PHIDX` writer - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
pub type PHIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHIDX_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
    #[inline(always)]
    pub fn phidx(&self) -> PHIDX_R {
        PHIDX_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
    #[inline(always)]
    pub fn phidx(&mut self) -> PHIDX_W<0> {
        PHIDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phidx](index.html) module"]
pub struct PHIDX_SPEC;
impl crate::RegisterSpec for PHIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phidx::R](R) reader structure"]
impl crate::Readable for PHIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phidx::W](W) writer structure"]
impl crate::Writable for PHIDX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHIDX to value 0"]
impl crate::Resettable for PHIDX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
