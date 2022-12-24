#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLS` reader - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits"]
pub type WLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WLS` writer - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits"]
pub type WLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STB` reader - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits"]
pub type STB_R = crate::BitReader<bool>;
#[doc = "Field `STB` writer - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits"]
pub type STB_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `PEN` reader - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
pub type PEN_R = crate::BitReader<bool>;
#[doc = "Field `PEN` writer - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `EPS` reader - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
pub type EPS_R = crate::BitReader<bool>;
#[doc = "Field `EPS` writer - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
pub type EPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `SPS` reader - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
pub type SPS_R = crate::BitReader<bool>;
#[doc = "Field `SPS` writer - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
pub type SPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `BC` reader - Break control"]
pub type BC_R = crate::BitReader<bool>;
#[doc = "Field `BC` writer - Break control"]
pub type BC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `DLAB` reader - Divisor latch access bit"]
pub type DLAB_R = crate::BitReader<bool>;
#[doc = "Field `DLAB` writer - Divisor latch access bit"]
pub type DLAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits"]
    #[inline(always)]
    pub fn stb(&self) -> STB_R {
        STB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break control"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor latch access bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits"]
    #[inline(always)]
    #[must_use]
    pub fn wls(&mut self) -> WLS_W<0> {
        WLS_W::new(self)
    }
    #[doc = "Bit 2 - Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> STB_W<2> {
        STB_W::new(self)
    }
    #[doc = "Bit 3 - Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<3> {
        PEN_W::new(self)
    }
    #[doc = "Bit 4 - Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EPS_W<4> {
        EPS_W::new(self)
    }
    #[doc = "Bit 5 - Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SPS_W<5> {
        SPS_W::new(self)
    }
    #[doc = "Bit 6 - Break control"]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BC_W<6> {
        BC_W::new(self)
    }
    #[doc = "Bit 7 - Divisor latch access bit"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DLAB_W<7> {
        DLAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
