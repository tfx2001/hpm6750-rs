#[doc = "Register `RUN` reader"]
pub struct R(crate::R<RUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RUN` writer"]
pub struct W(crate::W<RUN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RUN_SPEC>;
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
impl From<crate::W<RUN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RUN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDM_EN` reader - Asserted to enable the module"]
pub type PDM_EN_R = crate::BitReader<bool>;
#[doc = "Field `PDM_EN` writer - Asserted to enable the module"]
pub type PDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RUN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Asserted to enable the module"]
    #[inline(always)]
    pub fn pdm_en(&self) -> PDM_EN_R {
        PDM_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asserted to enable the module"]
    #[inline(always)]
    #[must_use]
    pub fn pdm_en(&mut self) -> PDM_EN_W<0> {
        PDM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Run Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [run](index.html) module"]
pub struct RUN_SPEC;
impl crate::RegisterSpec for RUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [run::R](R) reader structure"]
impl crate::Readable for RUN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [run::W](W) writer structure"]
impl crate::Writable for RUN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RUN to value 0"]
impl crate::Resettable for RUN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
