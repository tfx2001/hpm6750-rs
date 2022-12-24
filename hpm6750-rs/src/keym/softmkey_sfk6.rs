#[doc = "Register `SOFTMKEY_SFK6` reader"]
pub struct R(crate::R<SOFTMKEY_SFK6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTMKEY_SFK6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTMKEY_SFK6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTMKEY_SFK6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTMKEY_SFK6` writer"]
pub struct W(crate::W<SOFTMKEY_SFK6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTMKEY_SFK6_SPEC>;
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
impl From<crate::W<SOFTMKEY_SFK6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTMKEY_SFK6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOFTMKEY_SFK6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software set symmetric key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softmkey_sfk6](index.html) module"]
pub struct SOFTMKEY_SFK6_SPEC;
impl crate::RegisterSpec for SOFTMKEY_SFK6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softmkey_sfk6::R](R) reader structure"]
impl crate::Readable for SOFTMKEY_SFK6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softmkey_sfk6::W](W) writer structure"]
impl crate::Writable for SOFTMKEY_SFK6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFTMKEY_SFK6 to value 0"]
impl crate::Resettable for SOFTMKEY_SFK6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
