#[doc = "Register `GROUP0_1_VALUE` reader"]
pub struct R(crate::R<GROUP0_1_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GROUP0_1_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GROUP0_1_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GROUP0_1_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GROUP0_1_VALUE` writer"]
pub struct W(crate::W<GROUP0_1_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP0_1_VALUE_SPEC>;
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
impl From<crate::W<GROUP0_1_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP0_1_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTMR6` reader - "]
pub type GPTMR6_R = crate::BitReader<GPTMR6_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTMR6_A {
    #[doc = "0: Unlink from resource group 0"]
    UNLINK = 0,
    #[doc = "1: Link to resource group 0"]
    LINK = 1,
}
impl From<GPTMR6_A> for bool {
    #[inline(always)]
    fn from(variant: GPTMR6_A) -> Self {
        variant as u8 != 0
    }
}
impl GPTMR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTMR6_A {
        match self.bits {
            false => GPTMR6_A::UNLINK,
            true => GPTMR6_A::LINK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLINK`"]
    #[inline(always)]
    pub fn is_unlink(&self) -> bool {
        *self == GPTMR6_A::UNLINK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == GPTMR6_A::LINK
    }
}
#[doc = "Field `GPTMR6` writer - "]
pub type GPTMR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GROUP0_1_VALUE_SPEC, GPTMR6_A, O>;
impl<'a, const O: u8> GPTMR6_W<'a, O> {
    #[doc = "Unlink from resource group 0"]
    #[inline(always)]
    pub fn unlink(self) -> &'a mut W {
        self.variant(GPTMR6_A::UNLINK)
    }
    #[doc = "Link to resource group 0"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(GPTMR6_A::LINK)
    }
}
#[doc = "Field `GPTMR7` reader - "]
pub use GPTMR6_R as GPTMR7_R;
#[doc = "Field `UART0` reader - "]
pub use GPTMR6_R as UART0_R;
#[doc = "Field `UART1` reader - "]
pub use GPTMR6_R as UART1_R;
#[doc = "Field `UART2` reader - "]
pub use GPTMR6_R as UART2_R;
#[doc = "Field `UART3` reader - "]
pub use GPTMR6_R as UART3_R;
#[doc = "Field `UART4` reader - "]
pub use GPTMR6_R as UART4_R;
#[doc = "Field `UART5` reader - "]
pub use GPTMR6_R as UART5_R;
#[doc = "Field `UART6` reader - "]
pub use GPTMR6_R as UART6_R;
#[doc = "Field `UART7` reader - "]
pub use GPTMR6_R as UART7_R;
#[doc = "Field `UART8` reader - "]
pub use GPTMR6_R as UART8_R;
#[doc = "Field `UART9` reader - "]
pub use GPTMR6_R as UART9_R;
#[doc = "Field `UART10` reader - "]
pub use GPTMR6_R as UART10_R;
#[doc = "Field `UART11` reader - "]
pub use GPTMR6_R as UART11_R;
#[doc = "Field `UART12` reader - "]
pub use GPTMR6_R as UART12_R;
#[doc = "Field `UART13` reader - "]
pub use GPTMR6_R as UART13_R;
#[doc = "Field `UART14` reader - "]
pub use GPTMR6_R as UART14_R;
#[doc = "Field `UART15` reader - "]
pub use GPTMR6_R as UART15_R;
#[doc = "Field `I2C0` reader - "]
pub use GPTMR6_R as I2C0_R;
#[doc = "Field `I2C1` reader - "]
pub use GPTMR6_R as I2C1_R;
#[doc = "Field `I2C2` reader - "]
pub use GPTMR6_R as I2C2_R;
#[doc = "Field `I2C3` reader - "]
pub use GPTMR6_R as I2C3_R;
#[doc = "Field `SPI0` reader - "]
pub use GPTMR6_R as SPI0_R;
#[doc = "Field `SPI1` reader - "]
pub use GPTMR6_R as SPI1_R;
#[doc = "Field `SPI2` reader - "]
pub use GPTMR6_R as SPI2_R;
#[doc = "Field `SPI3` reader - "]
pub use GPTMR6_R as SPI3_R;
#[doc = "Field `CAN0` reader - "]
pub use GPTMR6_R as CAN0_R;
#[doc = "Field `CAN1` reader - "]
pub use GPTMR6_R as CAN1_R;
#[doc = "Field `CAN2` reader - "]
pub use GPTMR6_R as CAN2_R;
#[doc = "Field `CAN3` reader - "]
pub use GPTMR6_R as CAN3_R;
#[doc = "Field `PTPC` reader - "]
pub use GPTMR6_R as PTPC_R;
#[doc = "Field `ADC0` reader - "]
pub use GPTMR6_R as ADC0_R;
#[doc = "Field `GPTMR7` writer - "]
pub use GPTMR6_W as GPTMR7_W;
#[doc = "Field `UART0` writer - "]
pub use GPTMR6_W as UART0_W;
#[doc = "Field `UART1` writer - "]
pub use GPTMR6_W as UART1_W;
#[doc = "Field `UART2` writer - "]
pub use GPTMR6_W as UART2_W;
#[doc = "Field `UART3` writer - "]
pub use GPTMR6_W as UART3_W;
#[doc = "Field `UART4` writer - "]
pub use GPTMR6_W as UART4_W;
#[doc = "Field `UART5` writer - "]
pub use GPTMR6_W as UART5_W;
#[doc = "Field `UART6` writer - "]
pub use GPTMR6_W as UART6_W;
#[doc = "Field `UART7` writer - "]
pub use GPTMR6_W as UART7_W;
#[doc = "Field `UART8` writer - "]
pub use GPTMR6_W as UART8_W;
#[doc = "Field `UART9` writer - "]
pub use GPTMR6_W as UART9_W;
#[doc = "Field `UART10` writer - "]
pub use GPTMR6_W as UART10_W;
#[doc = "Field `UART11` writer - "]
pub use GPTMR6_W as UART11_W;
#[doc = "Field `UART12` writer - "]
pub use GPTMR6_W as UART12_W;
#[doc = "Field `UART13` writer - "]
pub use GPTMR6_W as UART13_W;
#[doc = "Field `UART14` writer - "]
pub use GPTMR6_W as UART14_W;
#[doc = "Field `UART15` writer - "]
pub use GPTMR6_W as UART15_W;
#[doc = "Field `I2C0` writer - "]
pub use GPTMR6_W as I2C0_W;
#[doc = "Field `I2C1` writer - "]
pub use GPTMR6_W as I2C1_W;
#[doc = "Field `I2C2` writer - "]
pub use GPTMR6_W as I2C2_W;
#[doc = "Field `I2C3` writer - "]
pub use GPTMR6_W as I2C3_W;
#[doc = "Field `SPI0` writer - "]
pub use GPTMR6_W as SPI0_W;
#[doc = "Field `SPI1` writer - "]
pub use GPTMR6_W as SPI1_W;
#[doc = "Field `SPI2` writer - "]
pub use GPTMR6_W as SPI2_W;
#[doc = "Field `SPI3` writer - "]
pub use GPTMR6_W as SPI3_W;
#[doc = "Field `CAN0` writer - "]
pub use GPTMR6_W as CAN0_W;
#[doc = "Field `CAN1` writer - "]
pub use GPTMR6_W as CAN1_W;
#[doc = "Field `CAN2` writer - "]
pub use GPTMR6_W as CAN2_W;
#[doc = "Field `CAN3` writer - "]
pub use GPTMR6_W as CAN3_W;
#[doc = "Field `PTPC` writer - "]
pub use GPTMR6_W as PTPC_W;
#[doc = "Field `ADC0` writer - "]
pub use GPTMR6_W as ADC0_W;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gptmr6(&self) -> GPTMR6_R {
        GPTMR6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gptmr7(&self) -> GPTMR7_R {
        GPTMR7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uart5(&self) -> UART5_R {
        UART5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uart6(&self) -> UART6_R {
        UART6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uart7(&self) -> UART7_R {
        UART7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uart8(&self) -> UART8_R {
        UART8_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uart9(&self) -> UART9_R {
        UART9_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uart10(&self) -> UART10_R {
        UART10_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn uart11(&self) -> UART11_R {
        UART11_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn uart12(&self) -> UART12_R {
        UART12_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn uart13(&self) -> UART13_R {
        UART13_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart14(&self) -> UART14_R {
        UART14_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart15(&self) -> UART15_R {
        UART15_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn spi3(&self) -> SPI3_R {
        SPI3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn can2(&self) -> CAN2_R {
        CAN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn can3(&self) -> CAN3_R {
        CAN3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ptpc(&self) -> PTPC_R {
        PTPC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr6(&mut self) -> GPTMR6_W<0> {
        GPTMR6_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr7(&mut self) -> GPTMR7_W<1> {
        GPTMR7_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<2> {
        UART0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<3> {
        UART1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<4> {
        UART2_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<5> {
        UART3_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn uart4(&mut self) -> UART4_W<6> {
        UART4_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn uart5(&mut self) -> UART5_W<7> {
        UART5_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn uart6(&mut self) -> UART6_W<8> {
        UART6_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn uart7(&mut self) -> UART7_W<9> {
        UART7_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn uart8(&mut self) -> UART8_W<10> {
        UART8_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn uart9(&mut self) -> UART9_W<11> {
        UART9_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn uart10(&mut self) -> UART10_W<12> {
        UART10_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn uart11(&mut self) -> UART11_W<13> {
        UART11_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn uart12(&mut self) -> UART12_W<14> {
        UART12_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn uart13(&mut self) -> UART13_W<15> {
        UART13_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn uart14(&mut self) -> UART14_W<16> {
        UART14_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn uart15(&mut self) -> UART15_W<17> {
        UART15_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<18> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<19> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2(&mut self) -> I2C2_W<20> {
        I2C2_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3(&mut self) -> I2C3_W<21> {
        I2C3_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<22> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<23> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<24> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn spi3(&mut self) -> SPI3_W<25> {
        SPI3_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn can0(&mut self) -> CAN0_W<26> {
        CAN0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn can1(&mut self) -> CAN1_W<27> {
        CAN1_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn can2(&mut self) -> CAN2_W<28> {
        CAN2_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn can3(&mut self) -> CAN3_W<29> {
        CAN3_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn ptpc(&mut self) -> PTPC_W<30> {
        PTPC_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<31> {
        ADC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Goup setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group0_1_value](index.html) module"]
pub struct GROUP0_1_VALUE_SPEC;
impl crate::RegisterSpec for GROUP0_1_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [group0_1_value::R](R) reader structure"]
impl crate::Readable for GROUP0_1_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [group0_1_value::W](W) writer structure"]
impl crate::Writable for GROUP0_1_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GROUP0_1_VALUE to value 0"]
impl crate::Resettable for GROUP0_1_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
