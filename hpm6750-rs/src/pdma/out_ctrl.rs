#[doc = "Register `OUT_CTRL` reader"]
pub struct R(crate::R<OUT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CTRL` writer"]
pub struct W(crate::W<OUT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CTRL_SPEC>;
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
impl From<crate::W<OUT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)"]
pub type FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORMAT` writer - Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)"]
pub type FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `ABLEND_MODE` reader - Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\]
is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
pub type ABLEND_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABLEND_MODE` writer - Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\]
is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
pub type ABLEND_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SRCALPHA_OP` reader - The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
pub type SRCALPHA_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCALPHA_OP` writer - The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
pub type SRCALPHA_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DSTALPHA_OP` reader - The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
pub type DSTALPHA_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSTALPHA_OP` writer - The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
pub type DSTALPHA_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SRCALPHA` reader - The source (P0) system ALPHA value."]
pub type SRCALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCALPHA` writer - The source (P0) system ALPHA value."]
pub type SRCALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DSTALPHA` reader - The destination (P1) system ALPHA value."]
pub type DSTALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSTALPHA` writer - The destination (P1) system ALPHA value."]
pub type DSTALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:5 - Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\]
is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    #[inline(always)]
    pub fn ablend_mode(&self) -> ABLEND_MODE_R {
        ABLEND_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
    #[inline(always)]
    pub fn srcalpha_op(&self) -> SRCALPHA_OP_R {
        SRCALPHA_OP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
    #[inline(always)]
    pub fn dstalpha_op(&self) -> DSTALPHA_OP_R {
        DSTALPHA_OP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - The source (P0) system ALPHA value."]
    #[inline(always)]
    pub fn srcalpha(&self) -> SRCALPHA_R {
        SRCALPHA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The destination (P1) system ALPHA value."]
    #[inline(always)]
    pub fn dstalpha(&self) -> DSTALPHA_R {
        DSTALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)"]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Bits 8:11 - Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\]
is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ablend_mode(&mut self) -> ABLEND_MODE_W<8> {
        ABLEND_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn srcalpha_op(&mut self) -> SRCALPHA_OP_W<12> {
        SRCALPHA_OP_W::new(self)
    }
    #[doc = "Bits 14:15 - The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dstalpha_op(&mut self) -> DSTALPHA_OP_W<14> {
        DSTALPHA_OP_W::new(self)
    }
    #[doc = "Bits 16:23 - The source (P0) system ALPHA value."]
    #[inline(always)]
    #[must_use]
    pub fn srcalpha(&mut self) -> SRCALPHA_W<16> {
        SRCALPHA_W::new(self)
    }
    #[doc = "Bits 24:31 - The destination (P1) system ALPHA value."]
    #[inline(always)]
    #[must_use]
    pub fn dstalpha(&mut self) -> DSTALPHA_W<24> {
        DSTALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out Layer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ctrl](index.html) module"]
pub struct OUT_CTRL_SPEC;
impl crate::RegisterSpec for OUT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_ctrl::R](R) reader structure"]
impl crate::Readable for OUT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_ctrl::W](W) writer structure"]
impl crate::Writable for OUT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CTRL to value 0"]
impl crate::Resettable for OUT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
