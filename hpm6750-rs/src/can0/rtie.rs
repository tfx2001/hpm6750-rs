#[doc = "Register `RTIE` reader"]
pub struct R(crate::R<RTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTIE` writer"]
pub struct W(crate::W<RTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTIE_SPEC>;
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
impl From<crate::W<RTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSFF` reader - If TTEN=0 or TTTBM=0: Transmit Secondary buffer Full Flag 1 - The STB is filled with the maximal number of messages. 0 - The STB is not filled with the maximal number of messages. If the STB is disabled using STB_DISABLE, then TSFF=0. If TTEN=1 and TTTBM=1: Transmit buffer Slot Full Flag 1 - The buffer slot selected by TBPTR is filled. 0 - The buffer slot selected by TBPTR is empty."]
pub type TSFF_R = crate::BitReader<bool>;
#[doc = "Field `EIE` reader - Error Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `TSIE` reader - Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type TSIE_R = crate::BitReader<bool>;
#[doc = "Field `TSIE` writer - Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `TPIE` reader - Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type TPIE_R = crate::BitReader<bool>;
#[doc = "Field `TPIE` writer - Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type TPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `RAFIE` reader - RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type RAFIE_R = crate::BitReader<bool>;
#[doc = "Field `RAFIE` writer - RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type RAFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `RFIE` reader - RB Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type RFIE_R = crate::BitReader<bool>;
#[doc = "Field `RFIE` writer - RB Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type RFIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `ROIE` reader - RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type ROIE_R = crate::BitReader<bool>;
#[doc = "Field `ROIE` writer - RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type ROIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive Interrupt Enable 0 – Disabled, 1 – Enabled"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If TTEN=0 or TTTBM=0: Transmit Secondary buffer Full Flag 1 - The STB is filled with the maximal number of messages. 0 - The STB is not filled with the maximal number of messages. If the STB is disabled using STB_DISABLE, then TSFF=0. If TTEN=1 and TTTBM=1: Transmit buffer Slot Full Flag 1 - The buffer slot selected by TBPTR is filled. 0 - The buffer slot selected by TBPTR is empty."]
    #[inline(always)]
    pub fn tsff(&self) -> TSFF_R {
        TSFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn rafie(&self) -> RAFIE_R {
        RAFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RB Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn rfie(&self) -> RFIE_R {
        RFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Error Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<1> {
        EIE_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<2> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<3> {
        TPIE_W::new(self)
    }
    #[doc = "Bit 4 - RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rafie(&mut self) -> RAFIE_W<4> {
        RAFIE_W::new(self)
    }
    #[doc = "Bit 5 - RB Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rfie(&mut self) -> RFIE_W<5> {
        RFIE_W::new(self)
    }
    #[doc = "Bit 6 - RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<6> {
        ROIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive Interrupt Enable 0 – Disabled, 1 – Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<7> {
        RIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive and Transmit Interrupt Enable Register RTIE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtie](index.html) module"]
pub struct RTIE_SPEC;
impl crate::RegisterSpec for RTIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtie::R](R) reader structure"]
impl crate::Readable for RTIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtie::W](W) writer structure"]
impl crate::Writable for RTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTIE to value 0xfe"]
impl crate::Resettable for RTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0xfe;
}
