#[doc = "Register `PS_1_CTRL` reader"]
pub struct R(crate::R<PS_1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_1_CTRL` writer"]
pub struct W(crate::W<PS_1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_1_CTRL_SPEC>;
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
impl From<crate::W<PS_1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)"]
pub type FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORMAT` writer - PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)"]
pub type FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `HW_BYTE_SWAP` reader - Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
pub type HW_BYTE_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `HW_BYTE_SWAP` writer - Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
pub type HW_BYTE_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
#[doc = "Field `DECX` reader - Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
pub type DECX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECX` writer - Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
pub type DECX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DECY` reader - Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
pub type DECY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECY` writer - Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
pub type DECY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ROTATE` reader - Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270"]
pub type ROTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROTATE` writer - Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270"]
pub type ROTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `HFLIP` reader - Indicates that the input should be flipped horizontally (effect applied before rotation)."]
pub type HFLIP_R = crate::BitReader<bool>;
#[doc = "Field `HFLIP` writer - Indicates that the input should be flipped horizontally (effect applied before rotation)."]
pub type HFLIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
#[doc = "Field `VFLIP` reader - Indicates that the input should be flipped vertically (effect applied before rotation)."]
pub type VFLIP_R = crate::BitReader<bool>;
#[doc = "Field `VFLIP` writer - Indicates that the input should be flipped vertically (effect applied before rotation)."]
pub type VFLIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
#[doc = "Field `BYPASS` reader - Asserted to bypass the CSC stage"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Asserted to bypass the CSC stage"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
#[doc = "Field `YCBCR_MODE` reader - YCbCr mode or YUV mode"]
pub type YCBCR_MODE_R = crate::BitReader<bool>;
#[doc = "Field `YCBCR_MODE` writer - YCbCr mode or YUV mode"]
pub type YCBCR_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
#[doc = "Field `BKGCL4CLR` reader - Enable to use background color for clear area"]
pub type BKGCL4CLR_R = crate::BitReader<bool>;
#[doc = "Field `BKGCL4CLR` writer - Enable to use background color for clear area"]
pub type BKGCL4CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
#[doc = "Field `PACK_DIR` reader - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PACK_DIR` writer - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INB13_SWAP` reader - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack_dir operation."]
pub type INB13_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `INB13_SWAP` writer - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack_dir operation."]
pub type INB13_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PS_1_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
    #[inline(always)]
    pub fn hw_byte_swap(&self) -> HW_BYTE_SWAP_R {
        HW_BYTE_SWAP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
    #[inline(always)]
    pub fn decx(&self) -> DECX_R {
        DECX_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
    #[inline(always)]
    pub fn decy(&self) -> DECY_R {
        DECY_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270"]
    #[inline(always)]
    pub fn rotate(&self) -> ROTATE_R {
        ROTATE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Indicates that the input should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub fn hflip(&self) -> HFLIP_R {
        HFLIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates that the input should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub fn vflip(&self) -> VFLIP_R {
        VFLIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asserted to bypass the CSC stage"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - YCbCr mode or YUV mode"]
    #[inline(always)]
    pub fn ycbcr_mode(&self) -> YCBCR_MODE_R {
        YCBCR_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable to use background color for clear area"]
    #[inline(always)]
    pub fn bkgcl4clr(&self) -> BKGCL4CLR_R {
        BKGCL4CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack_dir operation."]
    #[inline(always)]
    pub fn inb13_swap(&self) -> INB13_SWAP_R {
        INB13_SWAP_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 6 - Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
    #[inline(always)]
    #[must_use]
    pub fn hw_byte_swap(&mut self) -> HW_BYTE_SWAP_W<6> {
        HW_BYTE_SWAP_W::new(self)
    }
    #[doc = "Bits 7:8 - Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
    #[inline(always)]
    #[must_use]
    pub fn decx(&mut self) -> DECX_W<7> {
        DECX_W::new(self)
    }
    #[doc = "Bits 9:10 - Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
    #[inline(always)]
    #[must_use]
    pub fn decy(&mut self) -> DECY_W<9> {
        DECY_W::new(self)
    }
    #[doc = "Bits 11:12 - Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270"]
    #[inline(always)]
    #[must_use]
    pub fn rotate(&mut self) -> ROTATE_W<11> {
        ROTATE_W::new(self)
    }
    #[doc = "Bit 13 - Indicates that the input should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    #[must_use]
    pub fn hflip(&mut self) -> HFLIP_W<13> {
        HFLIP_W::new(self)
    }
    #[doc = "Bit 14 - Indicates that the input should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    #[must_use]
    pub fn vflip(&mut self) -> VFLIP_W<14> {
        VFLIP_W::new(self)
    }
    #[doc = "Bit 15 - Asserted to bypass the CSC stage"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<15> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 16 - YCbCr mode or YUV mode"]
    #[inline(always)]
    #[must_use]
    pub fn ycbcr_mode(&mut self) -> YCBCR_MODE_W<16> {
        YCBCR_MODE_W::new(self)
    }
    #[doc = "Bit 17 - Enable to use background color for clear area"]
    #[inline(always)]
    #[must_use]
    pub fn bkgcl4clr(&mut self) -> BKGCL4CLR_W<17> {
        BKGCL4CLR_W::new(self)
    }
    #[doc = "Bits 18:19 - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    #[must_use]
    pub fn pack_dir(&mut self) -> PACK_DIR_W<18> {
        PACK_DIR_W::new(self)
    }
    #[doc = "Bit 20 - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack_dir operation."]
    #[inline(always)]
    #[must_use]
    pub fn inb13_swap(&mut self) -> INB13_SWAP_W<20> {
        INB13_SWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "layer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_1_ctrl](index.html) module"]
pub struct PS_1_CTRL_SPEC;
impl crate::RegisterSpec for PS_1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_1_ctrl::R](R) reader structure"]
impl crate::Readable for PS_1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_1_ctrl::W](W) writer structure"]
impl crate::Writable for PS_1_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PS_1_CTRL to value 0"]
impl crate::Resettable for PS_1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
