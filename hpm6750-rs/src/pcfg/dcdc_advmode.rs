#[doc = "Register `DCDC_ADVMODE` reader"]
pub struct R(crate::R<DCDC_ADVMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_ADVMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_ADVMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_ADVMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_ADVMODE` writer"]
pub struct W(crate::W<DCDC_ADVMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_ADVMODE_SPEC>;
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
impl From<crate::W<DCDC_ADVMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_ADVMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_DCM` reader - DCM mode 0: CCM mode 1: DCM mode"]
pub type EN_DCM_R = crate::BitReader<bool>;
#[doc = "Field `EN_DCM` writer - DCM mode 0: CCM mode 1: DCM mode"]
pub type EN_DCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `EN_IDLE` reader - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess"]
pub type EN_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `EN_IDLE` writer - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess"]
pub type EN_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `EN_SKIP` reader - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse"]
pub type EN_SKIP_R = crate::BitReader<bool>;
#[doc = "Field `EN_SKIP` writer - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse"]
pub type EN_SKIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `EN_DCM_EXIT` reader - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess"]
pub type EN_DCM_EXIT_R = crate::BitReader<bool>;
#[doc = "Field `EN_DCM_EXIT` writer - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess"]
pub type EN_DCM_EXIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `EN_AUTOLP` reader - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low"]
pub type EN_AUTOLP_R = crate::BitReader<bool>;
#[doc = "Field `EN_AUTOLP` writer - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low"]
pub type EN_AUTOLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `EN_FF_LOOP` reader - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled"]
pub type EN_FF_LOOP_R = crate::BitReader<bool>;
#[doc = "Field `EN_FF_LOOP` writer - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled"]
pub type EN_FF_LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `EN_FF_DET` reader - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled"]
pub type EN_FF_DET_R = crate::BitReader<bool>;
#[doc = "Field `EN_FF_DET` writer - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled"]
pub type EN_FF_DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_ADVMODE_SPEC, bool, O>;
#[doc = "Field `DC_R` reader - Loop R number"]
pub type DC_R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC_R` writer - Loop R number"]
pub type DC_R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_ADVMODE_SPEC, u8, u8, 4, O>;
#[doc = "Field `DC_C` reader - Loop C number"]
pub type DC_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC_C` writer - Loop C number"]
pub type DC_C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_ADVMODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `EN_RCSCALE` reader - Enable RC scale"]
pub type EN_RCSCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EN_RCSCALE` writer - Enable RC scale"]
pub type EN_RCSCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC_ADVMODE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - DCM mode 0: CCM mode 1: DCM mode"]
    #[inline(always)]
    pub fn en_dcm(&self) -> EN_DCM_R {
        EN_DCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess"]
    #[inline(always)]
    pub fn en_idle(&self) -> EN_IDLE_R {
        EN_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse"]
    #[inline(always)]
    pub fn en_skip(&self) -> EN_SKIP_R {
        EN_SKIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess"]
    #[inline(always)]
    pub fn en_dcm_exit(&self) -> EN_DCM_EXIT_R {
        EN_DCM_EXIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low"]
    #[inline(always)]
    pub fn en_autolp(&self) -> EN_AUTOLP_R {
        EN_AUTOLP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled"]
    #[inline(always)]
    pub fn en_ff_loop(&self) -> EN_FF_LOOP_R {
        EN_FF_LOOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled"]
    #[inline(always)]
    pub fn en_ff_det(&self) -> EN_FF_DET_R {
        EN_FF_DET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Loop R number"]
    #[inline(always)]
    pub fn dc_r(&self) -> DC_R_R {
        DC_R_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Loop C number"]
    #[inline(always)]
    pub fn dc_c(&self) -> DC_C_R {
        DC_C_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Enable RC scale"]
    #[inline(always)]
    pub fn en_rcscale(&self) -> EN_RCSCALE_R {
        EN_RCSCALE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCM mode 0: CCM mode 1: DCM mode"]
    #[inline(always)]
    #[must_use]
    pub fn en_dcm(&mut self) -> EN_DCM_W<0> {
        EN_DCM_W::new(self)
    }
    #[doc = "Bit 1 - enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess"]
    #[inline(always)]
    #[must_use]
    pub fn en_idle(&mut self) -> EN_IDLE_W<1> {
        EN_IDLE_W::new(self)
    }
    #[doc = "Bit 2 - enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse"]
    #[inline(always)]
    #[must_use]
    pub fn en_skip(&mut self) -> EN_SKIP_W<2> {
        EN_SKIP_W::new(self)
    }
    #[doc = "Bit 3 - avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess"]
    #[inline(always)]
    #[must_use]
    pub fn en_dcm_exit(&mut self) -> EN_DCM_EXIT_W<3> {
        EN_DCM_EXIT_W::new(self)
    }
    #[doc = "Bit 4 - enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low"]
    #[inline(always)]
    #[must_use]
    pub fn en_autolp(&mut self) -> EN_AUTOLP_W<4> {
        EN_AUTOLP_W::new(self)
    }
    #[doc = "Bit 5 - enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en_ff_loop(&mut self) -> EN_FF_LOOP_W<5> {
        EN_FF_LOOP_W::new(self)
    }
    #[doc = "Bit 6 - enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en_ff_det(&mut self) -> EN_FF_DET_W<6> {
        EN_FF_DET_W::new(self)
    }
    #[doc = "Bits 16:19 - Loop R number"]
    #[inline(always)]
    #[must_use]
    pub fn dc_r(&mut self) -> DC_R_W<16> {
        DC_R_W::new(self)
    }
    #[doc = "Bits 20:21 - Loop C number"]
    #[inline(always)]
    #[must_use]
    pub fn dc_c(&mut self) -> DC_C_W<20> {
        DC_C_W::new(self)
    }
    #[doc = "Bits 24:26 - Enable RC scale"]
    #[inline(always)]
    #[must_use]
    pub fn en_rcscale(&mut self) -> EN_RCSCALE_W<24> {
        EN_RCSCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC advance setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_advmode](index.html) module"]
pub struct DCDC_ADVMODE_SPEC;
impl crate::RegisterSpec for DCDC_ADVMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_advmode::R](R) reader structure"]
impl crate::Readable for DCDC_ADVMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_advmode::W](W) writer structure"]
impl crate::Writable for DCDC_ADVMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDC_ADVMODE to value 0x00ef_1c6e"]
impl crate::Resettable for DCDC_ADVMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ef_1c6e;
}
