#[doc = "Register `GROUP0_0_VALUE` reader"]
pub struct R(crate::R<GROUP0_0_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GROUP0_0_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GROUP0_0_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GROUP0_0_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GROUP0_0_VALUE` writer"]
pub struct W(crate::W<GROUP0_0_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP0_0_VALUE_SPEC>;
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
impl From<crate::W<GROUP0_0_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP0_0_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBAPB` reader - AHB/APB peripheral bus"]
pub type AHBAPB_R = crate::BitReader<AHBAPB_A>;
#[doc = "AHB/APB peripheral bus\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHBAPB_A {
    #[doc = "0: Unlink from resource group 0"]
    UNLINK = 0,
    #[doc = "1: Link to resource group 0"]
    LINK = 1,
}
impl From<AHBAPB_A> for bool {
    #[inline(always)]
    fn from(variant: AHBAPB_A) -> Self {
        variant as u8 != 0
    }
}
impl AHBAPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBAPB_A {
        match self.bits {
            false => AHBAPB_A::UNLINK,
            true => AHBAPB_A::LINK,
        }
    }
    #[doc = "Checks if the value of the field is `UNLINK`"]
    #[inline(always)]
    pub fn is_unlink(&self) -> bool {
        *self == AHBAPB_A::UNLINK
    }
    #[doc = "Checks if the value of the field is `LINK`"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == AHBAPB_A::LINK
    }
}
#[doc = "Field `AHBAPB` writer - AHB/APB peripheral bus"]
pub type AHBAPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GROUP0_0_VALUE_SPEC, AHBAPB_A, O>;
impl<'a, const O: u8> AHBAPB_W<'a, O> {
    #[doc = "Unlink from resource group 0"]
    #[inline(always)]
    pub fn unlink(self) -> &'a mut W {
        self.variant(AHBAPB_A::UNLINK)
    }
    #[doc = "Link to resource group 0"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(AHBAPB_A::LINK)
    }
}
#[doc = "Field `AXI` reader - AXI system bus"]
pub use AHBAPB_R as AXI_R;
#[doc = "Field `CONN` reader - CONN subsystem bus"]
pub use AHBAPB_R as CONN_R;
#[doc = "Field `VIS` reader - "]
pub use AHBAPB_R as VIS_R;
#[doc = "Field `DRAM` reader - "]
pub use AHBAPB_R as DRAM_R;
#[doc = "Field `ROM` reader - "]
pub use AHBAPB_R as ROM_R;
#[doc = "Field `ILM_DLM0` reader - "]
pub use AHBAPB_R as ILM_DLM0_R;
#[doc = "Field `ILM_DLM1` reader - "]
pub use AHBAPB_R as ILM_DLM1_R;
#[doc = "Field `MCHTMR0` reader - "]
pub use AHBAPB_R as MCHTMR0_R;
#[doc = "Field `MCHTMR1` reader - "]
pub use AHBAPB_R as MCHTMR1_R;
#[doc = "Field `AXI_SRAM0` reader - "]
pub use AHBAPB_R as AXI_SRAM0_R;
#[doc = "Field `AXI_SRAM1` reader - "]
pub use AHBAPB_R as AXI_SRAM1_R;
#[doc = "Field `XPI0` reader - "]
pub use AHBAPB_R as XPI0_R;
#[doc = "Field `XPI1` reader - "]
pub use AHBAPB_R as XPI1_R;
#[doc = "Field `SDP` reader - "]
pub use AHBAPB_R as SDP_R;
#[doc = "Field `RNG` reader - "]
pub use AHBAPB_R as RNG_R;
#[doc = "Field `KEYM` reader - "]
pub use AHBAPB_R as KEYM_R;
#[doc = "Field `HDMA` reader - "]
pub use AHBAPB_R as HDMA_R;
#[doc = "Field `XDMA` reader - "]
pub use AHBAPB_R as XDMA_R;
#[doc = "Field `GPIO` reader - "]
pub use AHBAPB_R as GPIO_R;
#[doc = "Field `MBX0` reader - "]
pub use AHBAPB_R as MBX0_R;
#[doc = "Field `MBX1` reader - "]
pub use AHBAPB_R as MBX1_R;
#[doc = "Field `WDG0` reader - "]
pub use AHBAPB_R as WDG0_R;
#[doc = "Field `WDG1` reader - "]
pub use AHBAPB_R as WDG1_R;
#[doc = "Field `WDG2` reader - "]
pub use AHBAPB_R as WDG2_R;
#[doc = "Field `WDG3` reader - "]
pub use AHBAPB_R as WDG3_R;
#[doc = "Field `GPTMR0` reader - "]
pub use AHBAPB_R as GPTMR0_R;
#[doc = "Field `GPTMR1` reader - "]
pub use AHBAPB_R as GPTMR1_R;
#[doc = "Field `GPTMR2` reader - "]
pub use AHBAPB_R as GPTMR2_R;
#[doc = "Field `GPTMR3` reader - "]
pub use AHBAPB_R as GPTMR3_R;
#[doc = "Field `GPTMR4` reader - "]
pub use AHBAPB_R as GPTMR4_R;
#[doc = "Field `GPTMR5` reader - "]
pub use AHBAPB_R as GPTMR5_R;
#[doc = "Field `AXI` writer - AXI system bus"]
pub use AHBAPB_W as AXI_W;
#[doc = "Field `CONN` writer - CONN subsystem bus"]
pub use AHBAPB_W as CONN_W;
#[doc = "Field `VIS` writer - "]
pub use AHBAPB_W as VIS_W;
#[doc = "Field `DRAM` writer - "]
pub use AHBAPB_W as DRAM_W;
#[doc = "Field `ROM` writer - "]
pub use AHBAPB_W as ROM_W;
#[doc = "Field `ILM_DLM0` writer - "]
pub use AHBAPB_W as ILM_DLM0_W;
#[doc = "Field `ILM_DLM1` writer - "]
pub use AHBAPB_W as ILM_DLM1_W;
#[doc = "Field `MCHTMR0` writer - "]
pub use AHBAPB_W as MCHTMR0_W;
#[doc = "Field `MCHTMR1` writer - "]
pub use AHBAPB_W as MCHTMR1_W;
#[doc = "Field `AXI_SRAM0` writer - "]
pub use AHBAPB_W as AXI_SRAM0_W;
#[doc = "Field `AXI_SRAM1` writer - "]
pub use AHBAPB_W as AXI_SRAM1_W;
#[doc = "Field `XPI0` writer - "]
pub use AHBAPB_W as XPI0_W;
#[doc = "Field `XPI1` writer - "]
pub use AHBAPB_W as XPI1_W;
#[doc = "Field `SDP` writer - "]
pub use AHBAPB_W as SDP_W;
#[doc = "Field `RNG` writer - "]
pub use AHBAPB_W as RNG_W;
#[doc = "Field `KEYM` writer - "]
pub use AHBAPB_W as KEYM_W;
#[doc = "Field `HDMA` writer - "]
pub use AHBAPB_W as HDMA_W;
#[doc = "Field `XDMA` writer - "]
pub use AHBAPB_W as XDMA_W;
#[doc = "Field `GPIO` writer - "]
pub use AHBAPB_W as GPIO_W;
#[doc = "Field `MBX0` writer - "]
pub use AHBAPB_W as MBX0_W;
#[doc = "Field `MBX1` writer - "]
pub use AHBAPB_W as MBX1_W;
#[doc = "Field `WDG0` writer - "]
pub use AHBAPB_W as WDG0_W;
#[doc = "Field `WDG1` writer - "]
pub use AHBAPB_W as WDG1_W;
#[doc = "Field `WDG2` writer - "]
pub use AHBAPB_W as WDG2_W;
#[doc = "Field `WDG3` writer - "]
pub use AHBAPB_W as WDG3_W;
#[doc = "Field `GPTMR0` writer - "]
pub use AHBAPB_W as GPTMR0_W;
#[doc = "Field `GPTMR1` writer - "]
pub use AHBAPB_W as GPTMR1_W;
#[doc = "Field `GPTMR2` writer - "]
pub use AHBAPB_W as GPTMR2_W;
#[doc = "Field `GPTMR3` writer - "]
pub use AHBAPB_W as GPTMR3_W;
#[doc = "Field `GPTMR4` writer - "]
pub use AHBAPB_W as GPTMR4_W;
#[doc = "Field `GPTMR5` writer - "]
pub use AHBAPB_W as GPTMR5_W;
impl R {
    #[doc = "Bit 0 - AHB/APB peripheral bus"]
    #[inline(always)]
    pub fn ahbapb(&self) -> AHBAPB_R {
        AHBAPB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI system bus"]
    #[inline(always)]
    pub fn axi(&self) -> AXI_R {
        AXI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CONN subsystem bus"]
    #[inline(always)]
    pub fn conn(&self) -> CONN_R {
        CONN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vis(&self) -> VIS_R {
        VIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dram(&self) -> DRAM_R {
        DRAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ilm_dlm0(&self) -> ILM_DLM0_R {
        ILM_DLM0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ilm_dlm1(&self) -> ILM_DLM1_R {
        ILM_DLM1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mchtmr0(&self) -> MCHTMR0_R {
        MCHTMR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mchtmr1(&self) -> MCHTMR1_R {
        MCHTMR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn axi_sram0(&self) -> AXI_SRAM0_R {
        AXI_SRAM0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn axi_sram1(&self) -> AXI_SRAM1_R {
        AXI_SRAM1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xpi0(&self) -> XPI0_R {
        XPI0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn xpi1(&self) -> XPI1_R {
        XPI1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sdp(&self) -> SDP_R {
        SDP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn keym(&self) -> KEYM_R {
        KEYM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hdma(&self) -> HDMA_R {
        HDMA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn xdma(&self) -> XDMA_R {
        XDMA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn mbx0(&self) -> MBX0_R {
        MBX0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn mbx1(&self) -> MBX1_R {
        MBX1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wdg0(&self) -> WDG0_R {
        WDG0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wdg1(&self) -> WDG1_R {
        WDG1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdg2(&self) -> WDG2_R {
        WDG2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdg3(&self) -> WDG3_R {
        WDG3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gptmr0(&self) -> GPTMR0_R {
        GPTMR0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gptmr1(&self) -> GPTMR1_R {
        GPTMR1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gptmr2(&self) -> GPTMR2_R {
        GPTMR2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gptmr3(&self) -> GPTMR3_R {
        GPTMR3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gptmr4(&self) -> GPTMR4_R {
        GPTMR4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gptmr5(&self) -> GPTMR5_R {
        GPTMR5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB/APB peripheral bus"]
    #[inline(always)]
    #[must_use]
    pub fn ahbapb(&mut self) -> AHBAPB_W<0> {
        AHBAPB_W::new(self)
    }
    #[doc = "Bit 1 - AXI system bus"]
    #[inline(always)]
    #[must_use]
    pub fn axi(&mut self) -> AXI_W<1> {
        AXI_W::new(self)
    }
    #[doc = "Bit 2 - CONN subsystem bus"]
    #[inline(always)]
    #[must_use]
    pub fn conn(&mut self) -> CONN_W<2> {
        CONN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vis(&mut self) -> VIS_W<3> {
        VIS_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dram(&mut self) -> DRAM_W<4> {
        DRAM_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<5> {
        ROM_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ilm_dlm0(&mut self) -> ILM_DLM0_W<6> {
        ILM_DLM0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ilm_dlm1(&mut self) -> ILM_DLM1_W<7> {
        ILM_DLM1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn mchtmr0(&mut self) -> MCHTMR0_W<8> {
        MCHTMR0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn mchtmr1(&mut self) -> MCHTMR1_W<9> {
        MCHTMR1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn axi_sram0(&mut self) -> AXI_SRAM0_W<10> {
        AXI_SRAM0_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn axi_sram1(&mut self) -> AXI_SRAM1_W<11> {
        AXI_SRAM1_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xpi0(&mut self) -> XPI0_W<12> {
        XPI0_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn xpi1(&mut self) -> XPI1_W<13> {
        XPI1_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn sdp(&mut self) -> SDP_W<14> {
        SDP_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<15> {
        RNG_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn keym(&mut self) -> KEYM_W<16> {
        KEYM_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn hdma(&mut self) -> HDMA_W<17> {
        HDMA_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn xdma(&mut self) -> XDMA_W<18> {
        XDMA_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GPIO_W<19> {
        GPIO_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn mbx0(&mut self) -> MBX0_W<20> {
        MBX0_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn mbx1(&mut self) -> MBX1_W<21> {
        MBX1_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn wdg0(&mut self) -> WDG0_W<22> {
        WDG0_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wdg1(&mut self) -> WDG1_W<23> {
        WDG1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wdg2(&mut self) -> WDG2_W<24> {
        WDG2_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wdg3(&mut self) -> WDG3_W<25> {
        WDG3_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr0(&mut self) -> GPTMR0_W<26> {
        GPTMR0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr1(&mut self) -> GPTMR1_W<27> {
        GPTMR1_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr2(&mut self) -> GPTMR2_W<28> {
        GPTMR2_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr3(&mut self) -> GPTMR3_W<29> {
        GPTMR3_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr4(&mut self) -> GPTMR4_W<30> {
        GPTMR4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn gptmr5(&mut self) -> GPTMR5_W<31> {
        GPTMR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Goup setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group0_0_value](index.html) module"]
pub struct GROUP0_0_VALUE_SPEC;
impl crate::RegisterSpec for GROUP0_0_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [group0_0_value::R](R) reader structure"]
impl crate::Readable for GROUP0_0_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [group0_0_value::W](W) writer structure"]
impl crate::Writable for GROUP0_0_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GROUP0_0_VALUE to value 0x23"]
impl crate::Resettable for GROUP0_0_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0x23;
}
