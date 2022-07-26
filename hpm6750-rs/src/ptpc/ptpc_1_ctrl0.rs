#[doc = "Register `PTPC_1_CTRL0` reader"]
pub struct R(crate::R<PTPC_1_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_1_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_1_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_1_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPC_1_CTRL0` writer"]
pub struct W(crate::W<PTPC_1_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPC_1_CTRL0_SPEC>;
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
impl From<crate::W<PTPC_1_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPC_1_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSEC_DIGITAL_ROLLOVER` reader - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
pub type SUBSEC_DIGITAL_ROLLOVER_R = crate::BitReader<bool>;
#[doc = "Field `SUBSEC_DIGITAL_ROLLOVER` writer - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
pub type SUBSEC_DIGITAL_ROLLOVER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `CAPT_SNAP_KEEP` reader - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
pub type CAPT_SNAP_KEEP_R = crate::BitReader<bool>;
#[doc = "Field `CAPT_SNAP_KEEP` writer - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
pub type CAPT_SNAP_KEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `CAPT_SNAP_POS_EN` reader - set will use posege of input capture signal to latch timestamp value"]
pub type CAPT_SNAP_POS_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAPT_SNAP_POS_EN` writer - set will use posege of input capture signal to latch timestamp value"]
pub type CAPT_SNAP_POS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `CAPT_SNAP_NEG_EN` reader - No description avaiable"]
pub type CAPT_SNAP_NEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `CAPT_SNAP_NEG_EN` writer - No description avaiable"]
pub type CAPT_SNAP_NEG_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `COMP_EN` reader - set to enable compare, will be cleared by HW when compare event triggered"]
pub type COMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP_EN` writer - set to enable compare, will be cleared by HW when compare event triggered"]
pub type COMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `UPDATE_TIMER` writer - update timer with +/- ts_updt, pulse, clear after set"]
pub type UPDATE_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `INIT_TIMER` writer - initial timer with ts_updt, pulse, clear after set"]
pub type INIT_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `FINE_COARSE_SEL` reader - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
pub type FINE_COARSE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `FINE_COARSE_SEL` writer - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
pub type FINE_COARSE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
#[doc = "Field `TIMER_ENABLE` reader - No description avaiable"]
pub type TIMER_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_ENABLE` writer - No description avaiable"]
pub type TIMER_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTPC_1_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
    #[inline(always)]
    pub fn subsec_digital_rollover(&self) -> SUBSEC_DIGITAL_ROLLOVER_R {
        SUBSEC_DIGITAL_ROLLOVER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
    #[inline(always)]
    pub fn capt_snap_keep(&self) -> CAPT_SNAP_KEEP_R {
        CAPT_SNAP_KEEP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - set will use posege of input capture signal to latch timestamp value"]
    #[inline(always)]
    pub fn capt_snap_pos_en(&self) -> CAPT_SNAP_POS_EN_R {
        CAPT_SNAP_POS_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    pub fn capt_snap_neg_en(&self) -> CAPT_SNAP_NEG_EN_R {
        CAPT_SNAP_NEG_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - set to enable compare, will be cleared by HW when compare event triggered"]
    #[inline(always)]
    pub fn comp_en(&self) -> COMP_EN_R {
        COMP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
    #[inline(always)]
    pub fn fine_coarse_sel(&self) -> FINE_COARSE_SEL_R {
        FINE_COARSE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn timer_enable(&self) -> TIMER_ENABLE_R {
        TIMER_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
    #[inline(always)]
    pub fn subsec_digital_rollover(&mut self) -> SUBSEC_DIGITAL_ROLLOVER_W<9> {
        SUBSEC_DIGITAL_ROLLOVER_W::new(self)
    }
    #[doc = "Bit 8 - set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
    #[inline(always)]
    pub fn capt_snap_keep(&mut self) -> CAPT_SNAP_KEEP_W<8> {
        CAPT_SNAP_KEEP_W::new(self)
    }
    #[doc = "Bit 7 - set will use posege of input capture signal to latch timestamp value"]
    #[inline(always)]
    pub fn capt_snap_pos_en(&mut self) -> CAPT_SNAP_POS_EN_W<7> {
        CAPT_SNAP_POS_EN_W::new(self)
    }
    #[doc = "Bit 6 - No description avaiable"]
    #[inline(always)]
    pub fn capt_snap_neg_en(&mut self) -> CAPT_SNAP_NEG_EN_W<6> {
        CAPT_SNAP_NEG_EN_W::new(self)
    }
    #[doc = "Bit 4 - set to enable compare, will be cleared by HW when compare event triggered"]
    #[inline(always)]
    pub fn comp_en(&mut self) -> COMP_EN_W<4> {
        COMP_EN_W::new(self)
    }
    #[doc = "Bit 3 - update timer with +/- ts_updt, pulse, clear after set"]
    #[inline(always)]
    pub fn update_timer(&mut self) -> UPDATE_TIMER_W<3> {
        UPDATE_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - initial timer with ts_updt, pulse, clear after set"]
    #[inline(always)]
    pub fn init_timer(&mut self) -> INIT_TIMER_W<2> {
        INIT_TIMER_W::new(self)
    }
    #[doc = "Bit 1 - 0: fine update, ns counter add ss_incr\\[7:0\\]
each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\]
each clk"]
    #[inline(always)]
    pub fn fine_coarse_sel(&mut self) -> FINE_COARSE_SEL_W<1> {
        FINE_COARSE_SEL_W::new(self)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn timer_enable(&mut self) -> TIMER_ENABLE_W<0> {
        TIMER_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_1_ctrl0](index.html) module"]
pub struct PTPC_1_CTRL0_SPEC;
impl crate::RegisterSpec for PTPC_1_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_1_ctrl0::R](R) reader structure"]
impl crate::Readable for PTPC_1_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpc_1_ctrl0::W](W) writer structure"]
impl crate::Writable for PTPC_1_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPC_1_CTRL0 to value 0"]
impl crate::Resettable for PTPC_1_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
