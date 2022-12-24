#[doc = "Register `RX_CFGR` reader"]
pub struct R(crate::R<RX_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CFGR` writer"]
pub struct W(crate::W<RX_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CFGR_SPEC>;
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
impl From<crate::W<RX_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_MAX` reader - CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc"]
pub type CH_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_MAX` writer - CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc"]
pub type CH_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RX_CFGR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 6:10 - CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc"]
    #[inline(always)]
    pub fn ch_max(&self) -> CH_MAX_R {
        CH_MAX_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:10 - CH_MAX\\[3:0\\]
is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels etc"]
    #[inline(always)]
    #[must_use]
    pub fn ch_max(&mut self) -> CH_MAX_W<6> {
        CH_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_cfgr](index.html) module"]
pub struct RX_CFGR_SPEC;
impl crate::RegisterSpec for RX_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_cfgr::R](R) reader structure"]
impl crate::Readable for RX_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_cfgr::W](W) writer structure"]
impl crate::Writable for RX_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CFGR to value 0"]
impl crate::Resettable for RX_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
