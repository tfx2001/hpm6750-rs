#[doc = "Register `BUFADDR` reader"]
pub struct R(crate::R<BUFADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFADDR` writer"]
pub struct W(crate::W<BUFADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFADDR_SPEC>;
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
impl From<crate::W<BUFADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR\\[31:28\\]
denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\]
is the address inside the buffer"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - ADDR\\[31:28\\]
denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\]
is the address inside the buffer"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ADDR\\[31:28\\]
denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\]
is the address inside the buffer"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDR\\[31:28\\]
denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\]
is the address inside the buffer"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buf Access Addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufaddr](index.html) module"]
pub struct BUFADDR_SPEC;
impl crate::RegisterSpec for BUFADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufaddr::R](R) reader structure"]
impl crate::Readable for BUFADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufaddr::W](W) writer structure"]
impl crate::Writable for BUFADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFADDR to value 0"]
impl crate::Resettable for BUFADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
