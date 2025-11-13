#[doc = "Register `EVTIMERH` reader"]
pub type R = crate::R<EvtimerhSpec>;
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - EVTimer Count Value"]
pub type EvtimerCountValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - EVTimer Count Value"]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EvtimerCountValueR {
        EvtimerCountValueR::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTIMERH")
            .field("evtimer_count_value", &self.evtimer_count_value())
            .finish()
    }
}
#[doc = "EVTIMER High\n\nYou can [`read`](crate::Reg::read) this register and get [`evtimerh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtimerhSpec;
impl crate::RegisterSpec for EvtimerhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtimerh::R`](R) reader structure"]
impl crate::Readable for EvtimerhSpec {}
#[doc = "`reset()` method sets EVTIMERH to value 0"]
impl crate::Resettable for EvtimerhSpec {}
