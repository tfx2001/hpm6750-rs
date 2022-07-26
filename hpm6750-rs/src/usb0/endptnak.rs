#[doc = "Register `ENDPTNAK` reader"]
pub struct R(crate::R<ENDPTNAK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTNAK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTNAK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTNAK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTNAK` writer"]
pub struct W(crate::W<ENDPTNAK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTNAK_SPEC>;
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
impl From<crate::W<ENDPTNAK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTNAK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPTN` reader - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTN` writer - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTNAK_SPEC, u8, u8, 8, O>;
#[doc = "Field `EPRN` reader - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPRN` writer - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTNAK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eptn(&self) -> EPTN_R {
        EPTN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eprn(&self) -> EPRN_R {
        EPRN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - EPTN TX Endpoint NAK - R/WC. Each TX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received IN token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eptn(&mut self) -> EPTN_W<16> {
        EPTN_W::new(self)
    }
    #[doc = "Bits 0:7 - EPRN RX Endpoint NAK - R/WC. Each RX endpoint has 1 bit in this field. The bit is set when the device sends a NAK handshake on a received OUT or PING token for the corresponding endpoint. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eprn(&mut self) -> EPRN_W<0> {
        EPRN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint NAK Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptnak](index.html) module"]
pub struct ENDPTNAK_SPEC;
impl crate::RegisterSpec for ENDPTNAK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptnak::R](R) reader structure"]
impl crate::Readable for ENDPTNAK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptnak::W](W) writer structure"]
impl crate::Writable for ENDPTNAK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDPTNAK to value 0"]
impl crate::Resettable for ENDPTNAK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
