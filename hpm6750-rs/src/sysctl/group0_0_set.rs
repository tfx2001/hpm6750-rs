#[doc = "Register `GROUP0_0_SET` writer"]
pub struct W(crate::W<GROUP0_0_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP0_0_SET_SPEC>;
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
impl From<crate::W<GROUP0_0_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP0_0_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AHB/APB peripheral bus\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHBAPB_AW {
    #[doc = "1: Link to resource group"]
    LINK = 1,
}
impl From<AHBAPB_AW> for bool {
    #[inline(always)]
    fn from(variant: AHBAPB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBAPB` writer - AHB/APB peripheral bus"]
pub type AHBAPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GROUP0_0_SET_SPEC, AHBAPB_AW, O>;
impl<'a, const O: u8> AHBAPB_W<'a, O> {
    #[doc = "Link to resource group"]
    #[inline(always)]
    pub fn link(self) -> &'a mut W {
        self.variant(AHBAPB_AW::LINK)
    }
}
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
#[doc = "Group set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group0_0_set](index.html) module"]
pub struct GROUP0_0_SET_SPEC;
impl crate::RegisterSpec for GROUP0_0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [group0_0_set::W](W) writer structure"]
impl crate::Writable for GROUP0_0_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GROUP0_0_SET to value 0x23"]
impl crate::Resettable for GROUP0_0_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x23;
}
