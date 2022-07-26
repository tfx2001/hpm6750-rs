#[doc = "Register `CHANNEL_CH2_CNT` reader"]
pub struct R(crate::R<CHANNEL_CH2_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH2_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH2_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH2_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNTER` reader - 32 bit counter value"]
pub type COUNTER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit counter value"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
#[doc = "Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch2_cnt](index.html) module"]
pub struct CHANNEL_CH2_CNT_SPEC;
impl crate::RegisterSpec for CHANNEL_CH2_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch2_cnt::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH2_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHANNEL_CH2_CNT to value 0"]
impl crate::Resettable for CHANNEL_CH2_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
