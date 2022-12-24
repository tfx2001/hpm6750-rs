#[doc = "Register `INFO` reader"]
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - The version of the PLIC design"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_PRIORITY` reader - The maximum priority supported"]
pub type MAX_PRIORITY_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The version of the PLIC design"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The maximum priority supported"]
    #[inline(always)]
    pub fn max_priority(&self) -> MAX_PRIORITY_R {
        MAX_PRIORITY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Version and the maximum priority\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [info](index.html) module"]
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [info::R](R) reader structure"]
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for INFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
