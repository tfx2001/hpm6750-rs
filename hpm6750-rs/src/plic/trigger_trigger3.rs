#[doc = "Register `TRIGGER_TRIGGER3` reader"]
pub struct R(crate::R<TRIGGER_TRIGGER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_TRIGGER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_TRIGGER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_TRIGGER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERRUPT` reader - The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt"]
pub type INTERRUPT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The interrupt trigger type of interrupt sources. Every interrupt source occupies 1 bit. 0: Level-triggered interrupt 1: Edge-triggered interrupt"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits)
    }
}
#[doc = "Trigger type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger_trigger3](index.html) module"]
pub struct TRIGGER_TRIGGER3_SPEC;
impl crate::RegisterSpec for TRIGGER_TRIGGER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger_trigger3::R](R) reader structure"]
impl crate::Readable for TRIGGER_TRIGGER3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TRIGGER_TRIGGER3 to value 0"]
impl crate::Resettable for TRIGGER_TRIGGER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
