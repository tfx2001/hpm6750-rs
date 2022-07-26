#[doc = "Register `ENDPTNAKEN` reader"]
pub struct R(crate::R<ENDPTNAKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTNAKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTNAKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTNAKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTNAKEN` writer"]
pub struct W(crate::W<ENDPTNAKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTNAKEN_SPEC>;
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
impl From<crate::W<ENDPTNAKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTNAKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPTNE` reader - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTNE` writer - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPTNE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTNAKEN_SPEC, u8, u8, 8, O>;
#[doc = "Field `EPRNE` reader - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPRNE` writer - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
pub type EPRNE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTNAKEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eptne(&self) -> EPTNE_R {
        EPTNE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eprne(&self) -> EPRNE_R {
        EPRNE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - EPTNE TX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding TX Endpoint NAK bit. If this bit is set and the corresponding TX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eptne(&mut self) -> EPTNE_W<16> {
        EPTNE_W::new(self)
    }
    #[doc = "Bits 0:7 - EPRNE RX Endpoint NAK Enable - R/W. Each bit is an enable bit for the corresponding RX Endpoint NAK bit. If this bit is set and the corresponding RX Endpoint NAK bit is set, the NAK Interrupt bit is set. Bit \\[N\\]
- Endpoint #\\[N\\], N is 0-7"]
    #[inline(always)]
    pub fn eprne(&mut self) -> EPRNE_W<0> {
        EPRNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint NAK Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptnaken](index.html) module"]
pub struct ENDPTNAKEN_SPEC;
impl crate::RegisterSpec for ENDPTNAKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptnaken::R](R) reader structure"]
impl crate::Readable for ENDPTNAKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptnaken::W](W) writer structure"]
impl crate::Writable for ENDPTNAKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDPTNAKEN to value 0"]
impl crate::Resettable for ENDPTNAKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
