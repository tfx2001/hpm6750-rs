#[doc = "Register `ENDPTCOMPLETE` reader"]
pub struct R(crate::R<ENDPTCOMPLETE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTCOMPLETE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTCOMPLETE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTCOMPLETE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTCOMPLETE` writer"]
pub struct W(crate::W<ENDPTCOMPLETE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTCOMPLETE_SPEC>;
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
impl From<crate::W<ENDPTCOMPLETE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTCOMPLETE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETCE` reader - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ETCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETCE` writer - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ETCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTCOMPLETE_SPEC, u8, u8, 8, O>;
#[doc = "Field `ERCE` reader - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ERCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERCE` writer - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ERCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTCOMPLETE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn etce(&self) -> ETCE_R {
        ETCE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn erce(&self) -> ERCE_R {
        ERCE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - ETCE Endpoint Transmit Complete Event - R/WC. Each bit indicates a transmit event (IN/INTERRUPT) occurred and software should read the corresponding endpoint queue to determine the endpoint status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ETCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn etce(&mut self) -> ETCE_W<16> {
        ETCE_W::new(self)
    }
    #[doc = "Bits 0:7 - ERCE Endpoint Receive Complete Event - RW/C. Each bit indicates a received event (OUT/SETUP) occurred and software should read the corresponding endpoint queue to determine the transfer status. If the corresponding IOC bit is set in the Transfer Descriptor, then this bit is set simultaneously with the USBINT . Writing one clears the corresponding bit in this register. ERCE\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn erce(&mut self) -> ERCE_W<0> {
        ERCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptcomplete](index.html) module"]
pub struct ENDPTCOMPLETE_SPEC;
impl crate::RegisterSpec for ENDPTCOMPLETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptcomplete::R](R) reader structure"]
impl crate::Readable for ENDPTCOMPLETE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptcomplete::W](W) writer structure"]
impl crate::Writable for ENDPTCOMPLETE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDPTCOMPLETE to value 0"]
impl crate::Resettable for ENDPTCOMPLETE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
