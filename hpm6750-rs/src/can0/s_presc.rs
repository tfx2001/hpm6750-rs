#[doc = "Register `S_PRESC` reader"]
pub struct R(crate::R<S_PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_PRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S_PRESC` writer"]
pub struct W(crate::W<S_PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_PRESC_SPEC>;
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
impl From<crate::W<S_PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_PRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S_SEG_1` reader - Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
pub type S_SEG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_SEG_1` writer - Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
pub type S_SEG_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, S_PRESC_SPEC, u8, u8, 8, O>;
#[doc = "Field `S_SEG_2` reader - Bit Timing Segment 2 (slow speed) Time after the sample point."]
pub type S_SEG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_SEG_2` writer - Bit Timing Segment 2 (slow speed) Time after the sample point."]
pub type S_SEG_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, S_PRESC_SPEC, u8, u8, 7, O>;
#[doc = "Field `S_SJW` reader - Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
pub type S_SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_SJW` writer - Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
pub type S_SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, S_PRESC_SPEC, u8, u8, 7, O>;
#[doc = "Field `S_PRESC` reader - Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
pub type S_PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S_PRESC` writer - Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
pub type S_PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, S_PRESC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
    #[inline(always)]
    pub fn s_seg_1(&self) -> S_SEG_1_R {
        S_SEG_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Bit Timing Segment 2 (slow speed) Time after the sample point."]
    #[inline(always)]
    pub fn s_seg_2(&self) -> S_SEG_2_R {
        S_SEG_2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
    #[inline(always)]
    pub fn s_sjw(&self) -> S_SJW_R {
        S_SJW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
    #[inline(always)]
    pub fn s_presc(&self) -> S_PRESC_R {
        S_PRESC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
    #[inline(always)]
    #[must_use]
    pub fn s_seg_1(&mut self) -> S_SEG_1_W<0> {
        S_SEG_1_W::new(self)
    }
    #[doc = "Bits 8:14 - Bit Timing Segment 2 (slow speed) Time after the sample point."]
    #[inline(always)]
    #[must_use]
    pub fn s_seg_2(&mut self) -> S_SEG_2_W<8> {
        S_SEG_2_W::new(self)
    }
    #[doc = "Bits 16:22 - Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
    #[inline(always)]
    #[must_use]
    pub fn s_sjw(&mut self) -> S_SJW_W<16> {
        S_SJW_W::new(self)
    }
    #[doc = "Bits 24:31 - Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
    #[inline(always)]
    #[must_use]
    pub fn s_presc(&mut self) -> S_PRESC_W<24> {
        S_PRESC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Timing Register(Slow Speed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s_presc](index.html) module"]
pub struct S_PRESC_SPEC;
impl crate::RegisterSpec for S_PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s_presc::R](R) reader structure"]
impl crate::Readable for S_PRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s_presc::W](W) writer structure"]
impl crate::Writable for S_PRESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S_PRESC to value 0x0102_0203"]
impl crate::Resettable for S_PRESC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102_0203;
}
