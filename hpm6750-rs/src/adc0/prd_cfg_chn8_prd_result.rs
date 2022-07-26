#[doc = "Register `PRD_CFG_CHN8_PRD_RESULT` reader"]
pub struct R(crate::R<PRD_CFG_CHN8_PRD_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRD_CFG_CHN8_PRD_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRD_CFG_CHN8_PRD_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRD_CFG_CHN8_PRD_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN_RESULT` reader - adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
pub type CHAN_RESULT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    #[inline(always)]
    pub fn chan_result(&self) -> CHAN_RESULT_R {
        CHAN_RESULT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prd_cfg_chn8_prd_result](index.html) module"]
pub struct PRD_CFG_CHN8_PRD_RESULT_SPEC;
impl crate::RegisterSpec for PRD_CFG_CHN8_PRD_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prd_cfg_chn8_prd_result::R](R) reader structure"]
impl crate::Readable for PRD_CFG_CHN8_PRD_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRD_CFG_CHN8_PRD_RESULT to value 0"]
impl crate::Resettable for PRD_CFG_CHN8_PRD_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
