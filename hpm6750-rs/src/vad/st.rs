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
#[doc = "Field `CIC_SAT_ERR` reader - CIC saturation"]
pub type CIC_SAT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CIC_SAT_ERR` writer - CIC saturation"]
pub type CIC_SAT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `CIC_OVLD_ERR` reader - CIC overload"]
pub type CIC_OVLD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CIC_OVLD_ERR` writer - CIC overload"]
pub type CIC_OVLD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `IIR_OVFL` reader - IIR oberflow"]
pub type IIR_OVFL_R = crate::BitReader<bool>;
#[doc = "Field `IIR_OVFL` writer - IIR oberflow"]
pub type IIR_OVFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `IIR_OVLD` reader - IIR overloading"]
pub type IIR_OVLD_R = crate::BitReader<bool>;
#[doc = "Field `IIR_OVLD` writer - IIR overloading"]
pub type IIR_OVLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `OFIFO_OVFL` reader - OFIFO overflow"]
pub type OFIFO_OVFL_R = crate::BitReader<bool>;
#[doc = "Field `OFIFO_OVFL` writer - OFIFO overflow"]
pub type OFIFO_OVFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `MEMBUF_EMPTY` reader - Buf empty"]
pub type MEMBUF_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `MEMBUF_EMPTY` writer - Buf empty"]
pub type MEMBUF_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `OFIFO_AV` reader - OFIFO data available"]
pub type OFIFO_AV_R = crate::BitReader<bool>;
#[doc = "Field `VAD` reader - VAD event found"]
pub type VAD_R = crate::BitReader<bool>;
#[doc = "Field `VAD` writer - VAD event found"]
pub type VAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CIC saturation"]
    #[inline(always)]
    pub fn cic_sat_err(&self) -> CIC_SAT_ERR_R {
        CIC_SAT_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CIC overload"]
    #[inline(always)]
    pub fn cic_ovld_err(&self) -> CIC_OVLD_ERR_R {
        CIC_OVLD_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IIR oberflow"]
    #[inline(always)]
    pub fn iir_ovfl(&self) -> IIR_OVFL_R {
        IIR_OVFL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IIR overloading"]
    #[inline(always)]
    pub fn iir_ovld(&self) -> IIR_OVLD_R {
        IIR_OVLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OFIFO overflow"]
    #[inline(always)]
    pub fn ofifo_ovfl(&self) -> OFIFO_OVFL_R {
        OFIFO_OVFL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buf empty"]
    #[inline(always)]
    pub fn membuf_empty(&self) -> MEMBUF_EMPTY_R {
        MEMBUF_EMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OFIFO data available"]
    #[inline(always)]
    pub fn ofifo_av(&self) -> OFIFO_AV_R {
        OFIFO_AV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VAD event found"]
    #[inline(always)]
    pub fn vad(&self) -> VAD_R {
        VAD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CIC saturation"]
    #[inline(always)]
    #[must_use]
    pub fn cic_sat_err(&mut self) -> CIC_SAT_ERR_W<0> {
        CIC_SAT_ERR_W::new(self)
    }
    #[doc = "Bit 1 - CIC overload"]
    #[inline(always)]
    #[must_use]
    pub fn cic_ovld_err(&mut self) -> CIC_OVLD_ERR_W<1> {
        CIC_OVLD_ERR_W::new(self)
    }
    #[doc = "Bit 2 - IIR oberflow"]
    #[inline(always)]
    #[must_use]
    pub fn iir_ovfl(&mut self) -> IIR_OVFL_W<2> {
        IIR_OVFL_W::new(self)
    }
    #[doc = "Bit 3 - IIR overloading"]
    #[inline(always)]
    #[must_use]
    pub fn iir_ovld(&mut self) -> IIR_OVLD_W<3> {
        IIR_OVLD_W::new(self)
    }
    #[doc = "Bit 4 - OFIFO overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ofifo_ovfl(&mut self) -> OFIFO_OVFL_W<4> {
        OFIFO_OVFL_W::new(self)
    }
    #[doc = "Bit 5 - Buf empty"]
    #[inline(always)]
    #[must_use]
    pub fn membuf_empty(&mut self) -> MEMBUF_EMPTY_W<5> {
        MEMBUF_EMPTY_W::new(self)
    }
    #[doc = "Bit 7 - VAD event found"]
    #[inline(always)]
    #[must_use]
    pub fn vad(&mut self) -> VAD_W<7> {
        VAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st](index.html) module"]
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
