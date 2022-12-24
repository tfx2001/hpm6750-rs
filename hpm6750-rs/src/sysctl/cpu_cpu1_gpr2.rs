#[doc = "Register `CPU_CPU1_GPR2` reader"]
pub struct R(crate::R<CPU_CPU1_GPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CPU1_GPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CPU1_GPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CPU1_GPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CPU1_GPR2` writer"]
pub struct W(crate::W<CPU_CPU1_GPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CPU1_GPR2_SPEC>;
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
impl From<crate::W<CPU_CPU1_GPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CPU1_GPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPR` reader - register for software to handle resume, can save resume address or status"]
pub type GPR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPR` writer - register for software to handle resume, can save resume address or status"]
pub type GPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU_CPU1_GPR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - register for software to handle resume, can save resume address or status"]
    #[inline(always)]
    pub fn gpr(&self) -> GPR_R {
        GPR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - register for software to handle resume, can save resume address or status"]
    #[inline(always)]
    #[must_use]
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
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_cpu1_gpr2](index.html) module"]
pub struct CPU_CPU1_GPR2_SPEC;
impl crate::RegisterSpec for CPU_CPU1_GPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_cpu1_gpr2::R](R) reader structure"]
impl crate::Readable for CPU_CPU1_GPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_cpu1_gpr2::W](W) writer structure"]
impl crate::Writable for CPU_CPU1_GPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_CPU1_GPR2 to value 0"]
impl crate::Resettable for CPU_CPU1_GPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
