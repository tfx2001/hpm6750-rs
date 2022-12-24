#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST` reader - Software Reset Reset all internal logic in SEMC except configuration register"]
pub type RST_R = crate::BitReader<bool>;
#[doc = "Field `RST` writer - Software Reset Reset all internal logic in SEMC except configuration register"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DIS` reader - Module Disable 0b - Module enabled 1b - Module disabled"]
pub type DIS_R = crate::BitReader<bool>;
#[doc = "Field `DIS` writer - Module Disable 0b - Module enabled 1b - Module disabled"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DQS` reader - DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad"]
pub type DQS_R = crate::BitReader<bool>;
#[doc = "Field `DQS` writer - DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad"]
pub type DQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CTO` reader - Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
pub type CTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTO` writer - Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
pub type CTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `BTO` reader - Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31"]
pub type BTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BTO` writer - Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31"]
pub type BTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Software Reset Reset all internal logic in SEMC except configuration register"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable 0b - Module enabled 1b - Module disabled"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad"]
    #[inline(always)]
    pub fn dqs(&self) -> DQS_R {
        DQS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
    #[inline(always)]
    pub fn cto(&self) -> CTO_R {
        CTO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31"]
    #[inline(always)]
    pub fn bto(&self) -> BTO_R {
        BTO_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset Reset all internal logic in SEMC except configuration register"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<0> {
        RST_W::new(self)
    }
    #[doc = "Bit 1 - Module Disable 0b - Module enabled 1b - Module disabled"]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<1> {
        DIS_W::new(self)
    }
    #[doc = "Bit 2 - DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad"]
    #[inline(always)]
    #[must_use]
    pub fn dqs(&mut self) -> DQS_W<2> {
        DQS_W::new(self)
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn cto(&mut self) -> CTO_W<16> {
        CTO_W::new(self)
    }
    #[doc = "Bits 24:28 - Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31"]
    #[inline(always)]
    #[must_use]
    pub fn bto(&mut self) -> BTO_W<24> {
        BTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
