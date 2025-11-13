#[doc = "Register `CTRL_W` writer"]
pub type W = crate::W<CtrlWSpec>;
#[doc = "Field `REF_SCALE` writer - Reference Clock Scaling Factor"]
pub type RefScaleW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Pulse Width Measurement Mode Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PulseMode {
    #[doc = "0: Frequency Measurement mode"]
    FreqMeMode = 0,
    #[doc = "1: Pulse Width Measurement mode"]
    PulseMeMode = 1,
}
impl From<PulseMode> for bool {
    #[inline(always)]
    fn from(variant: PulseMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSE_MODE` writer - Pulse Width Measurement Mode Select"]
pub type PulseModeW<'a, REG> = crate::BitWriter<'a, REG, PulseMode>;
impl<'a, REG> PulseModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frequency Measurement mode"]
    #[inline(always)]
    pub fn freq_me_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PulseMode::FreqMeMode)
    }
    #[doc = "Pulse Width Measurement mode"]
    #[inline(always)]
    pub fn pulse_me_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PulseMode::PulseMeMode)
    }
}
#[doc = "Pulse Polarity\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PulsePol {
    #[doc = "0: High period"]
    HighPeriod = 0,
    #[doc = "1: Low period"]
    LowPeriod = 1,
}
impl From<PulsePol> for bool {
    #[inline(always)]
    fn from(variant: PulsePol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PULSE_POL` writer - Pulse Polarity"]
pub type PulsePolW<'a, REG> = crate::BitWriter<'a, REG, PulsePol>;
impl<'a, REG> PulsePolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High period"]
    #[inline(always)]
    pub fn high_period(self) -> &'a mut crate::W<REG> {
        self.variant(PulsePol::HighPeriod)
    }
    #[doc = "Low period"]
    #[inline(always)]
    pub fn low_period(self) -> &'a mut crate::W<REG> {
        self.variant(PulsePol::LowPeriod)
    }
}
#[doc = "Less Than Minimum Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LtMinIntEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<LtMinIntEn> for bool {
    #[inline(always)]
    fn from(variant: LtMinIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LT_MIN_INT_EN` writer - Less Than Minimum Interrupt Enable"]
pub type LtMinIntEnW<'a, REG> = crate::BitWriter<'a, REG, LtMinIntEn>;
impl<'a, REG> LtMinIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(LtMinIntEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(LtMinIntEn::Enable)
    }
}
#[doc = "Greater Than Maximum Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GtMaxIntEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<GtMaxIntEn> for bool {
    #[inline(always)]
    fn from(variant: GtMaxIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GT_MAX_INT_EN` writer - Greater Than Maximum Interrupt Enable"]
pub type GtMaxIntEnW<'a, REG> = crate::BitWriter<'a, REG, GtMaxIntEn>;
impl<'a, REG> GtMaxIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GtMaxIntEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GtMaxIntEn::Enable)
    }
}
#[doc = "Result Ready Interrupt Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultReadyIntEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<ResultReadyIntEn> for bool {
    #[inline(always)]
    fn from(variant: ResultReadyIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULT_READY_INT_EN` writer - Result Ready Interrupt Enable"]
pub type ResultReadyIntEnW<'a, REG> = crate::BitWriter<'a, REG, ResultReadyIntEn>;
impl<'a, REG> ResultReadyIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ResultReadyIntEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ResultReadyIntEn::Enable)
    }
}
#[doc = "Continuous Mode Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuousModeEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<ContinuousModeEn> for bool {
    #[inline(always)]
    fn from(variant: ContinuousModeEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTINUOUS_MODE_EN` writer - Continuous Mode Enable"]
pub type ContinuousModeEnW<'a, REG> = crate::BitWriter<'a, REG, ContinuousModeEn>;
impl<'a, REG> ContinuousModeEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ContinuousModeEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ContinuousModeEn::Enable)
    }
}
#[doc = "Measurement In Progress\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeasureInProgress {
    #[doc = "0: Terminates measurement"]
    ForceTerminate = 0,
    #[doc = "1: Initiates measurement"]
    InitiateAFreqmeCycle = 1,
}
impl From<MeasureInProgress> for bool {
    #[inline(always)]
    fn from(variant: MeasureInProgress) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEASURE_IN_PROGRESS` writer - Measurement In Progress"]
pub type MeasureInProgressW<'a, REG> = crate::BitWriter<'a, REG, MeasureInProgress>;
impl<'a, REG> MeasureInProgressW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Terminates measurement"]
    #[inline(always)]
    pub fn force_terminate(self) -> &'a mut crate::W<REG> {
        self.variant(MeasureInProgress::ForceTerminate)
    }
    #[doc = "Initiates measurement"]
    #[inline(always)]
    pub fn initiate_a_freqme_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(MeasureInProgress::InitiateAFreqmeCycle)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<CtrlWSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:4 - Reference Clock Scaling Factor"]
    #[inline(always)]
    pub fn ref_scale(&mut self) -> RefScaleW<'_, CtrlWSpec> {
        RefScaleW::new(self, 0)
    }
    #[doc = "Bit 8 - Pulse Width Measurement Mode Select"]
    #[inline(always)]
    pub fn pulse_mode(&mut self) -> PulseModeW<'_, CtrlWSpec> {
        PulseModeW::new(self, 8)
    }
    #[doc = "Bit 9 - Pulse Polarity"]
    #[inline(always)]
    pub fn pulse_pol(&mut self) -> PulsePolW<'_, CtrlWSpec> {
        PulsePolW::new(self, 9)
    }
    #[doc = "Bit 12 - Less Than Minimum Interrupt Enable"]
    #[inline(always)]
    pub fn lt_min_int_en(&mut self) -> LtMinIntEnW<'_, CtrlWSpec> {
        LtMinIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Greater Than Maximum Interrupt Enable"]
    #[inline(always)]
    pub fn gt_max_int_en(&mut self) -> GtMaxIntEnW<'_, CtrlWSpec> {
        GtMaxIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Result Ready Interrupt Enable"]
    #[inline(always)]
    pub fn result_ready_int_en(&mut self) -> ResultReadyIntEnW<'_, CtrlWSpec> {
        ResultReadyIntEnW::new(self, 14)
    }
    #[doc = "Bit 30 - Continuous Mode Enable"]
    #[inline(always)]
    pub fn continuous_mode_en(&mut self) -> ContinuousModeEnW<'_, CtrlWSpec> {
        ContinuousModeEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Measurement In Progress"]
    #[inline(always)]
    pub fn measure_in_progress(&mut self) -> MeasureInProgressW<'_, CtrlWSpec> {
        MeasureInProgressW::new(self, 31)
    }
}
#[doc = "Control (in Write mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlWSpec;
impl crate::RegisterSpec for CtrlWSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ctrl_w::W`](W) writer structure"]
impl crate::Writable for CtrlWSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_W to value 0"]
impl crate::Resettable for CtrlWSpec {}
