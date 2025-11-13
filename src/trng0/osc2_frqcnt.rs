#[doc = "Register `OSC2_FRQCNT` reader"]
pub type R = crate::R<Osc2FrqcntSpec>;
#[doc = "Field `OSC2_FRQ_CT` reader - Frequency Count"]
pub type Osc2FrqCtR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count"]
    #[inline(always)]
    pub fn osc2_frq_ct(&self) -> Osc2FrqCtR {
        Osc2FrqCtR::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSC2_FRQCNT")
            .field("osc2_frq_ct", &self.osc2_frq_ct())
            .finish()
    }
}
#[doc = "Oscillator-2 Frequency Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`osc2_frqcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc2FrqcntSpec;
impl crate::RegisterSpec for Osc2FrqcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc2_frqcnt::R`](R) reader structure"]
impl crate::Readable for Osc2FrqcntSpec {}
#[doc = "`reset()` method sets OSC2_FRQCNT to value 0"]
impl crate::Resettable for Osc2FrqcntSpec {}
