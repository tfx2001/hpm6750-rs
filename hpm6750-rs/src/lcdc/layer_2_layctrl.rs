#[doc = "Register `LAYER_2_LAYCTRL` reader"]
pub struct R(crate::R<LAYER_2_LAYCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_2_LAYCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_2_LAYCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_2_LAYCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_2_LAYCTRL` writer"]
pub struct W(crate::W<LAYER_2_LAYCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_2_LAYCTRL_SPEC>;
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
impl From<crate::W<LAYER_2_LAYCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_2_LAYCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, bool, O>;
#[doc = "Field `AB_MODE` reader - Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
pub type AB_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AB_MODE` writer - Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
pub type AB_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `INALPHA_OP` reader - The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\]
is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\]
is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\]
is used to scale the alpha value from previous pipeline Others: Reserved"]
pub type INALPHA_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INALPHA_OP` writer - The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\]
is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\]
is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\]
is used to scale the alpha value from previous pipeline Others: Reserved"]
pub type INALPHA_OP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCALPHA_OP` reader - The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\]
is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\]
is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\]
is used to scale the alpha value from the data stream Others: Reserved"]
pub type LOCALPHA_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCALPHA_OP` writer - The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\]
is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\]
is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\]
is used to scale the alpha value from the data stream Others: Reserved"]
pub type LOCALPHA_OP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PIXFORMAT` reader - Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\]
1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
pub type PIXFORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIXFORMAT` writer - Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\]
1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
pub type PIXFORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `YUV_FORMAT` reader - The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
pub type YUV_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YUV_FORMAT` writer - The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
pub type YUV_FORMAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SHADOW_LOAD_EN` reader - Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
pub type SHADOW_LOAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `SHADOW_LOAD_EN` writer - Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
pub type SHADOW_LOAD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, bool, O>;
#[doc = "Field `PACK_DIR` reader - The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
pub type PACK_DIR_R = crate::BitReader<bool>;
#[doc = "Field `PACK_DIR` writer - The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
pub type PACK_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LAYER_2_LAYCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:5 - Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    #[inline(always)]
    pub fn ab_mode(&self) -> AB_MODE_R {
        AB_MODE_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\]
is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\]
is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\]
is used to scale the alpha value from previous pipeline Others: Reserved"]
    #[inline(always)]
    pub fn inalpha_op(&self) -> INALPHA_OP_R {
        INALPHA_OP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\]
is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\]
is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\]
is used to scale the alpha value from the data stream Others: Reserved"]
    #[inline(always)]
    pub fn localpha_op(&self) -> LOCALPHA_OP_R {
        LOCALPHA_OP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\]
1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    #[inline(always)]
    pub fn pixformat(&self) -> PIXFORMAT_R {
        PIXFORMAT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    #[inline(always)]
    pub fn yuv_format(&self) -> YUV_FORMAT_R {
        YUV_FORMAT_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    #[inline(always)]
    pub fn shadow_load_en(&self) -> SHADOW_LOAD_EN_R {
        SHADOW_LOAD_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 2:5 - Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ab_mode(&mut self) -> AB_MODE_W<2> {
        AB_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\]
is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\]
is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\]
is used to scale the alpha value from previous pipeline Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn inalpha_op(&mut self) -> INALPHA_OP_W<6> {
        INALPHA_OP_W::new(self)
    }
    #[doc = "Bits 8:9 - The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\]
is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\]
is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\]
is used to scale the alpha value from the data stream Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn localpha_op(&mut self) -> LOCALPHA_OP_W<8> {
        LOCALPHA_OP_W::new(self)
    }
    #[doc = "Bits 10:13 - Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\]
1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    #[inline(always)]
    #[must_use]
    pub fn pixformat(&mut self) -> PIXFORMAT_W<10> {
        PIXFORMAT_W::new(self)
    }
    #[doc = "Bits 14:15 - The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_format(&mut self) -> YUV_FORMAT_W<14> {
        YUV_FORMAT_W::new(self)
    }
    #[doc = "Bit 16 - Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_load_en(&mut self) -> SHADOW_LOAD_EN_W<16> {
        SHADOW_LOAD_EN_W::new(self)
    }
    #[doc = "Bit 19 - The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    #[inline(always)]
    #[must_use]
    pub fn pack_dir(&mut self) -> PACK_DIR_W<19> {
        PACK_DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_2_layctrl](index.html) module"]
pub struct LAYER_2_LAYCTRL_SPEC;
impl crate::RegisterSpec for LAYER_2_LAYCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_2_layctrl::R](R) reader structure"]
impl crate::Readable for LAYER_2_LAYCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_2_layctrl::W](W) writer structure"]
impl crate::Writable for LAYER_2_LAYCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYER_2_LAYCTRL to value 0"]
impl crate::Resettable for LAYER_2_LAYCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
