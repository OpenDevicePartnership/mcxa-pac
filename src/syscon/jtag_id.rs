#[doc = "Register `JTAG_ID` reader"]
pub type R = crate::R<JtagIdSpec>;
#[doc = "Field `JTAG_ID` reader - Indicates the device ID"]
pub type JtagIdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the device ID"]
    #[inline(always)]
    pub fn jtag_id(&self) -> JtagIdR {
        JtagIdR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG_ID").field("jtag_id", &self.jtag_id()).finish()
    }
}
#[doc = "JTAG Chip ID\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagIdSpec;
impl crate::RegisterSpec for JtagIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag_id::R`](R) reader structure"]
impl crate::Readable for JtagIdSpec {}
#[doc = "`reset()` method sets JTAG_ID to value 0x0726_802b"]
impl crate::Resettable for JtagIdSpec {
    const RESET_VALUE: u32 = 0x0726_802b;
}
