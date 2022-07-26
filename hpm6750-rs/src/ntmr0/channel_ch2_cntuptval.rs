#[doc = "Register `CHANNEL_CH2_CNTUPTVAL` reader"]
pub struct R(crate::R<CHANNEL_CH2_CNTUPTVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH2_CNTUPTVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH2_CNTUPTVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH2_CNTUPTVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CH2_CNTUPTVAL` writer"]
pub struct W(crate::W<CHANNEL_CH2_CNTUPTVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CH2_CNTUPTVAL_SPEC>;
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
impl From<crate::W<CHANNEL_CH2_CNTUPTVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CH2_CNTUPTVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTUPTVAL` reader - counter will be set to this value when software write cntupt bit in CR"]
pub type CNTUPTVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNTUPTVAL` writer - counter will be set to this value when software write cntupt bit in CR"]
pub type CNTUPTVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CH2_CNTUPTVAL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - counter will be set to this value when software write cntupt bit in CR"]
    #[inline(always)]
    pub fn cntuptval(&self) -> CNTUPTVAL_R {
        CNTUPTVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - counter will be set to this value when software write cntupt bit in CR"]
    #[inline(always)]
    pub fn cntuptval(&mut self) -> CNTUPTVAL_W<0> {
        CNTUPTVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter update value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch2_cntuptval](index.html) module"]
pub struct CHANNEL_CH2_CNTUPTVAL_SPEC;
impl crate::RegisterSpec for CHANNEL_CH2_CNTUPTVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch2_cntuptval::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH2_CNTUPTVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_ch2_cntuptval::W](W) writer structure"]
impl crate::Writable for CHANNEL_CH2_CNTUPTVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANNEL_CH2_CNTUPTVAL to value 0"]
impl crate::Resettable for CHANNEL_CH2_CNTUPTVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
