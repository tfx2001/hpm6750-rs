#[doc = "Register `BR_BASE0` reader"]
pub struct R(crate::R<BR_BASE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BR_BASE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BR_BASE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BR_BASE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BR_BASE0` writer"]
pub struct W(crate::W<BR_BASE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BR_BASE0_SPEC>;
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
impl From<crate::W<BR_BASE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BR_BASE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
pub type BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE` writer - Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
pub type BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_BASE0_SPEC, u32, u32, 20, O>;
#[doc = "Field `SIZE` reader - Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_BASE0_SPEC, u8, u8, 5, O>;
#[doc = "Field `VLD` reader - Valid"]
pub type VLD_R = crate::BitReader<bool>;
#[doc = "Field `VLD` writer - Valid"]
pub type VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BR_BASE0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 12:31 - Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 1:5 - Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W<12> {
        BASE_W::new(self)
    }
    #[doc = "Bits 1:5 - Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<1> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&mut self) -> VLD_W<0> {
        VLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base Register 0 (for SDRAM CS0 device)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br_base0](index.html) module"]
pub struct BR_BASE0_SPEC;
impl crate::RegisterSpec for BR_BASE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [br_base0::R](R) reader structure"]
impl crate::Readable for BR_BASE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [br_base0::W](W) writer structure"]
impl crate::Writable for BR_BASE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BR_BASE0 to value 0"]
impl crate::Resettable for BR_BASE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
