#[doc = "Register `DIRECTIO` reader"]
pub struct R(crate::R<DIRECTIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRECTIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRECTIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRECTIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIRECTIO` writer"]
pub struct W(crate::W<DIRECTIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRECTIO_SPEC>;
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
impl From<crate::W<DIRECTIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRECTIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS_I` reader - Status of the SPI CS (chip select) signal"]
pub type CS_I_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_I` reader - Status of the SPI SCLK signal"]
pub type SCLK_I_R = crate::BitReader<bool>;
#[doc = "Field `MOSI_I` reader - Status of the SPI MOSI signal"]
pub type MOSI_I_R = crate::BitReader<bool>;
#[doc = "Field `MISO_I` reader - Status of the SPI MISO signal"]
pub type MISO_I_R = crate::BitReader<bool>;
#[doc = "Field `WP_I` reader - Status of the SPI Flash write protect signal"]
pub type WP_I_R = crate::BitReader<bool>;
#[doc = "Field `HOLD_I` reader - Status of the SPI Flash hold signal"]
pub type HOLD_I_R = crate::BitReader<bool>;
#[doc = "Field `CS_O` reader - Output value for the SPI CS (chip select) signal"]
pub type CS_O_R = crate::BitReader<bool>;
#[doc = "Field `CS_O` writer - Output value for the SPI CS (chip select) signal"]
pub type CS_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `SCLK_O` reader - Output value for the SPI SCLK signal"]
pub type SCLK_O_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_O` writer - Output value for the SPI SCLK signal"]
pub type SCLK_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `MOSI_O` reader - Output value for the SPI MOSI signal"]
pub type MOSI_O_R = crate::BitReader<bool>;
#[doc = "Field `MOSI_O` writer - Output value for the SPI MOSI signal"]
pub type MOSI_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `MISO_O` reader - Output value for the SPI MISO signal"]
pub type MISO_O_R = crate::BitReader<bool>;
#[doc = "Field `MISO_O` writer - Output value for the SPI MISO signal"]
pub type MISO_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `WP_O` reader - Output value for the SPI Flash write protect signal"]
pub type WP_O_R = crate::BitReader<bool>;
#[doc = "Field `WP_O` writer - Output value for the SPI Flash write protect signal"]
pub type WP_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `HOLD_O` reader - Output value for the SPI Flash hold signal"]
pub type HOLD_O_R = crate::BitReader<bool>;
#[doc = "Field `HOLD_O` writer - Output value for the SPI Flash hold signal"]
pub type HOLD_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `CS_OE` reader - Output enable for SPI CS (chip select) signal"]
pub type CS_OE_R = crate::BitReader<bool>;
#[doc = "Field `CS_OE` writer - Output enable for SPI CS (chip select) signal"]
pub type CS_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `SCLK_OE` reader - Output enable for the SPI SCLK signal"]
pub type SCLK_OE_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_OE` writer - Output enable for the SPI SCLK signal"]
pub type SCLK_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `MOSI_OE` reader - Output enable for the SPI MOSI signal"]
pub type MOSI_OE_R = crate::BitReader<bool>;
#[doc = "Field `MOSI_OE` writer - Output enable for the SPI MOSI signal"]
pub type MOSI_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `MISO_OE` reader - Output enable fo the SPI MISO signal"]
pub type MISO_OE_R = crate::BitReader<bool>;
#[doc = "Field `MISO_OE` writer - Output enable fo the SPI MISO signal"]
pub type MISO_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `WP_OE` reader - Output enable for the SPI Flash write protect signal"]
pub type WP_OE_R = crate::BitReader<bool>;
#[doc = "Field `WP_OE` writer - Output enable for the SPI Flash write protect signal"]
pub type WP_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `HOLD_OE` reader - Output enable for the SPI Flash hold signal"]
pub type HOLD_OE_R = crate::BitReader<bool>;
#[doc = "Field `HOLD_OE` writer - Output enable for the SPI Flash hold signal"]
pub type HOLD_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
#[doc = "Field `DIRECTIOEN` reader - Enable Direct IO 0x0: Disable 0x1: Enable"]
pub type DIRECTIOEN_R = crate::BitReader<bool>;
#[doc = "Field `DIRECTIOEN` writer - Enable Direct IO 0x0: Disable 0x1: Enable"]
pub type DIRECTIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIRECTIO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Status of the SPI CS (chip select) signal"]
    #[inline(always)]
    pub fn cs_i(&self) -> CS_I_R {
        CS_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the SPI SCLK signal"]
    #[inline(always)]
    pub fn sclk_i(&self) -> SCLK_I_R {
        SCLK_I_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the SPI MOSI signal"]
    #[inline(always)]
    pub fn mosi_i(&self) -> MOSI_I_R {
        MOSI_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the SPI MISO signal"]
    #[inline(always)]
    pub fn miso_i(&self) -> MISO_I_R {
        MISO_I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the SPI Flash write protect signal"]
    #[inline(always)]
    pub fn wp_i(&self) -> WP_I_R {
        WP_I_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the SPI Flash hold signal"]
    #[inline(always)]
    pub fn hold_i(&self) -> HOLD_I_R {
        HOLD_I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Output value for the SPI CS (chip select) signal"]
    #[inline(always)]
    pub fn cs_o(&self) -> CS_O_R {
        CS_O_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output value for the SPI SCLK signal"]
    #[inline(always)]
    pub fn sclk_o(&self) -> SCLK_O_R {
        SCLK_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output value for the SPI MOSI signal"]
    #[inline(always)]
    pub fn mosi_o(&self) -> MOSI_O_R {
        MOSI_O_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output value for the SPI MISO signal"]
    #[inline(always)]
    pub fn miso_o(&self) -> MISO_O_R {
        MISO_O_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output value for the SPI Flash write protect signal"]
    #[inline(always)]
    pub fn wp_o(&self) -> WP_O_R {
        WP_O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output value for the SPI Flash hold signal"]
    #[inline(always)]
    pub fn hold_o(&self) -> HOLD_O_R {
        HOLD_O_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Output enable for SPI CS (chip select) signal"]
    #[inline(always)]
    pub fn cs_oe(&self) -> CS_OE_R {
        CS_OE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output enable for the SPI SCLK signal"]
    #[inline(always)]
    pub fn sclk_oe(&self) -> SCLK_OE_R {
        SCLK_OE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output enable for the SPI MOSI signal"]
    #[inline(always)]
    pub fn mosi_oe(&self) -> MOSI_OE_R {
        MOSI_OE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output enable fo the SPI MISO signal"]
    #[inline(always)]
    pub fn miso_oe(&self) -> MISO_OE_R {
        MISO_OE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output enable for the SPI Flash write protect signal"]
    #[inline(always)]
    pub fn wp_oe(&self) -> WP_OE_R {
        WP_OE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output enable for the SPI Flash hold signal"]
    #[inline(always)]
    pub fn hold_oe(&self) -> HOLD_OE_R {
        HOLD_OE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Direct IO 0x0: Disable 0x1: Enable"]
    #[inline(always)]
    pub fn directioen(&self) -> DIRECTIOEN_R {
        DIRECTIOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Output value for the SPI CS (chip select) signal"]
    #[inline(always)]
    #[must_use]
    pub fn cs_o(&mut self) -> CS_O_W<8> {
        CS_O_W::new(self)
    }
    #[doc = "Bit 9 - Output value for the SPI SCLK signal"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_o(&mut self) -> SCLK_O_W<9> {
        SCLK_O_W::new(self)
    }
    #[doc = "Bit 10 - Output value for the SPI MOSI signal"]
    #[inline(always)]
    #[must_use]
    pub fn mosi_o(&mut self) -> MOSI_O_W<10> {
        MOSI_O_W::new(self)
    }
    #[doc = "Bit 11 - Output value for the SPI MISO signal"]
    #[inline(always)]
    #[must_use]
    pub fn miso_o(&mut self) -> MISO_O_W<11> {
        MISO_O_W::new(self)
    }
    #[doc = "Bit 12 - Output value for the SPI Flash write protect signal"]
    #[inline(always)]
    #[must_use]
    pub fn wp_o(&mut self) -> WP_O_W<12> {
        WP_O_W::new(self)
    }
    #[doc = "Bit 13 - Output value for the SPI Flash hold signal"]
    #[inline(always)]
    #[must_use]
    pub fn hold_o(&mut self) -> HOLD_O_W<13> {
        HOLD_O_W::new(self)
    }
    #[doc = "Bit 16 - Output enable for SPI CS (chip select) signal"]
    #[inline(always)]
    #[must_use]
    pub fn cs_oe(&mut self) -> CS_OE_W<16> {
        CS_OE_W::new(self)
    }
    #[doc = "Bit 17 - Output enable for the SPI SCLK signal"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_oe(&mut self) -> SCLK_OE_W<17> {
        SCLK_OE_W::new(self)
    }
    #[doc = "Bit 18 - Output enable for the SPI MOSI signal"]
    #[inline(always)]
    #[must_use]
    pub fn mosi_oe(&mut self) -> MOSI_OE_W<18> {
        MOSI_OE_W::new(self)
    }
    #[doc = "Bit 19 - Output enable fo the SPI MISO signal"]
    #[inline(always)]
    #[must_use]
    pub fn miso_oe(&mut self) -> MISO_OE_W<19> {
        MISO_OE_W::new(self)
    }
    #[doc = "Bit 20 - Output enable for the SPI Flash write protect signal"]
    #[inline(always)]
    #[must_use]
    pub fn wp_oe(&mut self) -> WP_OE_W<20> {
        WP_OE_W::new(self)
    }
    #[doc = "Bit 21 - Output enable for the SPI Flash hold signal"]
    #[inline(always)]
    #[must_use]
    pub fn hold_oe(&mut self) -> HOLD_OE_W<21> {
        HOLD_OE_W::new(self)
    }
    #[doc = "Bit 24 - Enable Direct IO 0x0: Disable 0x1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn directioen(&mut self) -> DIRECTIOEN_W<24> {
        DIRECTIOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct IO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [directio](index.html) module"]
pub struct DIRECTIO_SPEC;
impl crate::RegisterSpec for DIRECTIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [directio::R](R) reader structure"]
impl crate::Readable for DIRECTIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [directio::W](W) writer structure"]
impl crate::Writable for DIRECTIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIRECTIO to value 0x3100"]
impl crate::Resettable for DIRECTIO_SPEC {
    const RESET_VALUE: Self::Ux = 0x3100;
}
