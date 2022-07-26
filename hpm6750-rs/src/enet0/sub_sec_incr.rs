#[doc = "Register `SUB_SEC_INCR` reader"]
pub struct R(crate::R<SUB_SEC_INCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUB_SEC_INCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUB_SEC_INCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUB_SEC_INCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUB_SEC_INCR` writer"]
pub struct W(crate::W<SUB_SEC_INCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUB_SEC_INCR_SPEC>;
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
impl From<crate::W<SUB_SEC_INCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUB_SEC_INCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSINC` reader - Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register. For example, when PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time- Nanoseconds register has an accuracy of 1 ns \\[Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register)\\]. When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0.465ns. In this case, you should program a value of 43 (0x2B) that is derived by 20ns/0.465."]
pub type SSINC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSINC` writer - Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register. For example, when PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time- Nanoseconds register has an accuracy of 1 ns \\[Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register)\\]. When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0.465ns. In this case, you should program a value of 43 (0x2B) that is derived by 20ns/0.465."]
pub type SSINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUB_SEC_INCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register. For example, when PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time- Nanoseconds register has an accuracy of 1 ns \\[Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register)\\]. When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0.465ns. In this case, you should program a value of 43 (0x2B) that is derived by 20ns/0.465."]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the sub-second register. For example, when PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time- Nanoseconds register has an accuracy of 1 ns \\[Bit 9 (TSCTRLSSR) is set in Register 448 (Timestamp Control Register)\\]. When TSCTRLSSR is clear, the Nanoseconds register has a resolution of ~0.465ns. In this case, you should program a value of 43 (0x2B) that is derived by 20ns/0.465."]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W<0> {
        SSINC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Second Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sub_sec_incr](index.html) module"]
pub struct SUB_SEC_INCR_SPEC;
impl crate::RegisterSpec for SUB_SEC_INCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sub_sec_incr::R](R) reader structure"]
impl crate::Readable for SUB_SEC_INCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sub_sec_incr::W](W) writer structure"]
impl crate::Writable for SUB_SEC_INCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUB_SEC_INCR to value 0"]
impl crate::Resettable for SUB_SEC_INCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
