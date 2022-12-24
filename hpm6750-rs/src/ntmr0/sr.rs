#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0RLDF` reader - channel 1 counter reload flag"]
pub type CH0RLDF_R = crate::BitReader<bool>;
#[doc = "Field `CH0RLDF` writer - channel 1 counter reload flag"]
pub type CH0RLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH0CAPF` reader - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH0CAPF_R = crate::BitReader<bool>;
#[doc = "Field `CH0CAPF` writer - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH0CAPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH0CMP0F` reader - channel 1 compare value 1 match flag"]
pub type CH0CMP0F_R = crate::BitReader<bool>;
#[doc = "Field `CH0CMP0F` writer - channel 1 compare value 1 match flag"]
pub type CH0CMP0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH0CMP1F` reader - channel 1 compare value 1 match flag"]
pub type CH0CMP1F_R = crate::BitReader<bool>;
#[doc = "Field `CH0CMP1F` writer - channel 1 compare value 1 match flag"]
pub type CH0CMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH1RLDF` reader - channel 1 counter reload flag"]
pub type CH1RLDF_R = crate::BitReader<bool>;
#[doc = "Field `CH1RLDF` writer - channel 1 counter reload flag"]
pub type CH1RLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH1CAPF` reader - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH1CAPF_R = crate::BitReader<bool>;
#[doc = "Field `CH1CAPF` writer - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH1CAPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH1CMP0F` reader - channel 1 compare value 1 match flag"]
pub type CH1CMP0F_R = crate::BitReader<bool>;
#[doc = "Field `CH1CMP0F` writer - channel 1 compare value 1 match flag"]
pub type CH1CMP0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH1CMP1F` reader - channel 1 compare value 1 match flag"]
pub type CH1CMP1F_R = crate::BitReader<bool>;
#[doc = "Field `CH1CMP1F` writer - channel 1 compare value 1 match flag"]
pub type CH1CMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH2RLDF` reader - channel 2 counter reload flag"]
pub type CH2RLDF_R = crate::BitReader<bool>;
#[doc = "Field `CH2RLDF` writer - channel 2 counter reload flag"]
pub type CH2RLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH2CAPF` reader - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH2CAPF_R = crate::BitReader<bool>;
#[doc = "Field `CH2CAPF` writer - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH2CAPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH2CMP0F` reader - channel 2 compare value 1 match flag"]
pub type CH2CMP0F_R = crate::BitReader<bool>;
#[doc = "Field `CH2CMP0F` writer - channel 2 compare value 1 match flag"]
pub type CH2CMP0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH2CMP1F` reader - channel 2 compare value 1 match flag"]
pub type CH2CMP1F_R = crate::BitReader<bool>;
#[doc = "Field `CH2CMP1F` writer - channel 2 compare value 1 match flag"]
pub type CH2CMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH3RLDF` reader - channel 3 counter reload flag"]
pub type CH3RLDF_R = crate::BitReader<bool>;
#[doc = "Field `CH3RLDF` writer - channel 3 counter reload flag"]
pub type CH3RLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH3CAPF` reader - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH3CAPF_R = crate::BitReader<bool>;
#[doc = "Field `CH3CAPF` writer - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH3CAPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH3CMP0F` reader - channel 3 compare value 1 match flag"]
pub type CH3CMP0F_R = crate::BitReader<bool>;
#[doc = "Field `CH3CMP0F` writer - channel 3 compare value 1 match flag"]
pub type CH3CMP0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CH3CMP1F` reader - channel 3 compare value 1 match flag"]
pub type CH3CMP1F_R = crate::BitReader<bool>;
#[doc = "Field `CH3CMP1F` writer - channel 3 compare value 1 match flag"]
pub type CH3CMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - channel 1 counter reload flag"]
    #[inline(always)]
    pub fn ch0rldf(&self) -> CH0RLDF_R {
        CH0RLDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    pub fn ch0capf(&self) -> CH0CAPF_R {
        CH0CAPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch0cmp0f(&self) -> CH0CMP0F_R {
        CH0CMP0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch0cmp1f(&self) -> CH0CMP1F_R {
        CH0CMP1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel 1 counter reload flag"]
    #[inline(always)]
    pub fn ch1rldf(&self) -> CH1RLDF_R {
        CH1RLDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    pub fn ch1capf(&self) -> CH1CAPF_R {
        CH1CAPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch1cmp0f(&self) -> CH1CMP0F_R {
        CH1CMP0F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch1cmp1f(&self) -> CH1CMP1F_R {
        CH1CMP1F_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - channel 2 counter reload flag"]
    #[inline(always)]
    pub fn ch2rldf(&self) -> CH2RLDF_R {
        CH2RLDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    pub fn ch2capf(&self) -> CH2CAPF_R {
        CH2CAPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - channel 2 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch2cmp0f(&self) -> CH2CMP0F_R {
        CH2CMP0F_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - channel 2 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch2cmp1f(&self) -> CH2CMP1F_R {
        CH2CMP1F_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - channel 3 counter reload flag"]
    #[inline(always)]
    pub fn ch3rldf(&self) -> CH3RLDF_R {
        CH3RLDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    pub fn ch3capf(&self) -> CH3CAPF_R {
        CH3CAPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - channel 3 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch3cmp0f(&self) -> CH3CMP0F_R {
        CH3CMP0F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - channel 3 compare value 1 match flag"]
    #[inline(always)]
    pub fn ch3cmp1f(&self) -> CH3CMP1F_R {
        CH3CMP1F_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel 1 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rldf(&mut self) -> CH0RLDF_W<0> {
        CH0RLDF_W::new(self)
    }
    #[doc = "Bit 1 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch0capf(&mut self) -> CH0CAPF_W<1> {
        CH0CAPF_W::new(self)
    }
    #[doc = "Bit 2 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cmp0f(&mut self) -> CH0CMP0F_W<2> {
        CH0CMP0F_W::new(self)
    }
    #[doc = "Bit 3 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cmp1f(&mut self) -> CH0CMP1F_W<3> {
        CH0CMP1F_W::new(self)
    }
    #[doc = "Bit 4 - channel 1 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rldf(&mut self) -> CH1RLDF_W<4> {
        CH1RLDF_W::new(self)
    }
    #[doc = "Bit 5 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch1capf(&mut self) -> CH1CAPF_W<5> {
        CH1CAPF_W::new(self)
    }
    #[doc = "Bit 6 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cmp0f(&mut self) -> CH1CMP0F_W<6> {
        CH1CMP0F_W::new(self)
    }
    #[doc = "Bit 7 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cmp1f(&mut self) -> CH1CMP1F_W<7> {
        CH1CMP1F_W::new(self)
    }
    #[doc = "Bit 8 - channel 2 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2rldf(&mut self) -> CH2RLDF_W<8> {
        CH2RLDF_W::new(self)
    }
    #[doc = "Bit 9 - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch2capf(&mut self) -> CH2CAPF_W<9> {
        CH2CAPF_W::new(self)
    }
    #[doc = "Bit 10 - channel 2 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cmp0f(&mut self) -> CH2CMP0F_W<10> {
        CH2CMP0F_W::new(self)
    }
    #[doc = "Bit 11 - channel 2 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cmp1f(&mut self) -> CH2CMP1F_W<11> {
        CH2CMP1F_W::new(self)
    }
    #[doc = "Bit 12 - channel 3 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3rldf(&mut self) -> CH3RLDF_W<12> {
        CH3RLDF_W::new(self)
    }
    #[doc = "Bit 13 - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch3capf(&mut self) -> CH3CAPF_W<13> {
        CH3CAPF_W::new(self)
    }
    #[doc = "Bit 14 - channel 3 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cmp0f(&mut self) -> CH3CMP0F_W<14> {
        CH3CMP0F_W::new(self)
    }
    #[doc = "Bit 15 - channel 3 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cmp1f(&mut self) -> CH3CMP1F_W<15> {
        CH3CMP1F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
