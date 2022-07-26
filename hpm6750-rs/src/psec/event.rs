#[doc = "Register `EVENT` reader"]
pub struct R(crate::R<EVENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVENT` reader - local event statue, each bit represents one security event"]
pub type EVENT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PMIC_ESC_NSC` reader - PMIC is escalating non-secure event"]
pub type PMIC_ESC_NSC_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_ESC_SEC` reader - PMIC is escalting secure event"]
pub type PMIC_ESC_SEC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 16:31 - local event statue, each bit represents one security event"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 3 - PMIC is escalating non-secure event"]
    #[inline(always)]
    pub fn pmic_esc_nsc(&self) -> PMIC_ESC_NSC_R {
        PMIC_ESC_NSC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PMIC is escalting secure event"]
    #[inline(always)]
    pub fn pmic_esc_sec(&self) -> PMIC_ESC_SEC_R {
        PMIC_ESC_SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Event and escalate status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [event](index.html) module"]
pub struct EVENT_SPEC;
impl crate::RegisterSpec for EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [event::R](R) reader structure"]
impl crate::Readable for EVENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EVENT to value 0"]
impl crate::Resettable for EVENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
