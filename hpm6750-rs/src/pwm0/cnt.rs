#[doc = "Register `CNT` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XCNT` reader - current extended counter value"]
pub type XCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNT` reader - current clock counter value"]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - current extended counter value"]
    #[inline(always)]
    pub fn xcnt(&self) -> XCNT_R {
        XCNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 4:27 - current clock counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 4) & 0x00ff_ffff) as u32)
    }
}
#[doc = "Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
