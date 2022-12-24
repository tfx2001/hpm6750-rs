#[doc = "Register `FILTCTRL` reader"]
pub struct R(crate::R<FILTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTCTRL` writer"]
pub struct W(crate::W<FILTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTCTRL_SPEC>;
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
impl From<crate::W<FILTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IIR_SLOT_EN` reader - IIR slot enable"]
pub type IIR_SLOT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IIR_SLOT_EN` writer - IIR slot enable"]
pub type IIR_SLOT_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTCTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DECRATIO` reader - the decimation ratio of iir after CIC -1 2: means dec-by-3"]
pub type DECRATIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECRATIO` writer - the decimation ratio of iir after CIC -1 2: means dec-by-3"]
pub type DECRATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTCTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - IIR slot enable"]
    #[inline(always)]
    pub fn iir_slot_en(&self) -> IIR_SLOT_EN_R {
        IIR_SLOT_EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - the decimation ratio of iir after CIC -1 2: means dec-by-3"]
    #[inline(always)]
    pub fn decratio(&self) -> DECRATIO_R {
        DECRATIO_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IIR slot enable"]
    #[inline(always)]
    #[must_use]
    pub fn iir_slot_en(&mut self) -> IIR_SLOT_EN_W<0> {
        IIR_SLOT_EN_W::new(self)
    }
    #[doc = "Bits 8:10 - the decimation ratio of iir after CIC -1 2: means dec-by-3"]
    #[inline(always)]
    #[must_use]
    pub fn decratio(&mut self) -> DECRATIO_W<8> {
        DECRATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filtctrl](index.html) module"]
pub struct FILTCTRL_SPEC;
impl crate::RegisterSpec for FILTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filtctrl::R](R) reader structure"]
impl crate::Readable for FILTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filtctrl::W](W) writer structure"]
impl crate::Writable for FILTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTCTRL to value 0"]
impl crate::Resettable for FILTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
