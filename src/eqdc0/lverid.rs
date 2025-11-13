#[doc = "Register `LVERID` reader"]
pub type R = crate::R<LveridSpec>;
#[doc = "Field `LVERID` reader - LVERID"]
pub type LveridR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LVERID"]
    #[inline(always)]
    pub fn lverid(&self) -> LveridR {
        LveridR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LVERID").field("lverid", &self.lverid()).finish()
    }
}
#[doc = "Lower VERID\n\nYou can [`read`](crate::Reg::read) this register and get [`lverid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LveridSpec;
impl crate::RegisterSpec for LveridSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lverid::R`](R) reader structure"]
impl crate::Readable for LveridSpec {}
#[doc = "`reset()` method sets LVERID to value 0x01"]
impl crate::Resettable for LveridSpec {
    const RESET_VALUE: u16 = 0x01;
}
