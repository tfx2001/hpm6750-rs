#[doc = "Register `ENDPTFLUSH` reader"]
pub struct R(crate::R<ENDPTFLUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTFLUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTFLUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTFLUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTFLUSH` writer"]
pub struct W(crate::W<ENDPTFLUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTFLUSH_SPEC>;
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
impl From<crate::W<ENDPTFLUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTFLUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETB` reader - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FETB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FETB` writer - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FETB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTFLUSH_SPEC, u8, u8, 8, O>;
#[doc = "Field `FERB` reader - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FERB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FERB` writer - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FERB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENDPTFLUSH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn fetb(&self) -> FETB_R {
        FETB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn ferb(&self) -> FERB_R {
        FERB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn fetb(&mut self) -> FETB_W<16> {
        FETB_W::new(self)
    }
    #[doc = "Bits 0:7 - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn ferb(&mut self) -> FERB_W<0> {
        FERB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Flush Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptflush](index.html) module"]
pub struct ENDPTFLUSH_SPEC;
impl crate::RegisterSpec for ENDPTFLUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptflush::R](R) reader structure"]
impl crate::Readable for ENDPTFLUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptflush::W](W) writer structure"]
impl crate::Writable for ENDPTFLUSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDPTFLUSH to value 0"]
impl crate::Resettable for ENDPTFLUSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
