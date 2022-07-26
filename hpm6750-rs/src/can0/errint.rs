#[doc = "Register `ERRINT` reader"]
pub struct R(crate::R<ERRINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRINT` writer"]
pub struct W(crate::W<ERRINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRINT_SPEC>;
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
impl From<crate::W<ERRINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWARN` reader - Error WARNing limit reached 1 - One of the error counters RECNT or TECNT is equal or bigger than EWL0 - The values in both counters are less than EWL."]
pub type EWARN_R = crate::BitReader<bool>;
#[doc = "Field `EPASS` reader - Error Passive mode active 0 - not active (node is error active) 1 - active (node is error passive)"]
pub type EPASS_R = crate::BitReader<bool>;
#[doc = "Field `EPIE` reader - Error Passive Interrupt Enable"]
pub type EPIE_R = crate::BitReader<bool>;
#[doc = "Field `EPIE` writer - Error Passive Interrupt Enable"]
pub type EPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `EPIF` reader - Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
pub type EPIF_R = crate::BitReader<bool>;
#[doc = "Field `EPIF` writer - Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
pub type EPIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable"]
pub type ALIE_R = crate::BitReader<bool>;
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `ALIF` reader - Arbitration Lost Interrupt Flag"]
pub type ALIF_R = crate::BitReader<bool>;
#[doc = "Field `ALIF` writer - Arbitration Lost Interrupt Flag"]
pub type ALIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable"]
pub type BEIE_R = crate::BitReader<bool>;
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable"]
pub type BEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
#[doc = "Field `BEIF` reader - Bus Error Interrupt Flag"]
pub type BEIF_R = crate::BitReader<bool>;
#[doc = "Field `BEIF` writer - Bus Error Interrupt Flag"]
pub type BEIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRINT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Error WARNing limit reached 1 - One of the error counters RECNT or TECNT is equal or bigger than EWL0 - The values in both counters are less than EWL."]
    #[inline(always)]
    pub fn ewarn(&self) -> EWARN_R {
        EWARN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Passive mode active 0 - not active (node is error active) 1 - active (node is error passive)"]
    #[inline(always)]
    pub fn epass(&self) -> EPASS_R {
        EPASS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
    #[inline(always)]
    pub fn epif(&self) -> EPIF_R {
        EPIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn alif(&self) -> ALIF_R {
        ALIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn beif(&self) -> BEIF_R {
        BEIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Error Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&mut self) -> EPIE_W<5> {
        EPIE_W::new(self)
    }
    #[doc = "Bit 4 - Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
    #[inline(always)]
    pub fn epif(&mut self) -> EPIF_W<4> {
        EPIF_W::new(self)
    }
    #[doc = "Bit 3 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W<3> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 2 - Arbitration Lost Interrupt Flag"]
    #[inline(always)]
    pub fn alif(&mut self) -> ALIF_W<2> {
        ALIF_W::new(self)
    }
    #[doc = "Bit 1 - Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&mut self) -> BEIE_W<1> {
        BEIE_W::new(self)
    }
    #[doc = "Bit 0 - Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn beif(&mut self) -> BEIF_W<0> {
        BEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERRor INTerrupt Enable and Flag Register ERRINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errint](index.html) module"]
pub struct ERRINT_SPEC;
impl crate::RegisterSpec for ERRINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [errint::R](R) reader structure"]
impl crate::Readable for ERRINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errint::W](W) writer structure"]
impl crate::Writable for ERRINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRINT to value 0"]
impl crate::Resettable for ERRINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
