#[doc = "Register `F_PRESC` reader"]
pub struct R(crate::R<F_PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F_PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F_PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F_PRESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F_PRESC` writer"]
pub struct W(crate::W<F_PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F_PRESC_SPEC>;
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
impl From<crate::W<F_PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F_PRESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F_PRESC` reader - Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
pub type F_PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F_PRESC` writer - Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
pub type F_PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F_PRESC_SPEC, u8, u8, 8, O>;
#[doc = "Field `F_SJW` reader - Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
pub type F_SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F_SJW` writer - Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
pub type F_SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F_PRESC_SPEC, u8, u8, 4, O>;
#[doc = "Field `F_SEG_2` reader - Bit Timing Segment 2 (fast speed) Time after the sample point"]
pub type F_SEG_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F_SEG_2` writer - Bit Timing Segment 2 (fast speed) Time after the sample point"]
pub type F_SEG_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F_PRESC_SPEC, u8, u8, 4, O>;
#[doc = "Field `F_SEG_1` reader - Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
pub type F_SEG_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F_SEG_1` writer - Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
pub type F_SEG_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, F_PRESC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 24:31 - Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
    #[inline(always)]
    pub fn f_presc(&self) -> F_PRESC_R {
        F_PRESC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
    #[inline(always)]
    pub fn f_sjw(&self) -> F_SJW_R {
        F_SJW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Bit Timing Segment 2 (fast speed) Time after the sample point"]
    #[inline(always)]
    pub fn f_seg_2(&self) -> F_SEG_2_R {
        F_SEG_2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
    #[inline(always)]
    pub fn f_seg_1(&self) -> F_SEG_1_R {
        F_SEG_1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\]
results in divider values 1 to 256."]
    #[inline(always)]
    pub fn f_presc(&mut self) -> F_PRESC_W<24> {
        F_PRESC_W::new(self)
    }
    #[doc = "Bits 16:19 - Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
    #[inline(always)]
    pub fn f_sjw(&mut self) -> F_SJW_W<16> {
        F_SJW_W::new(self)
    }
    #[doc = "Bits 8:11 - Bit Timing Segment 2 (fast speed) Time after the sample point"]
    #[inline(always)]
    pub fn f_seg_2(&mut self) -> F_SEG_2_W<8> {
        F_SEG_2_W::new(self)
    }
    #[doc = "Bits 0:3 - Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
    #[inline(always)]
    pub fn f_seg_1(&mut self) -> F_SEG_1_W<0> {
        F_SEG_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Timing Register(Fast Speed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f_presc](index.html) module"]
pub struct F_PRESC_SPEC;
impl crate::RegisterSpec for F_PRESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f_presc::R](R) reader structure"]
impl crate::Readable for F_PRESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f_presc::W](W) writer structure"]
impl crate::Writable for F_PRESC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F_PRESC to value 0x0102_0203"]
impl crate::Resettable for F_PRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0102_0203
    }
}
