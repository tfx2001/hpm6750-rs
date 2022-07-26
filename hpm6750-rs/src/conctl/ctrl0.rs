#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENET1_RXCLK_DLY_SEL` reader - No description avaiable"]
pub type ENET1_RXCLK_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET1_RXCLK_DLY_SEL` writer - No description avaiable"]
pub type ENET1_RXCLK_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ENET1_TXCLK_DLY_SEL` reader - No description avaiable"]
pub type ENET1_TXCLK_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET1_TXCLK_DLY_SEL` writer - No description avaiable"]
pub type ENET1_TXCLK_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ENET0_RXCLK_DLY_SEL` reader - No description avaiable"]
pub type ENET0_RXCLK_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET0_RXCLK_DLY_SEL` writer - No description avaiable"]
pub type ENET0_RXCLK_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ENET0_TXCLK_DLY_SEL` reader - No description avaiable"]
pub type ENET0_TXCLK_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENET0_TXCLK_DLY_SEL` writer - No description avaiable"]
pub type ENET0_TXCLK_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 15:19 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_rxclk_dly_sel(&self) -> ENET1_RXCLK_DLY_SEL_R {
        ENET1_RXCLK_DLY_SEL_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_txclk_dly_sel(&self) -> ENET1_TXCLK_DLY_SEL_R {
        ENET1_TXCLK_DLY_SEL_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_rxclk_dly_sel(&self) -> ENET0_RXCLK_DLY_SEL_R {
        ENET0_RXCLK_DLY_SEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_txclk_dly_sel(&self) -> ENET0_TXCLK_DLY_SEL_R {
        ENET0_TXCLK_DLY_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 15:19 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_rxclk_dly_sel(&mut self) -> ENET1_RXCLK_DLY_SEL_W<15> {
        ENET1_RXCLK_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 10:14 - No description avaiable"]
    #[inline(always)]
    pub fn enet1_txclk_dly_sel(&mut self) -> ENET1_TXCLK_DLY_SEL_W<10> {
        ENET1_TXCLK_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 5:9 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_rxclk_dly_sel(&mut self) -> ENET0_RXCLK_DLY_SEL_W<5> {
        ENET0_RXCLK_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 0:4 - No description avaiable"]
    #[inline(always)]
    pub fn enet0_txclk_dly_sel(&mut self) -> ENET0_TXCLK_DLY_SEL_W<0> {
        ENET0_TXCLK_DLY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
