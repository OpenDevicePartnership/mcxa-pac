#[doc = "Register `RDROR` reader"]
pub type R = crate::R<RdrorSpec>;
#[doc = "Field `DATA` reader - Receive Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDROR").field("data", &self.data()).finish()
    }
}
#[doc = "Receive Data Read Only\n\nYou can [`read`](crate::Reg::read) this register and get [`rdror::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrorSpec;
impl crate::RegisterSpec for RdrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdror::R`](R) reader structure"]
impl crate::Readable for RdrorSpec {}
#[doc = "`reset()` method sets RDROR to value 0"]
impl crate::Resettable for RdrorSpec {}
