#[doc = "Register `IRC32K_CFG` reader"]
pub struct R(crate::R<IRC32K_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRC32K_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRC32K_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRC32K_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRC32K_CFG` writer"]
pub struct W(crate::W<IRC32K_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRC32K_CFG_SPEC>;
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
impl From<crate::W<IRC32K_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRC32K_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRC_TRIMMED` reader - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
pub type IRC_TRIMMED_R = crate::BitReader<bool>;
#[doc = "Field `IRC_TRIMMED` writer - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
pub type IRC_TRIMMED_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRC32K_CFG_SPEC, bool, O>;
#[doc = "Field `CAPEX7_TRIM` reader - IRC32K bit 7"]
pub type CAPEX7_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `CAPEX7_TRIM` writer - IRC32K bit 7"]
pub type CAPEX7_TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRC32K_CFG_SPEC, bool, O>;
#[doc = "Field `CAPEX6_TRIM` reader - IRC32K bit 6"]
pub type CAPEX6_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `CAPEX6_TRIM` writer - IRC32K bit 6"]
pub type CAPEX6_TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRC32K_CFG_SPEC, bool, O>;
#[doc = "Field `CAP_TRIM` reader - capacitor trim bits"]
pub type CAP_TRIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAP_TRIM` writer - capacitor trim bits"]
pub type CAP_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRC32K_CFG_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bit 31 - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
    #[inline(always)]
    pub fn irc_trimmed(&self) -> IRC_TRIMMED_R {
        IRC_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 23 - IRC32K bit 7"]
    #[inline(always)]
    pub fn capex7_trim(&self) -> CAPEX7_TRIM_R {
        CAPEX7_TRIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - IRC32K bit 6"]
    #[inline(always)]
    pub fn capex6_trim(&self) -> CAPEX6_TRIM_R {
        CAPEX6_TRIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 0:8 - capacitor trim bits"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CAP_TRIM_R {
        CAP_TRIM_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
    #[inline(always)]
    pub fn irc_trimmed(&mut self) -> IRC_TRIMMED_W<31> {
        IRC_TRIMMED_W::new(self)
    }
    #[doc = "Bit 23 - IRC32K bit 7"]
    #[inline(always)]
    pub fn capex7_trim(&mut self) -> CAPEX7_TRIM_W<23> {
        CAPEX7_TRIM_W::new(self)
    }
    #[doc = "Bit 22 - IRC32K bit 6"]
    #[inline(always)]
    pub fn capex6_trim(&mut self) -> CAPEX6_TRIM_W<22> {
        CAPEX6_TRIM_W::new(self)
    }
    #[doc = "Bits 0:8 - capacitor trim bits"]
    #[inline(always)]
    pub fn cap_trim(&mut self) -> CAP_TRIM_W<0> {
        CAP_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On-chip 32k oscillator config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irc32k_cfg](index.html) module"]
pub struct IRC32K_CFG_SPEC;
impl crate::RegisterSpec for IRC32K_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irc32k_cfg::R](R) reader structure"]
impl crate::Readable for IRC32K_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irc32k_cfg::W](W) writer structure"]
impl crate::Writable for IRC32K_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRC32K_CFG to value 0"]
impl crate::Resettable for IRC32K_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
