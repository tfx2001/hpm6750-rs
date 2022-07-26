#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCKY` reader - Y block that is processing"]
pub type BLOCKY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLOCKX` reader - X block that is processing"]
pub type BLOCKX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDMA_DONE` reader - PDMA one image done"]
pub type PDMA_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AXI_ERR_ID` reader - AXI error ID"]
pub type AXI_ERR_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AXI_0_WRITE_ERR` reader - AXI0 write err"]
pub type AXI_0_WRITE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AXI_0_WRITE_ERR` writer - AXI0 write err"]
pub type AXI_0_WRITE_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `AXI_1_READ_ERR` reader - AXI1 read err"]
pub type AXI_1_READ_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AXI_1_READ_ERR` writer - AXI1 read err"]
pub type AXI_1_READ_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `AXI_0_READ_ERR` reader - AXI0 read err"]
pub type AXI_0_READ_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AXI_0_READ_ERR` writer - AXI0 read err"]
pub type AXI_0_READ_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `IRQ` reader - Asserted to indicate a IRQ event"]
pub type IRQ_R = crate::BitReader<bool>;
#[doc = "Field `IRQ` writer - Asserted to indicate a IRQ event"]
pub type IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:31 - Y block that is processing"]
    #[inline(always)]
    pub fn blocky(&self) -> BLOCKY_R {
        BLOCKY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - X block that is processing"]
    #[inline(always)]
    pub fn blockx(&self) -> BLOCKX_R {
        BLOCKX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 9 - PDMA one image done"]
    #[inline(always)]
    pub fn pdma_done(&self) -> PDMA_DONE_R {
        PDMA_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 5:8 - AXI error ID"]
    #[inline(always)]
    pub fn axi_err_id(&self) -> AXI_ERR_ID_R {
        AXI_ERR_ID_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 4 - AXI0 write err"]
    #[inline(always)]
    pub fn axi_0_write_err(&self) -> AXI_0_WRITE_ERR_R {
        AXI_0_WRITE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI1 read err"]
    #[inline(always)]
    pub fn axi_1_read_err(&self) -> AXI_1_READ_ERR_R {
        AXI_1_READ_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI0 read err"]
    #[inline(always)]
    pub fn axi_0_read_err(&self) -> AXI_0_READ_ERR_R {
        AXI_0_READ_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Asserted to indicate a IRQ event"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AXI0 write err"]
    #[inline(always)]
    pub fn axi_0_write_err(&mut self) -> AXI_0_WRITE_ERR_W<4> {
        AXI_0_WRITE_ERR_W::new(self)
    }
    #[doc = "Bit 3 - AXI1 read err"]
    #[inline(always)]
    pub fn axi_1_read_err(&mut self) -> AXI_1_READ_ERR_W<3> {
        AXI_1_READ_ERR_W::new(self)
    }
    #[doc = "Bit 2 - AXI0 read err"]
    #[inline(always)]
    pub fn axi_0_read_err(&mut self) -> AXI_0_READ_ERR_W<2> {
        AXI_0_READ_ERR_W::new(self)
    }
    #[doc = "Bit 0 - Asserted to indicate a IRQ event"]
    #[inline(always)]
    pub fn irq(&mut self) -> IRQ_W<0> {
        IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
