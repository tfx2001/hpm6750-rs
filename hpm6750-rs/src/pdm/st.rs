#[doc = "Register `ST` reader"]
pub struct R(crate::R<ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST` writer"]
pub struct W(crate::W<ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST_SPEC>;
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
impl From<crate::W<ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIC_SAT_ERR` reader - CIC saturation. Write 1 clear"]
pub type CIC_SAT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CIC_SAT_ERR` writer - CIC saturation. Write 1 clear"]
pub type CIC_SAT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `CIC_OVLD_ERR` reader - CIC overload error. write 1 clear"]
pub type CIC_OVLD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CIC_OVLD_ERR` writer - CIC overload error. write 1 clear"]
pub type CIC_OVLD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `OFIFO_OVFL_ERR` reader - output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
pub type OFIFO_OVFL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `OFIFO_OVFL_ERR` writer - output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
pub type OFIFO_OVFL_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `FILT_CRX_ERR` reader - data accessed out of boundary error"]
pub type FILT_CRX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FILT_CRX_ERR` writer - data accessed out of boundary error"]
pub type FILT_CRX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CIC saturation. Write 1 clear"]
    #[inline(always)]
    pub fn cic_sat_err(&self) -> CIC_SAT_ERR_R {
        CIC_SAT_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CIC overload error. write 1 clear"]
    #[inline(always)]
    pub fn cic_ovld_err(&self) -> CIC_OVLD_ERR_R {
        CIC_OVLD_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
    #[inline(always)]
    pub fn ofifo_ovfl_err(&self) -> OFIFO_OVFL_ERR_R {
        OFIFO_OVFL_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - data accessed out of boundary error"]
    #[inline(always)]
    pub fn filt_crx_err(&self) -> FILT_CRX_ERR_R {
        FILT_CRX_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CIC saturation. Write 1 clear"]
    #[inline(always)]
    #[must_use]
    pub fn cic_sat_err(&mut self) -> CIC_SAT_ERR_W<0> {
        CIC_SAT_ERR_W::new(self)
    }
    #[doc = "Bit 1 - CIC overload error. write 1 clear"]
    #[inline(always)]
    #[must_use]
    pub fn cic_ovld_err(&mut self) -> CIC_OVLD_ERR_W<1> {
        CIC_OVLD_ERR_W::new(self)
    }
    #[doc = "Bit 2 - output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
    #[inline(always)]
    #[must_use]
    pub fn ofifo_ovfl_err(&mut self) -> OFIFO_OVFL_ERR_W<2> {
        OFIFO_OVFL_ERR_W::new(self)
    }
    #[doc = "Bit 3 - data accessed out of boundary error"]
    #[inline(always)]
    #[must_use]
    pub fn filt_crx_err(&mut self) -> FILT_CRX_ERR_W<3> {
        FILT_CRX_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st](index.html) module"]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st::R](R) reader structure"]
impl crate::Readable for ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st::W](W) writer structure"]
impl crate::Writable for ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
