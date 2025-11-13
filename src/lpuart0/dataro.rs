#[doc = "Register `DATARO` reader"]
pub type R = crate::R<DataroSpec>;
#[doc = "Field `DATA` reader - Receive Data"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATARO").field("data", &self.data()).finish()
    }
}
#[doc = "Data Read-Only\n\nYou can [`read`](crate::Reg::read) this register and get [`dataro::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataroSpec;
impl crate::RegisterSpec for DataroSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataro::R`](R) reader structure"]
impl crate::Readable for DataroSpec {}
#[doc = "`reset()` method sets DATARO to value 0x1000"]
impl crate::Resettable for DataroSpec {
    const RESET_VALUE: u32 = 0x1000;
}
