#[doc = "Register `PPS_2_WIDTH` reader"]
pub struct R(crate::R<PPS_2_WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_2_WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_2_WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_2_WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS_2_WIDTH` writer"]
pub struct W(crate::W<PPS_2_WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_2_WIDTH_SPEC>;
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
impl From<crate::W<PPS_2_WIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_2_WIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSWIDTH` reader - PPS1 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS1 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
pub type PPSWIDTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PPSWIDTH` writer - PPS1 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS1 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
pub type PPSWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PPS_2_WIDTH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PPS1 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS1 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
    #[inline(always)]
    pub fn ppswidth(&self) -> PPSWIDTH_R {
        PPSWIDTH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS1 Output Signal Width These bits store the width between the rising edge and corresponding falling edge of the PPS1 signal output in terms of units of sub-second increment value. You need to program one value less than the required interval. For example, if PTP reference clock is 50 MHz (period of 20ns), and desired width between the rising and corresponding falling edges of PPS1 signal output is 80ns (that is, four units of sub-second increment value), then you should program value 3 (4 – 1) in this register."]
    #[inline(always)]
    pub fn ppswidth(&mut self) -> PPSWIDTH_W<0> {
        PPSWIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS Width Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps_2_width](index.html) module"]
pub struct PPS_2_WIDTH_SPEC;
impl crate::RegisterSpec for PPS_2_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps_2_width::R](R) reader structure"]
impl crate::Readable for PPS_2_WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps_2_width::W](W) writer structure"]
impl crate::Writable for PPS_2_WIDTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPS_2_WIDTH to value 0"]
impl crate::Resettable for PPS_2_WIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
