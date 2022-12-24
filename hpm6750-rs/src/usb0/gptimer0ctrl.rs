#[doc = "Register `GPTIMER0CTRL` reader"]
pub struct R(crate::R<GPTIMER0CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTIMER0CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTIMER0CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTIMER0CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTIMER0CTRL` writer"]
pub struct W(crate::W<GPTIMER0CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTIMER0CTRL_SPEC>;
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
impl From<crate::W<GPTIMER0CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTIMER0CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTCNT` reader - GPTCNT General Purpose Timer Counter. This field is the count value of the countdown timer."]
pub type GPTCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPTMODE` reader - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
pub type GPTMODE_R = crate::BitReader<bool>;
#[doc = "Field `GPTMODE` writer - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
pub type GPTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTIMER0CTRL_SPEC, bool, O>;
#[doc = "Field `GPTRST` writer - GPTRST General Purpose Timer Reset 0 - No action 1 - Load counter value from GPTLD bits in n_GPTIMER0LD"]
pub type GPTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTIMER0CTRL_SPEC, bool, O>;
#[doc = "Field `GPTRUN` reader - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
pub type GPTRUN_R = crate::BitReader<bool>;
#[doc = "Field `GPTRUN` writer - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
pub type GPTRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTIMER0CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - GPTCNT General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub fn gptcnt(&self) -> GPTCNT_R {
        GPTCNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
    #[inline(always)]
    pub fn gptmode(&self) -> GPTMODE_R {
        GPTMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
    #[inline(always)]
    pub fn gptrun(&self) -> GPTRUN_R {
        GPTRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - GPTMODE General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again. 0 - One Shot Mode 1 - Repeat Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gptmode(&mut self) -> GPTMODE_W<24> {
        GPTMODE_W::new(self)
    }
    #[doc = "Bit 30 - GPTRST General Purpose Timer Reset 0 - No action 1 - Load counter value from GPTLD bits in n_GPTIMER0LD"]
    #[inline(always)]
    #[must_use]
    pub fn gptrst(&mut self) -> GPTRST_W<30> {
        GPTRST_W::new(self)
    }
    #[doc = "Bit 31 - GPTRUN General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit. 0 - Stop counting 1 - Run"]
    #[inline(always)]
    #[must_use]
    pub fn gptrun(&mut self) -> GPTRUN_W<31> {
        GPTRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Timer #0 Controller Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer0ctrl](index.html) module"]
pub struct GPTIMER0CTRL_SPEC;
impl crate::RegisterSpec for GPTIMER0CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptimer0ctrl::R](R) reader structure"]
impl crate::Readable for GPTIMER0CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptimer0ctrl::W](W) writer structure"]
impl crate::Writable for GPTIMER0CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTIMER0CTRL to value 0"]
impl crate::Resettable for GPTIMER0CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
