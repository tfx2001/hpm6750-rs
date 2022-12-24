#[doc = "Register `CHANNEL_CHN3_SR` reader"]
pub struct R(crate::R<CHANNEL_CHN3_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CHN3_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CHN3_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CHN3_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CHN3_SR` writer"]
pub struct W(crate::W<CHANNEL_CHN3_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CHN3_SR_SPEC>;
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
impl From<crate::W<CHANNEL_CHN3_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CHN3_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDGF` reader - Output rising edge flag. Write 1 to clear this flag."]
pub type REDGF_R = crate::BitReader<bool>;
#[doc = "Field `REDGF` writer - Output rising edge flag. Write 1 to clear this flag."]
pub type REDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN3_SR_SPEC, bool, O>;
#[doc = "Field `FEDGF` reader - Output falling edge flag. Write 1 to clear this flag."]
pub type FEDGF_R = crate::BitReader<bool>;
#[doc = "Field `FEDGF` writer - Output falling edge flag. Write 1 to clear this flag."]
pub type FEDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN3_SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Output rising edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn redgf(&self) -> REDGF_R {
        REDGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output falling edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    pub fn fedgf(&self) -> FEDGF_R {
        FEDGF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output rising edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    #[must_use]
    pub fn redgf(&mut self) -> REDGF_W<0> {
        REDGF_W::new(self)
    }
    #[doc = "Bit 1 - Output falling edge flag. Write 1 to clear this flag."]
    #[inline(always)]
    #[must_use]
    pub fn fedgf(&mut self) -> FEDGF_W<1> {
        FEDGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_chn3_sr](index.html) module"]
pub struct CHANNEL_CHN3_SR_SPEC;
impl crate::RegisterSpec for CHANNEL_CHN3_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_chn3_sr::R](R) reader structure"]
impl crate::Readable for CHANNEL_CHN3_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_chn3_sr::W](W) writer structure"]
impl crate::Writable for CHANNEL_CHN3_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL_CHN3_SR to value 0"]
impl crate::Resettable for CHANNEL_CHN3_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
