#[doc = "Register `CURR_CAPABILITIES2` reader"]
pub struct R(crate::R<CURR_CAPABILITIES2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURR_CAPABILITIES2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURR_CAPABILITIES2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURR_CAPABILITIES2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAX_CUR_VDD2_18V` reader - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
pub type MAX_CUR_VDD2_18V_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_vdd2_18v(&self) -> MAX_CUR_VDD2_18V_R {
        MAX_CUR_VDD2_18V_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curr_capabilities2](index.html) module"]
pub struct CURR_CAPABILITIES2_SPEC;
impl crate::RegisterSpec for CURR_CAPABILITIES2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [curr_capabilities2::R](R) reader structure"]
impl crate::Readable for CURR_CAPABILITIES2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURR_CAPABILITIES2 to value 0"]
impl crate::Resettable for CURR_CAPABILITIES2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
