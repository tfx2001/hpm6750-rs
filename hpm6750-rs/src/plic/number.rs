#[doc = "Register `NUMBER` reader"]
pub struct R(crate::R<NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_INTERRUPT` reader - The number of supported interrupt sources"]
pub type NUM_INTERRUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NUM_TARGET` reader - The number of supported targets"]
pub type NUM_TARGET_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The number of supported interrupt sources"]
    #[inline(always)]
    pub fn num_interrupt(&self) -> NUM_INTERRUPT_R {
        NUM_INTERRUPT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The number of supported targets"]
    #[inline(always)]
    pub fn num_target(&self) -> NUM_TARGET_R {
        NUM_TARGET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Number of supported interrupt sources and targets\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [number](index.html) module"]
pub struct NUMBER_SPEC;
impl crate::RegisterSpec for NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [number::R](R) reader structure"]
impl crate::Readable for NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NUMBER to value 0"]
impl crate::Resettable for NUMBER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
