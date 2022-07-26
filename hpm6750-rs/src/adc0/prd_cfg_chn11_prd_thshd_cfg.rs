#[doc = "Register `PRD_CFG_CHN11_PRD_THSHD_CFG` reader"]
pub struct R(crate::R<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRD_CFG_CHN11_PRD_THSHD_CFG` writer"]
pub struct W(crate::W<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>;
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
impl From<crate::W<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THSHDH` reader - threshold high, assert interrupt(if enabled) if result exceed high or low."]
pub type THSHDH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THSHDH` writer - threshold high, assert interrupt(if enabled) if result exceed high or low."]
pub type THSHDH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `THSHDL` reader - threshold low"]
pub type THSHDL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THSHDL` writer - threshold low"]
pub type THSHDL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - threshold high, assert interrupt(if enabled) if result exceed high or low."]
    #[inline(always)]
    pub fn thshdh(&self) -> THSHDH_R {
        THSHDH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - threshold low"]
    #[inline(always)]
    pub fn thshdl(&self) -> THSHDL_R {
        THSHDL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - threshold high, assert interrupt(if enabled) if result exceed high or low."]
    #[inline(always)]
    pub fn thshdh(&mut self) -> THSHDH_W<16> {
        THSHDH_W::new(self)
    }
    #[doc = "Bits 0:15 - threshold low"]
    #[inline(always)]
    pub fn thshdl(&mut self) -> THSHDL_W<0> {
        THSHDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prd_cfg_chn11_prd_thshd_cfg](index.html) module"]
pub struct PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC;
impl crate::RegisterSpec for PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prd_cfg_chn11_prd_thshd_cfg::R](R) reader structure"]
impl crate::Readable for PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prd_cfg_chn11_prd_thshd_cfg::W](W) writer structure"]
impl crate::Writable for PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRD_CFG_CHN11_PRD_THSHD_CFG to value 0"]
impl crate::Resettable for PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
