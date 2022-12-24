#[doc = "Register `FIFO_THRESH` reader"]
pub struct R(crate::R<FIFO_THRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_THRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_THRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_THRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_THRESH` writer"]
pub struct W(crate::W<FIFO_THRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_THRESH_SPEC>;
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
impl From<crate::W<FIFO_THRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_THRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX` reader - RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
pub type RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX` writer - RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
pub type RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_THRESH_SPEC, u8, u8, 8, O>;
#[doc = "Field `TX` reader - TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
pub type TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX` writer - TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_THRESH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<0> {
        RX_W::new(self)
    }
    #[doc = "Bits 8:15 - TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<8> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX/RX FIFO Threshold setting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_thresh](index.html) module"]
pub struct FIFO_THRESH_SPEC;
impl crate::RegisterSpec for FIFO_THRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_thresh::R](R) reader structure"]
impl crate::Readable for FIFO_THRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_thresh::W](W) writer structure"]
impl crate::Writable for FIFO_THRESH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFO_THRESH to value 0"]
impl crate::Resettable for FIFO_THRESH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
