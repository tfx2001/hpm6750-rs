#[doc = "Register `GROUP0_2_VALUE` reader"]
pub struct R(crate::R<GROUP0_2_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GROUP0_2_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GROUP0_2_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GROUP0_2_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GROUP0_2_VALUE` writer"]
pub struct W(crate::W<GROUP0_2_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP0_2_VALUE_SPEC>;
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
impl From<crate::W<GROUP0_2_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP0_2_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC1` reader - "]
pub type ADC1_R = crate::BitReader<ADC1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_A {
    #[doc = "0: Unlink from resource group 0"]
    UNLINK = 0,
    #[doc = "1: Link to resource group 0"]
    LINK = 1,
}
impl From<ADC1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_A {
        match self.bits {
            false => ADC1_A::UNLINK,
            true => ADC1_A::LINK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLINK`"]
    #[inline(always)]
    pub fn is_unlink(&self) -> bool {
        *self == ADC1_A::UNLINK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == ADC1_A::LINK
    }
}
#[doc = "Field `ADC1` writer - "]
pub type ADC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GROUP0_2_VALUE_SPEC, ADC1_A, O>;
impl<'a, const O: u8> ADC1_W<'a, O> {
    #[doc = "Unlink from resource group 0"]
    #[inline(always)]
    pub fn unlink(self) -> &'a mut W {
        self.variant(ADC1_A::UNLINK)
    }
    #[doc = "Link to resource group 0"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(ADC1_A::LINK)
    }
}
#[doc = "Field `ADC2` reader - "]
pub use ADC1_R as ADC2_R;
#[doc = "Field `ADC3` reader - "]
pub use ADC1_R as ADC3_R;
#[doc = "Field `ACMP` reader - "]
pub use ADC1_R as ACMP_R;
#[doc = "Field `I2S0` reader - "]
pub use ADC1_R as I2S0_R;
#[doc = "Field `I2S1` reader - "]
pub use ADC1_R as I2S1_R;
#[doc = "Field `I2S2` reader - "]
pub use ADC1_R as I2S2_R;
#[doc = "Field `I2S3` reader - "]
pub use ADC1_R as I2S3_R;
#[doc = "Field `PDM` reader - "]
pub use ADC1_R as PDM_R;
#[doc = "Field `DAO` reader - "]
pub use ADC1_R as DAO_R;
#[doc = "Field `SYNT` reader - "]
pub use ADC1_R as SYNT_R;
#[doc = "Field `MOT0` reader - "]
pub use ADC1_R as MOT0_R;
#[doc = "Field `MOT1` reader - "]
pub use ADC1_R as MOT1_R;
#[doc = "Field `MOT2` reader - "]
pub use ADC1_R as MOT2_R;
#[doc = "Field `MOT3` reader - "]
pub use ADC1_R as MOT3_R;
#[doc = "Field `LCDC` reader - "]
pub use ADC1_R as LCDC_R;
#[doc = "Field `CAM0` reader - "]
pub use ADC1_R as CAM0_R;
#[doc = "Field `CAM1` reader - "]
pub use ADC1_R as CAM1_R;
#[doc = "Field `JPEG` reader - "]
pub use ADC1_R as JPEG_R;
#[doc = "Field `PDMA` reader - "]
pub use ADC1_R as PDMA_R;
#[doc = "Field `ENET0` reader - "]
pub use ADC1_R as ENET0_R;
#[doc = "Field `ENET1` reader - "]
pub use ADC1_R as ENET1_R;
#[doc = "Field `NTMR0` reader - "]
pub use ADC1_R as NTMR0_R;
#[doc = "Field `NTMR1` reader - "]
pub use ADC1_R as NTMR1_R;
#[doc = "Field `SDXC0` reader - "]
pub use ADC1_R as SDXC0_R;
#[doc = "Field `SDXC1` reader - "]
pub use ADC1_R as SDXC1_R;
#[doc = "Field `USB0` reader - "]
pub use ADC1_R as USB0_R;
#[doc = "Field `USB1` reader - "]
pub use ADC1_R as USB1_R;
#[doc = "Field `ADC2` writer - "]
pub use ADC1_W as ADC2_W;
#[doc = "Field `ADC3` writer - "]
pub use ADC1_W as ADC3_W;
#[doc = "Field `ACMP` writer - "]
pub use ADC1_W as ACMP_W;
#[doc = "Field `I2S0` writer - "]
pub use ADC1_W as I2S0_W;
#[doc = "Field `I2S1` writer - "]
pub use ADC1_W as I2S1_W;
#[doc = "Field `I2S2` writer - "]
pub use ADC1_W as I2S2_W;
#[doc = "Field `I2S3` writer - "]
pub use ADC1_W as I2S3_W;
#[doc = "Field `PDM` writer - "]
pub use ADC1_W as PDM_W;
#[doc = "Field `DAO` writer - "]
pub use ADC1_W as DAO_W;
#[doc = "Field `SYNT` writer - "]
pub use ADC1_W as SYNT_W;
#[doc = "Field `MOT0` writer - "]
pub use ADC1_W as MOT0_W;
#[doc = "Field `MOT1` writer - "]
pub use ADC1_W as MOT1_W;
#[doc = "Field `MOT2` writer - "]
pub use ADC1_W as MOT2_W;
#[doc = "Field `MOT3` writer - "]
pub use ADC1_W as MOT3_W;
#[doc = "Field `LCDC` writer - "]
pub use ADC1_W as LCDC_W;
#[doc = "Field `CAM0` writer - "]
pub use ADC1_W as CAM0_W;
#[doc = "Field `CAM1` writer - "]
pub use ADC1_W as CAM1_W;
#[doc = "Field `JPEG` writer - "]
pub use ADC1_W as JPEG_W;
#[doc = "Field `PDMA` writer - "]
pub use ADC1_W as PDMA_W;
#[doc = "Field `ENET0` writer - "]
pub use ADC1_W as ENET0_W;
#[doc = "Field `ENET1` writer - "]
pub use ADC1_W as ENET1_W;
#[doc = "Field `NTMR0` writer - "]
pub use ADC1_W as NTMR0_W;
#[doc = "Field `NTMR1` writer - "]
pub use ADC1_W as NTMR1_W;
#[doc = "Field `SDXC0` writer - "]
pub use ADC1_W as SDXC0_W;
#[doc = "Field `SDXC1` writer - "]
pub use ADC1_W as SDXC1_W;
#[doc = "Field `USB0` writer - "]
pub use ADC1_W as USB0_W;
#[doc = "Field `USB1` writer - "]
pub use ADC1_W as USB1_W;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2(&self) -> ADC2_R {
        ADC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc3(&self) -> ADC3_R {
        ADC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0(&self) -> I2S0_R {
        I2S0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s1(&self) -> I2S1_R {
        I2S1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s2(&self) -> I2S2_R {
        I2S2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s3(&self) -> I2S3_R {
        I2S3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pdm(&self) -> PDM_R {
        PDM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dao(&self) -> DAO_R {
        DAO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn synt(&self) -> SYNT_R {
        SYNT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mot0(&self) -> MOT0_R {
        MOT0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mot1(&self) -> MOT1_R {
        MOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mot2(&self) -> MOT2_R {
        MOT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn mot3(&self) -> MOT3_R {
        MOT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lcdc(&self) -> LCDC_R {
        LCDC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cam0(&self) -> CAM0_R {
        CAM0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cam1(&self) -> CAM1_R {
        CAM1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pdma(&self) -> PDMA_R {
        PDMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn enet0(&self) -> ENET0_R {
        ENET0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn enet1(&self) -> ENET1_R {
        ENET1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ntmr0(&self) -> NTMR0_R {
        NTMR0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ntmr1(&self) -> NTMR1_R {
        NTMR1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sdxc0(&self) -> SDXC0_R {
        SDXC0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sdxc1(&self) -> SDXC1_R {
        SDXC1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usb1(&self) -> USB1_R {
        USB1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<0> {
        ADC1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc2(&mut self) -> ADC2_W<1> {
        ADC2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn adc3(&mut self) -> ADC3_W<2> {
        ADC3_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn acmp(&mut self) -> ACMP_W<3> {
        ACMP_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0(&mut self) -> I2S0_W<4> {
        I2S0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1(&mut self) -> I2S1_W<5> {
        I2S1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2(&mut self) -> I2S2_W<6> {
        I2S2_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn i2s3(&mut self) -> I2S3_W<7> {
        I2S3_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pdm(&mut self) -> PDM_W<8> {
        PDM_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dao(&mut self) -> DAO_W<9> {
        DAO_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn synt(&mut self) -> SYNT_W<10> {
        SYNT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn mot0(&mut self) -> MOT0_W<11> {
        MOT0_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn mot1(&mut self) -> MOT1_W<12> {
        MOT1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn mot2(&mut self) -> MOT2_W<13> {
        MOT2_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn mot3(&mut self) -> MOT3_W<14> {
        MOT3_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn lcdc(&mut self) -> LCDC_W<15> {
        LCDC_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cam0(&mut self) -> CAM0_W<16> {
        CAM0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cam1(&mut self) -> CAM1_W<17> {
        CAM1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<18> {
        JPEG_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pdma(&mut self) -> PDMA_W<19> {
        PDMA_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn enet0(&mut self) -> ENET0_W<20> {
        ENET0_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn enet1(&mut self) -> ENET1_W<21> {
        ENET1_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ntmr0(&mut self) -> NTMR0_W<22> {
        NTMR0_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ntmr1(&mut self) -> NTMR1_W<23> {
        NTMR1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0(&mut self) -> SDXC0_W<24> {
        SDXC0_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc1(&mut self) -> SDXC1_W<25> {
        SDXC1_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn usb0(&mut self) -> USB0_W<26> {
        USB0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn usb1(&mut self) -> USB1_W<27> {
        USB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Goup setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group0_2_value](index.html) module"]
pub struct GROUP0_2_VALUE_SPEC;
impl crate::RegisterSpec for GROUP0_2_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [group0_2_value::R](R) reader structure"]
impl crate::Readable for GROUP0_2_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [group0_2_value::W](W) writer structure"]
impl crate::Writable for GROUP0_2_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GROUP0_2_VALUE to value 0"]
impl crate::Resettable for GROUP0_2_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
