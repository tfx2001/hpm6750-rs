#[doc = "Register `ANA_STATUS` reader"]
pub struct R(crate::R<ANA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_STATUS` writer"]
pub struct W(crate::W<ANA_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_STATUS_SPEC>;
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
impl From<crate::W<ANA_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALON` reader - Indicates if the ADC is in calibration mode (Active H)."]
pub type CALON_R = crate::BitReader<bool>;
#[doc = "Field `CALON` writer - Indicates if the ADC is in calibration mode (Active H)."]
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Indicates if the ADC is in calibration mode (Active H)."]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Indicates if the ADC is in calibration mode (Active H)."]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<7> {
        CALON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_status](index.html) module"]
pub struct ANA_STATUS_SPEC;
impl crate::RegisterSpec for ANA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_status::R](R) reader structure"]
impl crate::Readable for ANA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_status::W](W) writer structure"]
impl crate::Writable for ANA_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_STATUS to value 0"]
impl crate::Resettable for ANA_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
