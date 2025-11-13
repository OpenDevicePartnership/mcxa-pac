#[doc = "Register `CRCR` reader"]
pub type R = crate::R<CrcrSpec>;
#[doc = "Field `TXCRC` reader - Transmitted CRC value"]
pub type TxcrcR = crate::FieldReader<u16>;
#[doc = "Field `MBCRC` reader - CRC Message Buffer"]
pub type MbcrcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:14 - Transmitted CRC value"]
    #[inline(always)]
    pub fn txcrc(&self) -> TxcrcR {
        TxcrcR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - CRC Message Buffer"]
    #[inline(always)]
    pub fn mbcrc(&self) -> MbcrcR {
        MbcrcR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCR")
            .field("txcrc", &self.txcrc())
            .field("mbcrc", &self.mbcrc())
            .finish()
    }
}
#[doc = "Cyclic Redundancy Check\n\nYou can [`read`](crate::Reg::read) this register and get [`crcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcrSpec;
impl crate::RegisterSpec for CrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcr::R`](R) reader structure"]
impl crate::Readable for CrcrSpec {}
#[doc = "`reset()` method sets CRCR to value 0"]
impl crate::Resettable for CrcrSpec {}
