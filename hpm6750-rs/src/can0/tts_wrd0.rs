#[doc = "Register `TTS_WRD0` reader"]
pub struct R(crate::R<TTS_WRD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTS_WRD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTS_WRD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTS_WRD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TTS_WRD0` reader - transmission time stamp, word 0, LSB 32bit"]
pub type TTS_WRD0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - transmission time stamp, word 0, LSB 32bit"]
    #[inline(always)]
    pub fn tts_wrd0(&self) -> TTS_WRD0_R {
        TTS_WRD0_R::new(self.bits)
    }
}
#[doc = "transmission time stamp, LSB 32bit\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tts_wrd0](index.html) module"]
pub struct TTS_WRD0_SPEC;
impl crate::RegisterSpec for TTS_WRD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tts_wrd0::R](R) reader structure"]
impl crate::Readable for TTS_WRD0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TTS_WRD0 to value 0"]
impl crate::Resettable for TTS_WRD0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
