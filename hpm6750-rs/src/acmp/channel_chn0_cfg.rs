#[doc = "Register `CHANNEL_CHN0_CFG` reader"]
pub struct R(crate::R<CHANNEL_CHN0_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CHN0_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CHN0_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CHN0_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CHN0_CFG` writer"]
pub struct W(crate::W<CHANNEL_CHN0_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CHN0_CFG_SPEC>;
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
impl From<crate::W<CHANNEL_CHN0_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CHN0_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYST` reader - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DACEN` reader - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
pub type DACEN_R = crate::BitReader<bool>;
#[doc = "Field `DACEN` writer - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `HPMODE` reader - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
pub type HPMODE_R = crate::BitReader<bool>;
#[doc = "Field `HPMODE` writer - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
pub type HPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `CMPEN` reader - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
pub type CMPEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPEN` writer - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
pub type CMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `MINSEL` reader - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type MINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINSEL` writer - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type MINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PINSEL` reader - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type PINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINSEL` writer - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
pub type PINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `CMPOEN` reader - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
pub type CMPOEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPOEN` writer - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
pub type CMPOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `FLTBYPS` reader - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
pub type FLTBYPS_R = crate::BitReader<bool>;
#[doc = "Field `FLTBYPS` writer - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
pub type FLTBYPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `WINEN` reader - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
pub type WINEN_R = crate::BitReader<bool>;
#[doc = "Field `WINEN` writer - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
pub type WINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `OPOL` reader - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
pub type OPOL_R = crate::BitReader<bool>;
#[doc = "Field `OPOL` writer - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
pub type OPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `FLTMODE` reader - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
pub type FLTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTMODE` writer - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
pub type FLTMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SYNCEN` reader - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
pub type SYNCEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCEN` writer - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
pub type SYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, bool, O>;
#[doc = "Field `FLTLEN` reader - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
pub type FLTLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLTLEN` writer - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
pub type FLTLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CHN0_CFG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 30:31 - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bit 29 - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    #[inline(always)]
    pub fn hpmode(&self) -> HPMODE_R {
        HPMODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 24:26 - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    pub fn minsel(&self) -> MINSEL_R {
        MINSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 20:22 - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 19 - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    #[inline(always)]
    pub fn cmpoen(&self) -> CMPOEN_R {
        CMPOEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    #[inline(always)]
    pub fn fltbyps(&self) -> FLTBYPS_R {
        FLTBYPS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    #[inline(always)]
    pub fn winen(&self) -> WINEN_R {
        WINEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    #[inline(always)]
    pub fn opol(&self) -> OPOL_R {
        OPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 13:15 - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    #[inline(always)]
    pub fn fltmode(&self) -> FLTMODE_R {
        FLTMODE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 12 - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 0:11 - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    #[inline(always)]
    pub fn fltlen(&self) -> FLTLEN_R {
        FLTLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<30> {
        HYST_W::new(self)
    }
    #[doc = "Bit 29 - This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W<29> {
        DACEN_W::new(self)
    }
    #[doc = "Bit 28 - This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    #[inline(always)]
    pub fn hpmode(&mut self) -> HPMODE_W<28> {
        HPMODE_W::new(self)
    }
    #[doc = "Bit 27 - This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W<27> {
        CMPEN_W::new(self)
    }
    #[doc = "Bits 24:26 - PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    pub fn minsel(&mut self) -> MINSEL_W<24> {
        MINSEL_W::new(self)
    }
    #[doc = "Bits 20:22 - MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W<20> {
        PINSEL_W::new(self)
    }
    #[doc = "Bit 19 - This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    #[inline(always)]
    pub fn cmpoen(&mut self) -> CMPOEN_W<19> {
        CMPOEN_W::new(self)
    }
    #[doc = "Bit 18 - This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    #[inline(always)]
    pub fn fltbyps(&mut self) -> FLTBYPS_W<18> {
        FLTBYPS_W::new(self)
    }
    #[doc = "Bit 17 - This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    #[inline(always)]
    pub fn winen(&mut self) -> WINEN_W<17> {
        WINEN_W::new(self)
    }
    #[doc = "Bit 16 - The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    #[inline(always)]
    pub fn opol(&mut self) -> OPOL_W<16> {
        OPOL_W::new(self)
    }
    #[doc = "Bits 13:15 - This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    #[inline(always)]
    pub fn fltmode(&mut self) -> FLTMODE_W<13> {
        FLTMODE_W::new(self)
    }
    #[doc = "Bit 12 - This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W<12> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bits 0:11 - This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    #[inline(always)]
    pub fn fltlen(&mut self) -> FLTLEN_W<0> {
        FLTLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_chn0_cfg](index.html) module"]
pub struct CHANNEL_CHN0_CFG_SPEC;
impl crate::RegisterSpec for CHANNEL_CHN0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_chn0_cfg::R](R) reader structure"]
impl crate::Readable for CHANNEL_CHN0_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_chn0_cfg::W](W) writer structure"]
impl crate::Writable for CHANNEL_CHN0_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANNEL_CHN0_CFG to value 0"]
impl crate::Resettable for CHANNEL_CHN0_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
