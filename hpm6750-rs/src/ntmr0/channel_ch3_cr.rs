#[doc = "Register `CHANNEL_CH3_CR` reader"]
pub struct R(crate::R<CHANNEL_CH3_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH3_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH3_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH3_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CH3_CR` writer"]
pub struct W(crate::W<CHANNEL_CH3_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CH3_CR_SPEC>;
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
impl From<crate::W<CHANNEL_CH3_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CH3_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPMODE` reader - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
pub type CAPMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPMODE` writer - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
pub type CAPMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNEL_CH3_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBGPAUSE` reader - 1- counter will pause if chip is in debug mode"]
pub type DBGPAUSE_R = crate::BitReader<bool>;
#[doc = "Field `DBGPAUSE` writer - 1- counter will pause if chip is in debug mode"]
pub type DBGPAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `SWSYNCIEN` reader - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
pub type SWSYNCIEN_R = crate::BitReader<bool>;
#[doc = "Field `SWSYNCIEN` writer - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
pub type SWSYNCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - 1- enable dma"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - 1- enable dma"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `DMASEL` reader - select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
pub type DMASEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMASEL` writer - select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
pub type DMASEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHANNEL_CH3_CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CMPEN` reader - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
pub type CMPEN_R = crate::BitReader<bool>;
#[doc = "Field `CMPEN` writer - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
pub type CMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `CMPINIT` reader - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
pub type CMPINIT_R = crate::BitReader<bool>;
#[doc = "Field `CMPINIT` writer - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
pub type CMPINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `CEN` reader - 1- counter enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - 1- counter enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `SYNCIREN` reader - 1- SYNCI is valid on its rising edge"]
pub type SYNCIREN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCIREN` writer - 1- SYNCI is valid on its rising edge"]
pub type SYNCIREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `SYNCIFEN` reader - 1- SYNCI is valid on its falling edge"]
pub type SYNCIFEN_R = crate::BitReader<bool>;
#[doc = "Field `SYNCIFEN` writer - 1- SYNCI is valid on its falling edge"]
pub type SYNCIFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `SYNCFLW` reader - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
pub type SYNCFLW_R = crate::BitReader<bool>;
#[doc = "Field `SYNCFLW` writer - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
pub type SYNCFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `CNTRST` reader - 1- reset counter"]
pub type CNTRST_R = crate::BitReader<bool>;
#[doc = "Field `CNTRST` writer - 1- reset counter"]
pub type CNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
#[doc = "Field `CNTUPT` writer - 1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
pub type CNTUPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CH3_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1- counter will pause if chip is in debug mode"]
    #[inline(always)]
    pub fn dbgpause(&self) -> DBGPAUSE_R {
        DBGPAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    #[inline(always)]
    pub fn swsyncien(&self) -> SWSYNCIEN_R {
        SWSYNCIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- enable dma"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    #[inline(always)]
    pub fn cmpinit(&self) -> CMPINIT_R {
        CMPINIT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1- counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- SYNCI is valid on its rising edge"]
    #[inline(always)]
    pub fn synciren(&self) -> SYNCIREN_R {
        SYNCIREN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1- SYNCI is valid on its falling edge"]
    #[inline(always)]
    pub fn syncifen(&self) -> SYNCIFEN_R {
        SYNCIFEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    #[inline(always)]
    pub fn syncflw(&self) -> SYNCFLW_R {
        SYNCFLW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1- reset counter"]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    #[inline(always)]
    #[must_use]
    pub fn capmode(&mut self) -> CAPMODE_W<0> {
        CAPMODE_W::new(self)
    }
    #[doc = "Bit 3 - 1- counter will pause if chip is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgpause(&mut self) -> DBGPAUSE_W<3> {
        DBGPAUSE_W::new(self)
    }
    #[doc = "Bit 4 - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn swsyncien(&mut self) -> SWSYNCIEN_W<4> {
        SWSYNCIEN_W::new(self)
    }
    #[doc = "Bit 5 - 1- enable dma"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 6:7 - select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<6> {
        DMASEL_W::new(self)
    }
    #[doc = "Bit 8 - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<8> {
        CMPEN_W::new(self)
    }
    #[doc = "Bit 9 - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    #[inline(always)]
    #[must_use]
    pub fn cmpinit(&mut self) -> CMPINIT_W<9> {
        CMPINIT_W::new(self)
    }
    #[doc = "Bit 10 - 1- counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<10> {
        CEN_W::new(self)
    }
    #[doc = "Bit 11 - 1- SYNCI is valid on its rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn synciren(&mut self) -> SYNCIREN_W<11> {
        SYNCIREN_W::new(self)
    }
    #[doc = "Bit 12 - 1- SYNCI is valid on its falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn syncifen(&mut self) -> SYNCIFEN_W<12> {
        SYNCIFEN_W::new(self)
    }
    #[doc = "Bit 13 - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn syncflw(&mut self) -> SYNCFLW_W<13> {
        SYNCFLW_W::new(self)
    }
    #[doc = "Bit 14 - 1- reset counter"]
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CNTRST_W<14> {
        CNTRST_W::new(self)
    }
    #[doc = "Bit 31 - 1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cntupt(&mut self) -> CNTUPT_W<31> {
        CNTUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch3_cr](index.html) module"]
pub struct CHANNEL_CH3_CR_SPEC;
impl crate::RegisterSpec for CHANNEL_CH3_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch3_cr::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH3_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_ch3_cr::W](W) writer structure"]
impl crate::Writable for CHANNEL_CH3_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNEL_CH3_CR to value 0"]
impl crate::Resettable for CHANNEL_CH3_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
