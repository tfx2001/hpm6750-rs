#[doc = "Register `CHANNEL_CH3_CMP1` reader"]
pub struct R(crate::R<CHANNEL_CH3_CMP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH3_CMP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH3_CMP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH3_CMP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CH3_CMP1` writer"]
pub struct W(crate::W<CHANNEL_CH3_CMP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CH3_CMP1_SPEC>;
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
impl From<crate::W<CHANNEL_CH3_CMP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CH3_CMP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - compare value 0"]
pub type CMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMP` writer - compare value 0"]
pub type CMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CH3_CMP1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - compare value 0"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - compare value 0"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch3_cmp1](index.html) module"]
pub struct CHANNEL_CH3_CMP1_SPEC;
impl crate::RegisterSpec for CHANNEL_CH3_CMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch3_cmp1::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH3_CMP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_ch3_cmp1::W](W) writer structure"]
impl crate::Writable for CHANNEL_CH3_CMP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL_CH3_CMP1 to value 0xffff_ffff"]
impl crate::Resettable for CHANNEL_CH3_CMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
