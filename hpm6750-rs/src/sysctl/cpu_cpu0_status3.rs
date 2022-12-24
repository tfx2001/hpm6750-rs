#[doc = "Register `CPU_CPU0_STATUS3` reader"]
pub struct R(crate::R<CPU_CPU0_STATUS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CPU0_STATUS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CPU0_STATUS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CPU0_STATUS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - IRQ values"]
pub type STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IRQ values"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_cpu0_status3](index.html) module"]
pub struct CPU_CPU0_STATUS3_SPEC;
impl crate::RegisterSpec for CPU_CPU0_STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_cpu0_status3::R](R) reader structure"]
impl crate::Readable for CPU_CPU0_STATUS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_CPU0_STATUS3 to value 0"]
impl crate::Resettable for CPU_CPU0_STATUS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
