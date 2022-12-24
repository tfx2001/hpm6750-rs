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
#[doc = "Field `BATT_ESC_SEC` reader - BATT is escalting ssecure event"]
pub type BATT_ESC_SEC_R = crate::BitReader<bool>;
#[doc = "Field `BATT_ESC_NSC` reader - BATT is escalating non-secure event"]
pub type BATT_ESC_NSC_R = crate::BitReader<bool>;
#[doc = "Field `EVENT` reader - local event statue, each bit represents one security event"]
pub type EVENT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - BATT is escalting ssecure event"]
    #[inline(always)]
    pub fn batt_esc_sec(&self) -> BATT_ESC_SEC_R {
        BATT_ESC_SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BATT is escalating non-secure event"]
    #[inline(always)]
    pub fn batt_esc_nsc(&self) -> BATT_ESC_NSC_R {
        BATT_ESC_NSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31 - local event statue, each bit represents one security event"]
    #[inline(always)]
    pub fn event(&self) -> EVENT_R {
        EVENT_R::new(((self.bits >> 16) & 0xffff) as u16)
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
    const RESET_VALUE: Self::Ux = 0;
}
