#[doc = "Register `SEQ_CFG0` reader"]
pub struct R(crate::R<SEQ_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_CFG0` writer"]
pub struct W(crate::W<SEQ_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_CFG0_SPEC>;
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
impl From<crate::W<SEQ_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HW_TRIG_EN` reader - set to enable external HW trigger, only trigger on posedge"]
pub type HW_TRIG_EN_R = crate::BitReader<bool>;
#[doc = "Field `HW_TRIG_EN` writer - set to enable external HW trigger, only trigger on posedge"]
pub type HW_TRIG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_CFG0_SPEC, bool, O>;
#[doc = "Field `SW_TRIG_EN` reader - set to enable SW trigger"]
pub type SW_TRIG_EN_R = crate::BitReader<bool>;
#[doc = "Field `SW_TRIG_EN` writer - set to enable SW trigger"]
pub type SW_TRIG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_CFG0_SPEC, bool, O>;
#[doc = "Field `SW_TRIG` writer - SW trigger, pulse signal, cleared by HW one cycle later"]
pub type SW_TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_CFG0_SPEC, bool, O>;
#[doc = "Field `CONT_EN` reader - if set, HW will continue process the queue till end(seq_len) after trigger once"]
pub type CONT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CONT_EN` writer - if set, HW will continue process the queue till end(seq_len) after trigger once"]
pub type CONT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_CFG0_SPEC, bool, O>;
#[doc = "Field `RESTART_EN` reader - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
pub type RESTART_EN_R = crate::BitReader<bool>;
#[doc = "Field `RESTART_EN` writer - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
pub type RESTART_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_CFG0_SPEC, bool, O>;
#[doc = "Field `SEQ_LEN` reader - sequence queue length, 0 for one, 0xF for 16"]
pub type SEQ_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ_LEN` writer - sequence queue length, 0 for one, 0xF for 16"]
pub type SEQ_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ_CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CYCLE` reader - current dma write cycle bit"]
pub type CYCLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - set to enable external HW trigger, only trigger on posedge"]
    #[inline(always)]
    pub fn hw_trig_en(&self) -> HW_TRIG_EN_R {
        HW_TRIG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set to enable SW trigger"]
    #[inline(always)]
    pub fn sw_trig_en(&self) -> SW_TRIG_EN_R {
        SW_TRIG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - if set, HW will continue process the queue till end(seq_len) after trigger once"]
    #[inline(always)]
    pub fn cont_en(&self) -> CONT_EN_R {
        CONT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
    #[inline(always)]
    pub fn restart_en(&self) -> RESTART_EN_R {
        RESTART_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - sequence queue length, 0 for one, 0xF for 16"]
    #[inline(always)]
    pub fn seq_len(&self) -> SEQ_LEN_R {
        SEQ_LEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - current dma write cycle bit"]
    #[inline(always)]
    pub fn cycle(&self) -> CYCLE_R {
        CYCLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set to enable external HW trigger, only trigger on posedge"]
    #[inline(always)]
    #[must_use]
    pub fn hw_trig_en(&mut self) -> HW_TRIG_EN_W<0> {
        HW_TRIG_EN_W::new(self)
    }
    #[doc = "Bit 1 - set to enable SW trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sw_trig_en(&mut self) -> SW_TRIG_EN_W<1> {
        SW_TRIG_EN_W::new(self)
    }
    #[doc = "Bit 2 - SW trigger, pulse signal, cleared by HW one cycle later"]
    #[inline(always)]
    #[must_use]
    pub fn sw_trig(&mut self) -> SW_TRIG_W<2> {
        SW_TRIG_W::new(self)
    }
    #[doc = "Bit 3 - if set, HW will continue process the queue till end(seq_len) after trigger once"]
    #[inline(always)]
    #[must_use]
    pub fn cont_en(&mut self) -> CONT_EN_W<3> {
        CONT_EN_W::new(self)
    }
    #[doc = "Bit 4 - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
    #[inline(always)]
    #[must_use]
    pub fn restart_en(&mut self) -> RESTART_EN_W<4> {
        RESTART_EN_W::new(self)
    }
    #[doc = "Bits 8:11 - sequence queue length, 0 for one, 0xF for 16"]
    #[inline(always)]
    #[must_use]
    pub fn seq_len(&mut self) -> SEQ_LEN_W<8> {
        SEQ_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_cfg0](index.html) module"]
pub struct SEQ_CFG0_SPEC;
impl crate::RegisterSpec for SEQ_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_cfg0::R](R) reader structure"]
impl crate::Readable for SEQ_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_cfg0::W](W) writer structure"]
impl crate::Writable for SEQ_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_CFG0 to value 0"]
impl crate::Resettable for SEQ_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
