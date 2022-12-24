#[doc = "Register `DCDC_MISC` reader"]
pub struct R(crate::R<DCDC_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_MISC` writer"]
pub struct W(crate::W<DCDC_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_MISC_SPEC>;
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
impl From<crate::W<DCDC_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_STEP` reader - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
pub type EN_STEP_R = crate::BitReader<bool>;
#[doc = "Field `EN_STEP` writer - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
pub type EN_STEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `CLK_SEL` reader - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
pub type CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLK_SEL` writer - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
pub type CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `DELAY` reader - enable delay 0: delay disabled, 1: delay enabled"]
pub type DELAY_R = crate::BitReader<bool>;
#[doc = "Field `DELAY` writer - enable delay 0: delay disabled, 1: delay enabled"]
pub type DELAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `OL_HYST` reader - current hysteres range 0: 12.5mV 1: 25mV"]
pub type OL_HYST_R = crate::BitReader<bool>;
#[doc = "Field `OL_HYST` writer - current hysteres range 0: 12.5mV 1: 25mV"]
pub type OL_HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `OL_THRE` reader - overload for threshold for lod power mode"]
pub type OL_THRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OL_THRE` writer - overload for threshold for lod power mode"]
pub type OL_THRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_MISC_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC_FF` reader - Loop feed forward number"]
pub type DC_FF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DC_FF` writer - Loop feed forward number"]
pub type DC_FF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_MISC_SPEC, u8, u8, 3, O>;
#[doc = "Field `RC_SCALE` reader - Loop RC scale threshold"]
pub type RC_SCALE_R = crate::BitReader<bool>;
#[doc = "Field `RC_SCALE` writer - Loop RC scale threshold"]
pub type RC_SCALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `HYST_THRS` reader - hysteres threshold"]
pub type HYST_THRS_R = crate::BitReader<bool>;
#[doc = "Field `HYST_THRS` writer - hysteres threshold"]
pub type HYST_THRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `HYST_SIGN` reader - hysteres sign"]
pub type HYST_SIGN_R = crate::BitReader<bool>;
#[doc = "Field `HYST_SIGN` writer - hysteres sign"]
pub type HYST_SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
#[doc = "Field `EN_HYST` reader - hysteres enable"]
pub type EN_HYST_R = crate::BitReader<bool>;
#[doc = "Field `EN_HYST` writer - hysteres enable"]
pub type EN_HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDC_MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
    #[inline(always)]
    pub fn en_step(&self) -> EN_STEP_R {
        EN_STEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable delay 0: delay disabled, 1: delay enabled"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - current hysteres range 0: 12.5mV 1: 25mV"]
    #[inline(always)]
    pub fn ol_hyst(&self) -> OL_HYST_R {
        OL_HYST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - overload for threshold for lod power mode"]
    #[inline(always)]
    pub fn ol_thre(&self) -> OL_THRE_R {
        OL_THRE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Loop feed forward number"]
    #[inline(always)]
    pub fn dc_ff(&self) -> DC_FF_R {
        DC_FF_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Loop RC scale threshold"]
    #[inline(always)]
    pub fn rc_scale(&self) -> RC_SCALE_R {
        RC_SCALE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - hysteres threshold"]
    #[inline(always)]
    pub fn hyst_thrs(&self) -> HYST_THRS_R {
        HYST_THRS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - hysteres sign"]
    #[inline(always)]
    pub fn hyst_sign(&self) -> HYST_SIGN_R {
        HYST_SIGN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - hysteres enable"]
    #[inline(always)]
    pub fn en_hyst(&self) -> EN_HYST_R {
        EN_HYST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en_step(&mut self) -> EN_STEP_W<0> {
        EN_STEP_W::new(self)
    }
    #[doc = "Bit 1 - clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<1> {
        CLK_SEL_W::new(self)
    }
    #[doc = "Bit 2 - enable delay 0: delay disabled, 1: delay enabled"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<2> {
        DELAY_W::new(self)
    }
    #[doc = "Bit 4 - current hysteres range 0: 12.5mV 1: 25mV"]
    #[inline(always)]
    #[must_use]
    pub fn ol_hyst(&mut self) -> OL_HYST_W<4> {
        OL_HYST_W::new(self)
    }
    #[doc = "Bits 8:9 - overload for threshold for lod power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ol_thre(&mut self) -> OL_THRE_W<8> {
        OL_THRE_W::new(self)
    }
    #[doc = "Bits 16:18 - Loop feed forward number"]
    #[inline(always)]
    #[must_use]
    pub fn dc_ff(&mut self) -> DC_FF_W<16> {
        DC_FF_W::new(self)
    }
    #[doc = "Bit 20 - Loop RC scale threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rc_scale(&mut self) -> RC_SCALE_W<20> {
        RC_SCALE_W::new(self)
    }
    #[doc = "Bit 24 - hysteres threshold"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_thrs(&mut self) -> HYST_THRS_W<24> {
        HYST_THRS_W::new(self)
    }
    #[doc = "Bit 25 - hysteres sign"]
    #[inline(always)]
    #[must_use]
    pub fn hyst_sign(&mut self) -> HYST_SIGN_W<25> {
        HYST_SIGN_W::new(self)
    }
    #[doc = "Bit 28 - hysteres enable"]
    #[inline(always)]
    #[must_use]
    pub fn en_hyst(&mut self) -> EN_HYST_W<28> {
        EN_HYST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC misc parameter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_misc](index.html) module"]
pub struct DCDC_MISC_SPEC;
impl crate::RegisterSpec for DCDC_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_misc::R](R) reader structure"]
impl crate::Readable for DCDC_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_misc::W](W) writer structure"]
impl crate::Writable for DCDC_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDC_MISC to value 0x0007_0100"]
impl crate::Resettable for DCDC_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0100;
}
