#[doc = "Register `CHANNEL_CH3_RLD` reader"]
pub struct R(crate::R<CHANNEL_CH3_RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH3_RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH3_RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH3_RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CH3_RLD` writer"]
pub struct W(crate::W<CHANNEL_CH3_RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CH3_RLD_SPEC>;
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
impl From<crate::W<CHANNEL_CH3_RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CH3_RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLD` reader - reload value"]
pub type RLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RLD` writer - reload value"]
pub type RLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CH3_RLD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reload value"]
    #[inline(always)]
    pub fn rld(&mut self) -> RLD_W<0> {
        RLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch3_rld](index.html) module"]
pub struct CHANNEL_CH3_RLD_SPEC;
impl crate::RegisterSpec for CHANNEL_CH3_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch3_rld::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH3_RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_ch3_rld::W](W) writer structure"]
impl crate::Writable for CHANNEL_CH3_RLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANNEL_CH3_RLD to value 0xffff_ffff"]
impl crate::Resettable for CHANNEL_CH3_RLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
