#[doc = "Register `ACF` reader"]
pub struct R(crate::R<ACF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACF` writer"]
pub struct W(crate::W<ACF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACF_SPEC>;
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
impl From<crate::W<ACF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE_MASK` reader - Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
pub type CODE_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CODE_MASK` writer - Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
pub type CODE_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACF_SPEC, u32, u32, 29, O>;
#[doc = "Field `AIDE` reader - Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
pub type AIDE_R = crate::BitReader<bool>;
#[doc = "Field `AIDE` writer - Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
pub type AIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACF_SPEC, bool, O>;
#[doc = "Field `AIDEE` reader - Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
pub type AIDEE_R = crate::BitReader<bool>;
#[doc = "Field `AIDEE` writer - Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
pub type AIDEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
    #[inline(always)]
    pub fn code_mask(&self) -> CODE_MASK_R {
        CODE_MASK_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
    #[inline(always)]
    pub fn aide(&self) -> AIDE_R {
        AIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
    #[inline(always)]
    pub fn aidee(&self) -> AIDEE_R {
        AIDEE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
    #[inline(always)]
    #[must_use]
    pub fn code_mask(&mut self) -> CODE_MASK_W<0> {
        CODE_MASK_W::new(self)
    }
    #[doc = "Bit 29 - Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
    #[inline(always)]
    #[must_use]
    pub fn aide(&mut self) -> AIDE_W<29> {
        AIDE_W::new(self)
    }
    #[doc = "Bit 30 - Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
    #[inline(always)]
    #[must_use]
    pub fn aidee(&mut self) -> AIDEE_W<30> {
        AIDEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acceptance CODE ACODE or ACMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acf](index.html) module"]
pub struct ACF_SPEC;
impl crate::RegisterSpec for ACF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acf::R](R) reader structure"]
impl crate::Readable for ACF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acf::W](W) writer structure"]
impl crate::Writable for ACF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACF to value 0"]
impl crate::Resettable for ACF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
