#[doc = "Register `AUTO_TUNING_CTRL` reader"]
pub struct R(crate::R<AUTO_TUNING_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTO_TUNING_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTO_TUNING_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTO_TUNING_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTO_TUNING_CTRL` writer"]
pub struct W(crate::W<AUTO_TUNING_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTO_TUNING_CTRL_SPEC>;
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
impl From<crate::W<AUTO_TUNING_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTO_TUNING_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AT_EN` reader - Setting this bit enables Auto tuning engine. This bit is enabled by default when core is configured with mode3 retuning support. Clear this bit to 0 when core is configured to have Mode3 re-tuning but SW wishes to disable mode3 retuning. This field should be programmed only when CLK_CTRL_R.SD_CLK_EN is 0. Values: 0x1 (AT_ENABLE): AutoTuning is enabled 0x0 (AT_DISABLE): AutoTuning is disabled"]
pub type AT_EN_R = crate::BitReader<bool>;
#[doc = "Field `AT_EN` writer - Setting this bit enables Auto tuning engine. This bit is enabled by default when core is configured with mode3 retuning support. Clear this bit to 0 when core is configured to have Mode3 re-tuning but SW wishes to disable mode3 retuning. This field should be programmed only when CLK_CTRL_R.SD_CLK_EN is 0. Values: 0x1 (AT_ENABLE): AutoTuning is enabled 0x0 (AT_DISABLE): AutoTuning is disabled"]
pub type AT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `CI_SEL` reader - Selects the interval when the corrected center phase select code can be driven on tuning_cclk_sel output. Values: 0x0 (WHEN_IN_BLK_GAP): Driven in block gap interval 0x1 (WHEN_IN_IDLE): Driven at the end of the transfer"]
pub type CI_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CI_SEL` writer - Selects the interval when the corrected center phase select code can be driven on tuning_cclk_sel output. Values: 0x0 (WHEN_IN_BLK_GAP): Driven in block gap interval 0x1 (WHEN_IN_IDLE): Driven at the end of the transfer"]
pub type CI_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `SWIN_TH_EN` reader - Sampling window Threshold enable Selects the tuning mode Field should be programmed only when SAMPLE_CLK_SEL is '0' Values: 0x1 (THRESHOLD_MODE): Tuning engine selects the first complete sampling window that meets the threshold set by SWIN_TH_VAL field 0x0 (LARGEST_WIN_MODE): Tuning engine sweeps all taps and settles at the largest window"]
pub type SWIN_TH_EN_R = crate::BitReader<bool>;
#[doc = "Field `SWIN_TH_EN` writer - Sampling window Threshold enable Selects the tuning mode Field should be programmed only when SAMPLE_CLK_SEL is '0' Values: 0x1 (THRESHOLD_MODE): Tuning engine selects the first complete sampling window that meets the threshold set by SWIN_TH_VAL field 0x0 (LARGEST_WIN_MODE): Tuning engine sweeps all taps and settles at the largest window"]
pub type SWIN_TH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `RPT_TUNE_ERR` reader - Framing errors are not generated when executing tuning. This debug bit allows users to report these errors. Values: 0x1 (DEBUG_ERRORS): Debug mode for reporting framing errors 0x0 (ERRORS_DISABLED): Default mode where as per SDXC no errors are reported."]
pub type RPT_TUNE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RPT_TUNE_ERR` writer - Framing errors are not generated when executing tuning. This debug bit allows users to report these errors. Values: 0x1 (DEBUG_ERRORS): Debug mode for reporting framing errors 0x0 (ERRORS_DISABLED): Default mode where as per SDXC no errors are reported."]
pub type RPT_TUNE_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `SW_TUNE_EN` reader - This fields enables software-managed tuning flow. Values: 0x1 (SW_TUNING_ENABLE): Software-managed tuning enabled. AT_STAT_R.CENTER_PH_CODE Field is now writable. 0x0 (SW_TUNING_DISABLE): Software-managed tuning disabled"]
pub type SW_TUNE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SW_TUNE_EN` writer - This fields enables software-managed tuning flow. Values: 0x1 (SW_TUNING_ENABLE): Software-managed tuning enabled. AT_STAT_R.CENTER_PH_CODE Field is now writable. 0x0 (SW_TUNING_DISABLE): Software-managed tuning disabled"]
pub type SW_TUNE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `WIN_EDGE_SEL` reader - This field sets the phase for Left and Right edges for drift monitoring. \\[Left edge offset + Right edge offset\\]
must not be less than total taps of delayLine. 0x0: User selection disabled. Tuning calculated edges are used. 0x1: Right edge Phase is center + 2 stages, Left edge Phase is center - 2 stages. 0x2: Right edge Phase is center + 3 stages, Left edge Phase is center - 3 stagess ... 0xF: Right edge Phase is center + 16 stages, Left edge Phase is center - 16 stages."]
pub type WIN_EDGE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIN_EDGE_SEL` writer - This field sets the phase for Left and Right edges for drift monitoring. \\[Left edge offset + Right edge offset\\]
must not be less than total taps of delayLine. 0x0: User selection disabled. Tuning calculated edges are used. 0x1: Right edge Phase is center + 2 stages, Left edge Phase is center - 2 stages. 0x2: Right edge Phase is center + 3 stages, Left edge Phase is center - 3 stagess ... 0xF: Right edge Phase is center + 16 stages, Left edge Phase is center - 16 stages."]
pub type WIN_EDGE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TUNE_CLK_STOP_EN` reader - Clock stopping control for Tuning and auto-tuning circuit. When enabled, clock gate control output of SDXC (clk2card_on) is pulled low before changing phase select codes on tuning_cclk_sel and autotuning_cclk_sel. This effectively stops the Device/Card clock, cclk_rx and also drift_cclk_rx. Changing phase code when clocks are stopped ensures glitch free phase switching. Set this bit to 0 if the PHY or delayline can guarantee glitch free switching. Values: 0x1 (ENABLE_CLK_STOPPING): Clocks stopped during phase code change 0x0 (DISABLE_CLK_STOPPING): Clocks not stopped. PHY ensures glitch free phase switching"]
pub type TUNE_CLK_STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TUNE_CLK_STOP_EN` writer - Clock stopping control for Tuning and auto-tuning circuit. When enabled, clock gate control output of SDXC (clk2card_on) is pulled low before changing phase select codes on tuning_cclk_sel and autotuning_cclk_sel. This effectively stops the Device/Card clock, cclk_rx and also drift_cclk_rx. Changing phase code when clocks are stopped ensures glitch free phase switching. Set this bit to 0 if the PHY or delayline can guarantee glitch free switching. Values: 0x1 (ENABLE_CLK_STOPPING): Clocks stopped during phase code change 0x0 (DISABLE_CLK_STOPPING): Clocks not stopped. PHY ensures glitch free phase switching"]
pub type TUNE_CLK_STOP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, bool, O>;
#[doc = "Field `PRE_CHANGE_DLY` reader - Maximum Latency specification between cclk_tx and cclk_rx. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
pub type PRE_CHANGE_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_CHANGE_DLY` writer - Maximum Latency specification between cclk_tx and cclk_rx. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
pub type PRE_CHANGE_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `POST_CHANGE_DLY` reader - Time taken for phase switching and stable clock output. Specifies the maximum time (in terms of cclk cycles) that the delay line can take to switch its output phase after a change in tuning_cclk_sel or autotuning_cclk_sel. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
pub type POST_CHANGE_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POST_CHANGE_DLY` writer - Time taken for phase switching and stable clock output. Specifies the maximum time (in terms of cclk cycles) that the delay line can take to switch its output phase after a change in tuning_cclk_sel or autotuning_cclk_sel. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
pub type POST_CHANGE_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWIN_TH_VAL` reader - Sampling window threshold value setting The maximum value that can be set here depends on the length of delayline used for tuning. A delayLine with 32 taps can use values from 0x0 to 0x1F. This field is valid only when SWIN_TH_EN is '1'. Should be programmed only when SAMPLE_CLK_SEL is '0' 0x0 : Threshold values is 0x1, windows of length 1 tap and above can be selected as sampling window. 0x1 : Threshold values is 0x2, windows of length 2 taps and above can be selected as sampling window. 0x2 : Threshold values is 0x1, windows of length 3 taps and above can be selected as sampling window. ........ 0x1F : Threshold values is 0x1, windows of length 32 taps and above can be selected as sampling window."]
pub type SWIN_TH_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWIN_TH_VAL` writer - Sampling window threshold value setting The maximum value that can be set here depends on the length of delayline used for tuning. A delayLine with 32 taps can use values from 0x0 to 0x1F. This field is valid only when SWIN_TH_EN is '1'. Should be programmed only when SAMPLE_CLK_SEL is '0' 0x0 : Threshold values is 0x1, windows of length 1 tap and above can be selected as sampling window. 0x1 : Threshold values is 0x2, windows of length 2 taps and above can be selected as sampling window. 0x2 : Threshold values is 0x1, windows of length 3 taps and above can be selected as sampling window. ........ 0x1F : Threshold values is 0x1, windows of length 32 taps and above can be selected as sampling window."]
pub type SWIN_TH_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTO_TUNING_CTRL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Setting this bit enables Auto tuning engine. This bit is enabled by default when core is configured with mode3 retuning support. Clear this bit to 0 when core is configured to have Mode3 re-tuning but SW wishes to disable mode3 retuning. This field should be programmed only when CLK_CTRL_R.SD_CLK_EN is 0. Values: 0x1 (AT_ENABLE): AutoTuning is enabled 0x0 (AT_DISABLE): AutoTuning is disabled"]
    #[inline(always)]
    pub fn at_en(&self) -> AT_EN_R {
        AT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the interval when the corrected center phase select code can be driven on tuning_cclk_sel output. Values: 0x0 (WHEN_IN_BLK_GAP): Driven in block gap interval 0x1 (WHEN_IN_IDLE): Driven at the end of the transfer"]
    #[inline(always)]
    pub fn ci_sel(&self) -> CI_SEL_R {
        CI_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sampling window Threshold enable Selects the tuning mode Field should be programmed only when SAMPLE_CLK_SEL is '0' Values: 0x1 (THRESHOLD_MODE): Tuning engine selects the first complete sampling window that meets the threshold set by SWIN_TH_VAL field 0x0 (LARGEST_WIN_MODE): Tuning engine sweeps all taps and settles at the largest window"]
    #[inline(always)]
    pub fn swin_th_en(&self) -> SWIN_TH_EN_R {
        SWIN_TH_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing errors are not generated when executing tuning. This debug bit allows users to report these errors. Values: 0x1 (DEBUG_ERRORS): Debug mode for reporting framing errors 0x0 (ERRORS_DISABLED): Default mode where as per SDXC no errors are reported."]
    #[inline(always)]
    pub fn rpt_tune_err(&self) -> RPT_TUNE_ERR_R {
        RPT_TUNE_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This fields enables software-managed tuning flow. Values: 0x1 (SW_TUNING_ENABLE): Software-managed tuning enabled. AT_STAT_R.CENTER_PH_CODE Field is now writable. 0x0 (SW_TUNING_DISABLE): Software-managed tuning disabled"]
    #[inline(always)]
    pub fn sw_tune_en(&self) -> SW_TUNE_EN_R {
        SW_TUNE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - This field sets the phase for Left and Right edges for drift monitoring. \\[Left edge offset + Right edge offset\\]
must not be less than total taps of delayLine. 0x0: User selection disabled. Tuning calculated edges are used. 0x1: Right edge Phase is center + 2 stages, Left edge Phase is center - 2 stages. 0x2: Right edge Phase is center + 3 stages, Left edge Phase is center - 3 stagess ... 0xF: Right edge Phase is center + 16 stages, Left edge Phase is center - 16 stages."]
    #[inline(always)]
    pub fn win_edge_sel(&self) -> WIN_EDGE_SEL_R {
        WIN_EDGE_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Clock stopping control for Tuning and auto-tuning circuit. When enabled, clock gate control output of SDXC (clk2card_on) is pulled low before changing phase select codes on tuning_cclk_sel and autotuning_cclk_sel. This effectively stops the Device/Card clock, cclk_rx and also drift_cclk_rx. Changing phase code when clocks are stopped ensures glitch free phase switching. Set this bit to 0 if the PHY or delayline can guarantee glitch free switching. Values: 0x1 (ENABLE_CLK_STOPPING): Clocks stopped during phase code change 0x0 (DISABLE_CLK_STOPPING): Clocks not stopped. PHY ensures glitch free phase switching"]
    #[inline(always)]
    pub fn tune_clk_stop_en(&self) -> TUNE_CLK_STOP_EN_R {
        TUNE_CLK_STOP_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Maximum Latency specification between cclk_tx and cclk_rx. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    #[inline(always)]
    pub fn pre_change_dly(&self) -> PRE_CHANGE_DLY_R {
        PRE_CHANGE_DLY_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Time taken for phase switching and stable clock output. Specifies the maximum time (in terms of cclk cycles) that the delay line can take to switch its output phase after a change in tuning_cclk_sel or autotuning_cclk_sel. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    #[inline(always)]
    pub fn post_change_dly(&self) -> POST_CHANGE_DLY_R {
        POST_CHANGE_DLY_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 24:30 - Sampling window threshold value setting The maximum value that can be set here depends on the length of delayline used for tuning. A delayLine with 32 taps can use values from 0x0 to 0x1F. This field is valid only when SWIN_TH_EN is '1'. Should be programmed only when SAMPLE_CLK_SEL is '0' 0x0 : Threshold values is 0x1, windows of length 1 tap and above can be selected as sampling window. 0x1 : Threshold values is 0x2, windows of length 2 taps and above can be selected as sampling window. 0x2 : Threshold values is 0x1, windows of length 3 taps and above can be selected as sampling window. ........ 0x1F : Threshold values is 0x1, windows of length 32 taps and above can be selected as sampling window."]
    #[inline(always)]
    pub fn swin_th_val(&self) -> SWIN_TH_VAL_R {
        SWIN_TH_VAL_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit enables Auto tuning engine. This bit is enabled by default when core is configured with mode3 retuning support. Clear this bit to 0 when core is configured to have Mode3 re-tuning but SW wishes to disable mode3 retuning. This field should be programmed only when CLK_CTRL_R.SD_CLK_EN is 0. Values: 0x1 (AT_ENABLE): AutoTuning is enabled 0x0 (AT_DISABLE): AutoTuning is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn at_en(&mut self) -> AT_EN_W<0> {
        AT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Selects the interval when the corrected center phase select code can be driven on tuning_cclk_sel output. Values: 0x0 (WHEN_IN_BLK_GAP): Driven in block gap interval 0x1 (WHEN_IN_IDLE): Driven at the end of the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ci_sel(&mut self) -> CI_SEL_W<1> {
        CI_SEL_W::new(self)
    }
    #[doc = "Bit 2 - Sampling window Threshold enable Selects the tuning mode Field should be programmed only when SAMPLE_CLK_SEL is '0' Values: 0x1 (THRESHOLD_MODE): Tuning engine selects the first complete sampling window that meets the threshold set by SWIN_TH_VAL field 0x0 (LARGEST_WIN_MODE): Tuning engine sweeps all taps and settles at the largest window"]
    #[inline(always)]
    #[must_use]
    pub fn swin_th_en(&mut self) -> SWIN_TH_EN_W<2> {
        SWIN_TH_EN_W::new(self)
    }
    #[doc = "Bit 3 - Framing errors are not generated when executing tuning. This debug bit allows users to report these errors. Values: 0x1 (DEBUG_ERRORS): Debug mode for reporting framing errors 0x0 (ERRORS_DISABLED): Default mode where as per SDXC no errors are reported."]
    #[inline(always)]
    #[must_use]
    pub fn rpt_tune_err(&mut self) -> RPT_TUNE_ERR_W<3> {
        RPT_TUNE_ERR_W::new(self)
    }
    #[doc = "Bit 4 - This fields enables software-managed tuning flow. Values: 0x1 (SW_TUNING_ENABLE): Software-managed tuning enabled. AT_STAT_R.CENTER_PH_CODE Field is now writable. 0x0 (SW_TUNING_DISABLE): Software-managed tuning disabled"]
    #[inline(always)]
    #[must_use]
    pub fn sw_tune_en(&mut self) -> SW_TUNE_EN_W<4> {
        SW_TUNE_EN_W::new(self)
    }
    #[doc = "Bits 8:11 - This field sets the phase for Left and Right edges for drift monitoring. \\[Left edge offset + Right edge offset\\]
must not be less than total taps of delayLine. 0x0: User selection disabled. Tuning calculated edges are used. 0x1: Right edge Phase is center + 2 stages, Left edge Phase is center - 2 stages. 0x2: Right edge Phase is center + 3 stages, Left edge Phase is center - 3 stagess ... 0xF: Right edge Phase is center + 16 stages, Left edge Phase is center - 16 stages."]
    #[inline(always)]
    #[must_use]
    pub fn win_edge_sel(&mut self) -> WIN_EDGE_SEL_W<8> {
        WIN_EDGE_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Clock stopping control for Tuning and auto-tuning circuit. When enabled, clock gate control output of SDXC (clk2card_on) is pulled low before changing phase select codes on tuning_cclk_sel and autotuning_cclk_sel. This effectively stops the Device/Card clock, cclk_rx and also drift_cclk_rx. Changing phase code when clocks are stopped ensures glitch free phase switching. Set this bit to 0 if the PHY or delayline can guarantee glitch free switching. Values: 0x1 (ENABLE_CLK_STOPPING): Clocks stopped during phase code change 0x0 (DISABLE_CLK_STOPPING): Clocks not stopped. PHY ensures glitch free phase switching"]
    #[inline(always)]
    #[must_use]
    pub fn tune_clk_stop_en(&mut self) -> TUNE_CLK_STOP_EN_W<16> {
        TUNE_CLK_STOP_EN_W::new(self)
    }
    #[doc = "Bits 17:18 - Maximum Latency specification between cclk_tx and cclk_rx. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    #[inline(always)]
    #[must_use]
    pub fn pre_change_dly(&mut self) -> PRE_CHANGE_DLY_W<17> {
        PRE_CHANGE_DLY_W::new(self)
    }
    #[doc = "Bits 19:20 - Time taken for phase switching and stable clock output. Specifies the maximum time (in terms of cclk cycles) that the delay line can take to switch its output phase after a change in tuning_cclk_sel or autotuning_cclk_sel. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    #[inline(always)]
    #[must_use]
    pub fn post_change_dly(&mut self) -> POST_CHANGE_DLY_W<19> {
        POST_CHANGE_DLY_W::new(self)
    }
    #[doc = "Bits 24:30 - Sampling window threshold value setting The maximum value that can be set here depends on the length of delayline used for tuning. A delayLine with 32 taps can use values from 0x0 to 0x1F. This field is valid only when SWIN_TH_EN is '1'. Should be programmed only when SAMPLE_CLK_SEL is '0' 0x0 : Threshold values is 0x1, windows of length 1 tap and above can be selected as sampling window. 0x1 : Threshold values is 0x2, windows of length 2 taps and above can be selected as sampling window. 0x2 : Threshold values is 0x1, windows of length 3 taps and above can be selected as sampling window. ........ 0x1F : Threshold values is 0x1, windows of length 32 taps and above can be selected as sampling window."]
    #[inline(always)]
    #[must_use]
    pub fn swin_th_val(&mut self) -> SWIN_TH_VAL_W<24> {
        SWIN_TH_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auto_tuning_ctrl](index.html) module"]
pub struct AUTO_TUNING_CTRL_SPEC;
impl crate::RegisterSpec for AUTO_TUNING_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auto_tuning_ctrl::R](R) reader structure"]
impl crate::Readable for AUTO_TUNING_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auto_tuning_ctrl::W](W) writer structure"]
impl crate::Writable for AUTO_TUNING_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTO_TUNING_CTRL to value 0"]
impl crate::Resettable for AUTO_TUNING_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
