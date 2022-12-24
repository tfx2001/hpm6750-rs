#[doc = "Register `CQCRA` reader"]
pub struct R(crate::R<CQCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMD_RESP_ARG` reader - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
pub type CMD_RESP_ARG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
    #[inline(always)]
    pub fn cmd_resp_arg(&self) -> CMD_RESP_ARG_R {
        CMD_RESP_ARG_R::new(self.bits)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcra](index.html) module"]
pub struct CQCRA_SPEC;
impl crate::RegisterSpec for CQCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcra::R](R) reader structure"]
impl crate::Readable for CQCRA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CQCRA to value 0"]
impl crate::Resettable for CQCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
