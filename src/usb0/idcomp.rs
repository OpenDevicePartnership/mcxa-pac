#[doc = "Register `IDCOMP` reader"]
pub type R = crate::R<IdcompSpec>;
#[doc = "Field `NID` reader - Negative Peripheral ID"]
pub type NidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Negative Peripheral ID"]
    #[inline(always)]
    pub fn nid(&self) -> NidR {
        NidR::new(self.bits & 0x3f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDCOMP").field("nid", &self.nid()).finish()
    }
}
#[doc = "Peripheral ID Complement\n\nYou can [`read`](crate::Reg::read) this register and get [`idcomp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcompSpec;
impl crate::RegisterSpec for IdcompSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`idcomp::R`](R) reader structure"]
impl crate::Readable for IdcompSpec {}
#[doc = "`reset()` method sets IDCOMP to value 0xfb"]
impl crate::Resettable for IdcompSpec {
    const RESET_VALUE: u8 = 0xfb;
}
