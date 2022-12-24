#[doc = "Register `SLVDATACNT` reader"]
pub struct R(crate::R<SLVDATACNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVDATACNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVDATACNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVDATACNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RCNT` reader - Slave received data count"]
pub type RCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WCNT` reader - Slave transmitted data count"]
pub type WCNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Slave received data count"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Slave transmitted data count"]
    #[inline(always)]
    pub fn wcnt(&self) -> WCNT_R {
        WCNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "Slave Data Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvdatacnt](index.html) module"]
pub struct SLVDATACNT_SPEC;
impl crate::RegisterSpec for SLVDATACNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvdatacnt::R](R) reader structure"]
impl crate::Readable for SLVDATACNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLVDATACNT to value 0"]
impl crate::Resettable for SLVDATACNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
