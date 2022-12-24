#[doc = "Register `CHANNEL_CHN3_DACCFG` reader"]
pub struct R(crate::R<CHANNEL_CHN3_DACCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CHN3_DACCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CHN3_DACCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CHN3_DACCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CHN3_DACCFG` writer"]
pub struct W(crate::W<CHANNEL_CHN3_DACCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CHN3_DACCFG_SPEC>;
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
impl From<crate::W<CHANNEL_CHN3_DACCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CHN3_DACCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACCFG` reader - 8bit DAC digital value"]
pub type DACCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACCFG` writer - 8bit DAC digital value"]
pub type DACCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CHN3_DACCFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8bit DAC digital value"]
    #[inline(always)]
    pub fn daccfg(&self) -> DACCFG_R {
        DACCFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8bit DAC digital value"]
    #[inline(always)]
    #[must_use]
    pub fn daccfg(&mut self) -> DACCFG_W<0> {
        DACCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_chn3_daccfg](index.html) module"]
pub struct CHANNEL_CHN3_DACCFG_SPEC;
impl crate::RegisterSpec for CHANNEL_CHN3_DACCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_chn3_daccfg::R](R) reader structure"]
impl crate::Readable for CHANNEL_CHN3_DACCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_chn3_daccfg::W](W) writer structure"]
impl crate::Writable for CHANNEL_CHN3_DACCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL_CHN3_DACCFG to value 0"]
impl crate::Resettable for CHANNEL_CHN3_DACCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
