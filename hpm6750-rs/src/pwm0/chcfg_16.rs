#[doc = "Register `CHCFG_16` reader"]
pub struct R(crate::R<CHCFG_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCFG_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCFG_16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCFG_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCFG_16` writer"]
pub struct W(crate::W<CHCFG_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCFG_16_SPEC>;
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
impl From<crate::W<CHCFG_16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCFG_16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPOL` reader - output polarity, set to 1 will invert the output"]
pub type OUTPOL_R = crate::BitReader<bool>;
#[doc = "Field `OUTPOL` writer - output polarity, set to 1 will invert the output"]
pub type OUTPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCFG_16_SPEC, bool, O>;
#[doc = "Field `CMPSELBEG` reader - assign the first comparator for this output channel"]
pub type CMPSELBEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSELBEG` writer - assign the first comparator for this output channel"]
pub type CMPSELBEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCFG_16_SPEC, u8, u8, 5, O>;
#[doc = "Field `CMPSELEND` reader - assign the last comparator for this output channel"]
pub type CMPSELEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSELEND` writer - assign the last comparator for this output channel"]
pub type CMPSELEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCFG_16_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 1 - output polarity, set to 1 will invert the output"]
    #[inline(always)]
    pub fn outpol(&self) -> OUTPOL_R {
        OUTPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:20 - assign the first comparator for this output channel"]
    #[inline(always)]
    pub fn cmpselbeg(&self) -> CMPSELBEG_R {
        CMPSELBEG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - assign the last comparator for this output channel"]
    #[inline(always)]
    pub fn cmpselend(&self) -> CMPSELEND_R {
        CMPSELEND_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - output polarity, set to 1 will invert the output"]
    #[inline(always)]
    #[must_use]
    pub fn outpol(&mut self) -> OUTPOL_W<1> {
        OUTPOL_W::new(self)
    }
    #[doc = "Bits 16:20 - assign the first comparator for this output channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpselbeg(&mut self) -> CMPSELBEG_W<16> {
        CMPSELBEG_W::new(self)
    }
    #[doc = "Bits 24:28 - assign the last comparator for this output channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpselend(&mut self) -> CMPSELEND_W<24> {
        CMPSELEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output channel configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg_16](index.html) module"]
pub struct CHCFG_16_SPEC;
impl crate::RegisterSpec for CHCFG_16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chcfg_16::R](R) reader structure"]
impl crate::Readable for CHCFG_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chcfg_16::W](W) writer structure"]
impl crate::Writable for CHCFG_16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCFG_16 to value 0"]
impl crate::Resettable for CHCFG_16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
