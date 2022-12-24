#[doc = "Register `SHADOW_SHADOW016` reader"]
pub struct R(crate::R<SHADOW_SHADOW016_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHADOW_SHADOW016_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHADOW_SHADOW016_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHADOW_SHADOW016_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHADOW_SHADOW016` writer"]
pub struct W(crate::W<SHADOW_SHADOW016_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHADOW_SHADOW016_SPEC>;
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
impl From<crate::W<SHADOW_SHADOW016_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHADOW_SHADOW016_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHADOW` reader - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
pub type SHADOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHADOW` writer - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
pub type SHADOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHADOW_SHADOW016_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
    #[inline(always)]
    pub fn shadow(&self) -> SHADOW_R {
        SHADOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - shadow register of fuse for pmic area for PMIC, index valid for 0-15, for SOC index valid for 16-128"]
    #[inline(always)]
    #[must_use]
    pub fn shadow(&mut self) -> SHADOW_W<0> {
        SHADOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fuse shadow registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shadow_shadow016](index.html) module"]
pub struct SHADOW_SHADOW016_SPEC;
impl crate::RegisterSpec for SHADOW_SHADOW016_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shadow_shadow016::R](R) reader structure"]
impl crate::Readable for SHADOW_SHADOW016_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shadow_shadow016::W](W) writer structure"]
impl crate::Writable for SHADOW_SHADOW016_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHADOW_SHADOW016 to value 0"]
impl crate::Resettable for SHADOW_SHADOW016_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
