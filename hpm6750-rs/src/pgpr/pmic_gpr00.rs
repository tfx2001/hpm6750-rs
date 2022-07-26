#[doc = "Register `PMIC_GPR00` reader"]
pub struct R(crate::R<PMIC_GPR00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMIC_GPR00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMIC_GPR00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMIC_GPR00_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMIC_GPR00` writer"]
pub struct W(crate::W<PMIC_GPR00_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMIC_GPR00_SPEC>;
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
impl From<crate::W<PMIC_GPR00_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMIC_GPR00_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPR` reader - Generic control"]
pub type GPR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPR` writer - Generic control"]
pub type GPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMIC_GPR00_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Generic control"]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Generic control"]
    #[inline(always)]
    pub fn gpr(&mut self) -> GPR_W<0> {
        GPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmic_gpr00](index.html) module"]
pub struct PMIC_GPR00_SPEC;
impl crate::RegisterSpec for PMIC_GPR00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmic_gpr00::R](R) reader structure"]
impl crate::Readable for PMIC_GPR00_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmic_gpr00::W](W) writer structure"]
impl crate::Writable for PMIC_GPR00_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMIC_GPR00 to value 0"]
impl crate::Resettable for PMIC_GPR00_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
