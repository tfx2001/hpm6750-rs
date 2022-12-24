#[doc = "Register `TPM` reader"]
pub struct R(crate::R<TPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPM` writer"]
pub struct W(crate::W<TPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TPM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPM` reader - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
pub type TPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPM` writer - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
pub type TPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPM_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - A multiplication value for I2C timing parameters. All the timing parameters in the Setup Register are multiplied by (TPM+1)."]
    #[inline(always)]
    #[must_use]
    pub fn tpm(&mut self) -> TPM_W<0> {
        TPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Timing Paramater Multiplier\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpm](index.html) module"]
pub struct TPM_SPEC;
impl crate::RegisterSpec for TPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpm::R](R) reader structure"]
impl crate::Readable for TPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpm::W](W) writer structure"]
impl crate::Writable for TPM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPM to value 0"]
impl crate::Resettable for TPM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
