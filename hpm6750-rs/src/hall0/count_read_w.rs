#[doc = "Register `COUNT_READ_W` reader"]
pub struct R(crate::R<COUNT_READ_W_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_READ_W_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_READ_W_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_READ_W_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WCNT` reader - wcnt counter"]
pub type WCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - wcnt counter"]
    #[inline(always)]
    pub fn wcnt(&self) -> WCNT_R {
        WCNT_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
#[doc = "W read register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_read_w](index.html) module"]
pub struct COUNT_READ_W_SPEC;
impl crate::RegisterSpec for COUNT_READ_W_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_read_w::R](R) reader structure"]
impl crate::Readable for COUNT_READ_W_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNT_READ_W to value 0"]
impl crate::Resettable for COUNT_READ_W_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
