#[doc = "Register `PRD_CFG_CHN1_PRD_CFG` reader"]
pub struct R(crate::R<PRD_CFG_CHN1_PRD_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRD_CFG_CHN1_PRD_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRD_CFG_CHN1_PRD_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRD_CFG_CHN1_PRD_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRD_CFG_CHN1_PRD_CFG` writer"]
pub struct W(crate::W<PRD_CFG_CHN1_PRD_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRD_CFG_CHN1_PRD_CFG_SPEC>;
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
impl From<crate::W<PRD_CFG_CHN1_PRD_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRD_CFG_CHN1_PRD_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALE` reader - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
pub type PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALE` writer - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
pub type PRESCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRD_CFG_CHN1_PRD_CFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `PRD` reader - conver period, with prescale. Set to 0 means disable current channel"]
pub type PRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRD` writer - conver period, with prescale. Set to 0 means disable current channel"]
pub type PRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRD_CFG_CHN1_PRD_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:12 - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - conver period, with prescale. Set to 0 means disable current channel"]
    #[inline(always)]
    pub fn prd(&self) -> PRD_R {
        PRD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - 0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W<8> {
        PRESCALE_W::new(self)
    }
    #[doc = "Bits 0:7 - conver period, with prescale. Set to 0 means disable current channel"]
    #[inline(always)]
    pub fn prd(&mut self) -> PRD_W<0> {
        PRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prd_cfg_chn1_prd_cfg](index.html) module"]
pub struct PRD_CFG_CHN1_PRD_CFG_SPEC;
impl crate::RegisterSpec for PRD_CFG_CHN1_PRD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prd_cfg_chn1_prd_cfg::R](R) reader structure"]
impl crate::Readable for PRD_CFG_CHN1_PRD_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prd_cfg_chn1_prd_cfg::W](W) writer structure"]
impl crate::Writable for PRD_CFG_CHN1_PRD_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRD_CFG_CHN1_PRD_CFG to value 0"]
impl crate::Resettable for PRD_CFG_CHN1_PRD_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
