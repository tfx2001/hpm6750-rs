#[doc = "Register `CH_CFG` reader"]
pub struct R(crate::R<CH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_CFG` writer"]
pub struct W(crate::W<CH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CFG_SPEC>;
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
impl From<crate::W<CH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH9_TYPE` reader - No description avaiable"]
pub type CH9_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH9_TYPE` writer - No description avaiable"]
pub type CH9_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH8_TYPE` reader - No description avaiable"]
pub type CH8_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH8_TYPE` writer - No description avaiable"]
pub type CH8_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH7_TYPE` reader - No description avaiable"]
pub type CH7_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH7_TYPE` writer - No description avaiable"]
pub type CH7_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH6_TYPE` reader - No description avaiable"]
pub type CH6_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH6_TYPE` writer - No description avaiable"]
pub type CH6_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH5_TYPE` reader - No description avaiable"]
pub type CH5_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH5_TYPE` writer - No description avaiable"]
pub type CH5_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH4_TYPE` reader - No description avaiable"]
pub type CH4_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH4_TYPE` writer - No description avaiable"]
pub type CH4_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH3_TYPE` reader - No description avaiable"]
pub type CH3_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3_TYPE` writer - No description avaiable"]
pub type CH3_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH2_TYPE` reader - No description avaiable"]
pub type CH2_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2_TYPE` writer - No description avaiable"]
pub type CH2_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH1_TYPE` reader - No description avaiable"]
pub type CH1_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1_TYPE` writer - No description avaiable"]
pub type CH1_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH0_TYPE` reader - Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)"]
pub type CH0_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0_TYPE` writer - Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)"]
pub type CH0_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 18:19 - No description avaiable"]
    #[inline(always)]
    pub fn ch9_type(&self) -> CH9_TYPE_R {
        CH9_TYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - No description avaiable"]
    #[inline(always)]
    pub fn ch8_type(&self) -> CH8_TYPE_R {
        CH8_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - No description avaiable"]
    #[inline(always)]
    pub fn ch7_type(&self) -> CH7_TYPE_R {
        CH7_TYPE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - No description avaiable"]
    #[inline(always)]
    pub fn ch6_type(&self) -> CH6_TYPE_R {
        CH6_TYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - No description avaiable"]
    #[inline(always)]
    pub fn ch5_type(&self) -> CH5_TYPE_R {
        CH5_TYPE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - No description avaiable"]
    #[inline(always)]
    pub fn ch4_type(&self) -> CH4_TYPE_R {
        CH4_TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - No description avaiable"]
    #[inline(always)]
    pub fn ch3_type(&self) -> CH3_TYPE_R {
        CH3_TYPE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - No description avaiable"]
    #[inline(always)]
    pub fn ch2_type(&self) -> CH2_TYPE_R {
        CH2_TYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - No description avaiable"]
    #[inline(always)]
    pub fn ch1_type(&self) -> CH1_TYPE_R {
        CH1_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)"]
    #[inline(always)]
    pub fn ch0_type(&self) -> CH0_TYPE_R {
        CH0_TYPE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - No description avaiable"]
    #[inline(always)]
    pub fn ch9_type(&mut self) -> CH9_TYPE_W<18> {
        CH9_TYPE_W::new(self)
    }
    #[doc = "Bits 16:17 - No description avaiable"]
    #[inline(always)]
    pub fn ch8_type(&mut self) -> CH8_TYPE_W<16> {
        CH8_TYPE_W::new(self)
    }
    #[doc = "Bits 14:15 - No description avaiable"]
    #[inline(always)]
    pub fn ch7_type(&mut self) -> CH7_TYPE_W<14> {
        CH7_TYPE_W::new(self)
    }
    #[doc = "Bits 12:13 - No description avaiable"]
    #[inline(always)]
    pub fn ch6_type(&mut self) -> CH6_TYPE_W<12> {
        CH6_TYPE_W::new(self)
    }
    #[doc = "Bits 10:11 - No description avaiable"]
    #[inline(always)]
    pub fn ch5_type(&mut self) -> CH5_TYPE_W<10> {
        CH5_TYPE_W::new(self)
    }
    #[doc = "Bits 8:9 - No description avaiable"]
    #[inline(always)]
    pub fn ch4_type(&mut self) -> CH4_TYPE_W<8> {
        CH4_TYPE_W::new(self)
    }
    #[doc = "Bits 6:7 - No description avaiable"]
    #[inline(always)]
    pub fn ch3_type(&mut self) -> CH3_TYPE_W<6> {
        CH3_TYPE_W::new(self)
    }
    #[doc = "Bits 4:5 - No description avaiable"]
    #[inline(always)]
    pub fn ch2_type(&mut self) -> CH2_TYPE_W<4> {
        CH2_TYPE_W::new(self)
    }
    #[doc = "Bits 2:3 - No description avaiable"]
    #[inline(always)]
    pub fn ch1_type(&mut self) -> CH1_TYPE_W<2> {
        CH1_TYPE_W::new(self)
    }
    #[doc = "Bits 0:1 - Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)"]
    #[inline(always)]
    pub fn ch0_type(&mut self) -> CH0_TYPE_W<0> {
        CH0_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_cfg](index.html) module"]
pub struct CH_CFG_SPEC;
impl crate::RegisterSpec for CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_cfg::R](R) reader structure"]
impl crate::Readable for CH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_cfg::W](W) writer structure"]
impl crate::Writable for CH_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_CFG to value 0"]
impl crate::Resettable for CH_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
