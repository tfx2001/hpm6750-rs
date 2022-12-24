#[doc = "Register `CURR_CAPABILITIES1` reader"]
pub struct R(crate::R<CURR_CAPABILITIES1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURR_CAPABILITIES1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURR_CAPABILITIES1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURR_CAPABILITIES1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAX_CUR_33V` reader - Maximum Current for 3.3V This bit specifies the Maximum Current for 3.3V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
pub type MAX_CUR_33V_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_CUR_30V` reader - Maximum Current for 3.0V This bit specifies the Maximum Current for 3.0V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
pub type MAX_CUR_30V_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_CUR_18V` reader - Maximum Current for 1.8V This bit specifies the Maximum Current for 1.8V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
pub type MAX_CUR_18V_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V This bit specifies the Maximum Current for 3.3V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_33v(&self) -> MAX_CUR_33V_R {
        MAX_CUR_33V_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V This bit specifies the Maximum Current for 3.0V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_30v(&self) -> MAX_CUR_30V_R {
        MAX_CUR_30V_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V This bit specifies the Maximum Current for 1.8V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_18v(&self) -> MAX_CUR_18V_R {
        MAX_CUR_18V_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [curr_capabilities1](index.html) module"]
pub struct CURR_CAPABILITIES1_SPEC;
impl crate::RegisterSpec for CURR_CAPABILITIES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [curr_capabilities1::R](R) reader structure"]
impl crate::Readable for CURR_CAPABILITIES1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURR_CAPABILITIES1 to value 0"]
impl crate::Resettable for CURR_CAPABILITIES1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
