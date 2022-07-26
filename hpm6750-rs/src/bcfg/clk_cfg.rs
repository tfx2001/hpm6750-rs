#[doc = "Register `CLK_CFG` reader"]
pub struct R(crate::R<CLK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CFG` writer"]
pub struct W(crate::W<CLK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG_SPEC>;
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
impl From<crate::W<CLK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_SEL` reader - crystal selected"]
pub type XTAL_SEL_R = crate::BitReader<bool>;
#[doc = "Field `KEEP_IRC` reader - force irc32k run"]
pub type KEEP_IRC_R = crate::BitReader<bool>;
#[doc = "Field `KEEP_IRC` writer - force irc32k run"]
pub type KEEP_IRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG_SPEC, bool, O>;
#[doc = "Field `FORCE_XTAL` reader - force switch to crystal"]
pub type FORCE_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_XTAL` writer - force switch to crystal"]
pub type FORCE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - crystal selected"]
    #[inline(always)]
    pub fn xtal_sel(&self) -> XTAL_SEL_R {
        XTAL_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 16 - force irc32k run"]
    #[inline(always)]
    pub fn keep_irc(&self) -> KEEP_IRC_R {
        KEEP_IRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 4 - force switch to crystal"]
    #[inline(always)]
    pub fn force_xtal(&self) -> FORCE_XTAL_R {
        FORCE_XTAL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - force irc32k run"]
    #[inline(always)]
    pub fn keep_irc(&mut self) -> KEEP_IRC_W<16> {
        KEEP_IRC_W::new(self)
    }
    #[doc = "Bit 4 - force switch to crystal"]
    #[inline(always)]
    pub fn force_xtal(&mut self) -> FORCE_XTAL_W<4> {
        FORCE_XTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg](index.html) module"]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg::R](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg::W](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
