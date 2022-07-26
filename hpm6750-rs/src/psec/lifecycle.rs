#[doc = "Register `LIFECYCLE` reader"]
pub struct R(crate::R<LIFECYCLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIFECYCLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIFECYCLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIFECYCLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LIFECYCLE` reader - lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow"]
pub type LIFECYCLE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow"]
    #[inline(always)]
    pub fn lifecycle(&self) -> LIFECYCLE_R {
        LIFECYCLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Lifecycle\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lifecycle](index.html) module"]
pub struct LIFECYCLE_SPEC;
impl crate::RegisterSpec for LIFECYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lifecycle::R](R) reader structure"]
impl crate::Readable for LIFECYCLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LIFECYCLE to value 0"]
impl crate::Resettable for LIFECYCLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
