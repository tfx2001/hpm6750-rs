#[doc = "Register `CPU_CPU1_ENABLE0` reader"]
pub struct R(crate::R<CPU_CPU1_ENABLE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CPU1_ENABLE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CPU1_ENABLE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CPU1_ENABLE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CPU1_ENABLE0` writer"]
pub struct W(crate::W<CPU_CPU1_ENABLE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CPU1_ENABLE0_SPEC>;
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
impl From<crate::W<CPU_CPU1_ENABLE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CPU1_ENABLE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - IRQ wakeup enable"]
pub type ENABLE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENABLE` writer - IRQ wakeup enable"]
pub type ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_CPU1_ENABLE0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IRQ wakeup enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IRQ wakeup enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_cpu1_enable0](index.html) module"]
pub struct CPU_CPU1_ENABLE0_SPEC;
impl crate::RegisterSpec for CPU_CPU1_ENABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_cpu1_enable0::R](R) reader structure"]
impl crate::Readable for CPU_CPU1_ENABLE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_cpu1_enable0::W](W) writer structure"]
impl crate::Writable for CPU_CPU1_ENABLE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_CPU1_ENABLE0 to value 0"]
impl crate::Resettable for CPU_CPU1_ENABLE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
