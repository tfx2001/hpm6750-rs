#[doc = "Register `BURSTSIZE` reader"]
pub struct R(crate::R<BURSTSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BURSTSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BURSTSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BURSTSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BURSTSIZE` writer"]
pub struct W(crate::W<BURSTSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BURSTSIZE_SPEC>;
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
impl From<crate::W<BURSTSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BURSTSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPBURST` reader - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
pub type TXPBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXPBURST` writer - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
pub type TXPBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BURSTSIZE_SPEC, u8, u8, 8, O>;
#[doc = "Field `RXPBURST` reader - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
pub type RXPBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPBURST` writer - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
pub type RXPBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BURSTSIZE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
    #[inline(always)]
    pub fn txpburst(&self) -> TXPBURST_R {
        TXPBURST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
    #[inline(always)]
    pub fn rxpburst(&self) -> RXPBURST_R {
        RXPBURST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - TXPBURST Programmable TX Burst Size. Default value is determined by TXBURST bits in n_HWTXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from system memory to the USB bus."]
    #[inline(always)]
    pub fn txpburst(&mut self) -> TXPBURST_W<8> {
        TXPBURST_W::new(self)
    }
    #[doc = "Bits 0:7 - RXPBURST Programmable RX Burst Size. Default value is determined by TXBURST bits in n_HWRXBUF. This register represents the maximum length of a the burst in 32-bit words while moving data from the USB bus to system memory."]
    #[inline(always)]
    pub fn rxpburst(&mut self) -> RXPBURST_W<0> {
        RXPBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Burst Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstsize](index.html) module"]
pub struct BURSTSIZE_SPEC;
impl crate::RegisterSpec for BURSTSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [burstsize::R](R) reader structure"]
impl crate::Readable for BURSTSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [burstsize::W](W) writer structure"]
impl crate::Writable for BURSTSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BURSTSIZE to value 0"]
impl crate::Resettable for BURSTSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
