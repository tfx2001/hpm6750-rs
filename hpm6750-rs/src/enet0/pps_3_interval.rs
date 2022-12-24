#[doc = "Register `PPS_3_INTERVAL` reader"]
pub struct R(crate::R<PPS_3_INTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_3_INTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_3_INTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_3_INTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS_3_INTERVAL` writer"]
pub struct W(crate::W<PPS_3_INTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_3_INTERVAL_SPEC>;
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
impl From<crate::W<PPS_3_INTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_3_INTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSINT` reader - PPS1 Output Signal Interval These bits store the interval between the rising edges of PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS1 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
pub type PPSINT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PPSINT` writer - PPS1 Output Signal Interval These bits store the interval between the rising edges of PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS1 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
pub type PPSINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PPS_3_INTERVAL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PPS1 Output Signal Interval These bits store the interval between the rising edges of PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS1 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
    #[inline(always)]
    pub fn ppsint(&self) -> PPSINT_R {
        PPSINT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS1 Output Signal Interval These bits store the interval between the rising edges of PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20ns), and desired interval between rising edges of PPS1 signal output is 100ns (that is, five units of sub-second increment value), then you should program value 4 (5 – 1) in this register."]
    #[inline(always)]
    #[must_use]
    pub fn ppsint(&mut self) -> PPSINT_W<0> {
        PPSINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps_3_interval](index.html) module"]
pub struct PPS_3_INTERVAL_SPEC;
impl crate::RegisterSpec for PPS_3_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps_3_interval::R](R) reader structure"]
impl crate::Readable for PPS_3_INTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps_3_interval::W](W) writer structure"]
impl crate::Writable for PPS_3_INTERVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPS_3_INTERVAL to value 0"]
impl crate::Resettable for PPS_3_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
