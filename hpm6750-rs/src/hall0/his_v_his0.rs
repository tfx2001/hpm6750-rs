#[doc = "Register `HIS_V_HIS0` reader"]
pub struct R(crate::R<HIS_V_HIS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIS_V_HIS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIS_V_HIS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIS_V_HIS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UHIS0` reader - copy of ucnt when u signal transition from 0 to 1"]
pub type UHIS0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - copy of ucnt when u signal transition from 0 to 1"]
    #[inline(always)]
    pub fn uhis0(&self) -> UHIS0_R {
        UHIS0_R::new(self.bits)
    }
}
#[doc = "V histroy register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [his_v_his0](index.html) module"]
pub struct HIS_V_HIS0_SPEC;
impl crate::RegisterSpec for HIS_V_HIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [his_v_his0::R](R) reader structure"]
impl crate::Readable for HIS_V_HIS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HIS_V_HIS0 to value 0"]
impl crate::Resettable for HIS_V_HIS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
