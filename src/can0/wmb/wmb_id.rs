#[doc = "Register `WMB_ID` reader"]
pub type R = crate::R<WmbIdSpec>;
#[doc = "Field `ID` reader - Received ID in Pretended Networking Mode"]
pub type IdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - Received ID in Pretended Networking Mode"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WMB_ID").field("id", &self.id()).finish()
    }
}
#[doc = "Wake-Up Message Buffer for ID\n\nYou can [`read`](crate::Reg::read) this register and get [`wmb_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WmbIdSpec;
impl crate::RegisterSpec for WmbIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wmb_id::R`](R) reader structure"]
impl crate::Readable for WmbIdSpec {}
#[doc = "`reset()` method sets WMB_ID to value 0"]
impl crate::Resettable for WmbIdSpec {}
