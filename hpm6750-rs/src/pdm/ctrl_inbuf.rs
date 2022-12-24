#[doc = "Register `CTRL_INBUF` reader"]
pub struct R(crate::R<CTRL_INBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_INBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_INBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_INBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_INBUF` writer"]
pub struct W(crate::W<CTRL_INBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_INBUF_SPEC>;
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
impl From<crate::W<CTRL_INBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_INBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_ADDR` reader - The starting address of channel 0 in filter data buffer"]
pub type START_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `START_ADDR` writer - The starting address of channel 0 in filter data buffer"]
pub type START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_INBUF_SPEC, u16, u16, 11, O>;
#[doc = "Field `PITCH` reader - The spacing between starting address of adjacent channels"]
pub type PITCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PITCH` writer - The spacing between starting address of adjacent channels"]
pub type PITCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_INBUF_SPEC, u16, u16, 11, O>;
#[doc = "Field `MAX_PTR` reader - The buf size-1 for each channel"]
pub type MAX_PTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_PTR` writer - The buf size-1 for each channel"]
pub type MAX_PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_INBUF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:10 - The starting address of channel 0 in filter data buffer"]
    #[inline(always)]
    pub fn start_addr(&self) -> START_ADDR_R {
        START_ADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - The spacing between starting address of adjacent channels"]
    #[inline(always)]
    pub fn pitch(&self) -> PITCH_R {
        PITCH_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:29 - The buf size-1 for each channel"]
    #[inline(always)]
    pub fn max_ptr(&self) -> MAX_PTR_R {
        MAX_PTR_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - The starting address of channel 0 in filter data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn start_addr(&mut self) -> START_ADDR_W<0> {
        START_ADDR_W::new(self)
    }
    #[doc = "Bits 11:21 - The spacing between starting address of adjacent channels"]
    #[inline(always)]
    #[must_use]
    pub fn pitch(&mut self) -> PITCH_W<11> {
        PITCH_W::new(self)
    }
    #[doc = "Bits 22:29 - The buf size-1 for each channel"]
    #[inline(always)]
    #[must_use]
    pub fn max_ptr(&mut self) -> MAX_PTR_W<22> {
        MAX_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In Buf Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_inbuf](index.html) module"]
pub struct CTRL_INBUF_SPEC;
impl crate::RegisterSpec for CTRL_INBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_inbuf::R](R) reader structure"]
impl crate::Readable for CTRL_INBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_inbuf::W](W) writer structure"]
impl crate::Writable for CTRL_INBUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_INBUF to value 0"]
impl crate::Resettable for CTRL_INBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
