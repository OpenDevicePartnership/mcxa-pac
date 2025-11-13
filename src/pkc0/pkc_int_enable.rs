#[doc = "Register `PKC_INT_ENABLE` reader"]
pub type R = crate::R<PkcIntEnableSpec>;
#[doc = "Field `EN_PDONE` reader - PDONE interrupt enable flag"]
pub type EnPdoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PDONE interrupt enable flag"]
    #[inline(always)]
    pub fn en_pdone(&self) -> EnPdoneR {
        EnPdoneR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKC_INT_ENABLE")
            .field("en_pdone", &self.en_pdone())
            .finish()
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pkc_int_enable::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkcIntEnableSpec;
impl crate::RegisterSpec for PkcIntEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkc_int_enable::R`](R) reader structure"]
impl crate::Readable for PkcIntEnableSpec {}
#[doc = "`reset()` method sets PKC_INT_ENABLE to value 0"]
impl crate::Resettable for PkcIntEnableSpec {}
