#[doc = "Register `INT_EN` reader"]
pub struct R(crate::R<INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN` writer"]
pub struct W(crate::W<INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SPEC>;
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
impl From<crate::W<INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP_INT_STS1` reader - No description avaiable"]
pub type COMP_INT_STS1_R = crate::BitReader<bool>;
#[doc = "Field `COMP_INT_STS1` writer - No description avaiable"]
pub type COMP_INT_STS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `CAPTURE_INT_STS1` reader - No description avaiable"]
pub type CAPTURE_INT_STS1_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE_INT_STS1` writer - No description avaiable"]
pub type CAPTURE_INT_STS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `PPS_INT_STS1` reader - No description avaiable"]
pub type PPS_INT_STS1_R = crate::BitReader<bool>;
#[doc = "Field `PPS_INT_STS1` writer - No description avaiable"]
pub type PPS_INT_STS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `COMP_INT_STS0` reader - No description avaiable"]
pub type COMP_INT_STS0_R = crate::BitReader<bool>;
#[doc = "Field `COMP_INT_STS0` writer - No description avaiable"]
pub type COMP_INT_STS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `CAPTURE_INT_STS0` reader - No description avaiable"]
pub type CAPTURE_INT_STS0_R = crate::BitReader<bool>;
#[doc = "Field `CAPTURE_INT_STS0` writer - No description avaiable"]
pub type CAPTURE_INT_STS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `PPS_INT_STS0` reader - No description avaiable"]
pub type PPS_INT_STS0_R = crate::BitReader<bool>;
#[doc = "Field `PPS_INT_STS0` writer - No description avaiable"]
pub type PPS_INT_STS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 18 - No description avaiable"]
    #[inline(always)]
    pub fn comp_int_sts1(&self) -> COMP_INT_STS1_R {
        COMP_INT_STS1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn capture_int_sts1(&self) -> CAPTURE_INT_STS1_R {
        CAPTURE_INT_STS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    pub fn pps_int_sts1(&self) -> PPS_INT_STS1_R {
        PPS_INT_STS1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn comp_int_sts0(&self) -> COMP_INT_STS0_R {
        COMP_INT_STS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn capture_int_sts0(&self) -> CAPTURE_INT_STS0_R {
        CAPTURE_INT_STS0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn pps_int_sts0(&self) -> PPS_INT_STS0_R {
        PPS_INT_STS0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - No description avaiable"]
    #[inline(always)]
    pub fn comp_int_sts1(&mut self) -> COMP_INT_STS1_W<18> {
        COMP_INT_STS1_W::new(self)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn capture_int_sts1(&mut self) -> CAPTURE_INT_STS1_W<17> {
        CAPTURE_INT_STS1_W::new(self)
    }
    #[doc = "Bit 16 - No description avaiable"]
    #[inline(always)]
    pub fn pps_int_sts1(&mut self) -> PPS_INT_STS1_W<16> {
        PPS_INT_STS1_W::new(self)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn comp_int_sts0(&mut self) -> COMP_INT_STS0_W<2> {
        COMP_INT_STS0_W::new(self)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn capture_int_sts0(&mut self) -> CAPTURE_INT_STS0_W<1> {
        CAPTURE_INT_STS0_W::new(self)
    }
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn pps_int_sts0(&mut self) -> PPS_INT_STS0_W<0> {
        PPS_INT_STS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](index.html) module"]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en::R](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en::W](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
