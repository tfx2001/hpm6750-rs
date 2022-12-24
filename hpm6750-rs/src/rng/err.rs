#[doc = "Register `ERR` reader"]
pub struct R(crate::R<ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCKERR` reader - Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]"]
pub type SCKERR_R = crate::BitReader<bool>;
#[doc = "Field `FUFE` reader - FIFO access error(underflow)"]
pub type FUFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]"]
    #[inline(always)]
    pub fn sckerr(&self) -> SCKERR_R {
        SCKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO access error(underflow)"]
    #[inline(always)]
    pub fn fufe(&self) -> FUFE_R {
        FUFE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Error Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](index.html) module"]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err::R](R) reader structure"]
impl crate::Readable for ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
