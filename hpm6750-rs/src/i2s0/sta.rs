#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DA` reader - Asserted when rx fifo data are available."]
pub type RX_DA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_DN` reader - Asserted when tx fifo data are needed."]
pub type TX_DN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_OV` reader - Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
pub type RX_OV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_OV` writer - Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
pub type RX_OV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STA_SPEC, u8, u8, 4, O>;
#[doc = "Field `TX_UD` reader - Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
pub type TX_UD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_UD` writer - Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
pub type TX_UD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 1:4 - Asserted when rx fifo data are available."]
    #[inline(always)]
    pub fn rx_da(&self) -> RX_DA_R {
        RX_DA_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Asserted when tx fifo data are needed."]
    #[inline(always)]
    pub fn tx_dn(&self) -> TX_DN_R {
        TX_DN_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
    #[inline(always)]
    pub fn rx_ov(&self) -> RX_OV_R {
        RX_OV_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
    #[inline(always)]
    pub fn tx_ud(&self) -> TX_UD_R {
        TX_UD_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:12 - Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ov(&mut self) -> RX_OV_W<9> {
        RX_OV_W::new(self)
    }
    #[doc = "Bits 13:16 - Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\]
the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ud(&mut self) -> TX_UD_W<13> {
        TX_UD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
