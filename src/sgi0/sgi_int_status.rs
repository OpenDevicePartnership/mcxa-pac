#[doc = "Register `sgi_int_status` reader"]
pub type R = crate::R<SgiIntStatusSpec>;
#[doc = "Field `int_pdone` reader - Interrupt status flag:"]
pub type IntPdoneR = crate::BitReader;
#[doc = "Field `intst_rsvd` reader - reserved"]
pub type IntstRsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Interrupt status flag:"]
    #[inline(always)]
    pub fn int_pdone(&self) -> IntPdoneR {
        IntPdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - reserved"]
    #[inline(always)]
    pub fn intst_rsvd(&self) -> IntstRsvdR {
        IntstRsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sgi_int_status")
            .field("int_pdone", &self.int_pdone())
            .field("intst_rsvd", &self.intst_rsvd())
            .finish()
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`sgi_int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgiIntStatusSpec;
impl crate::RegisterSpec for SgiIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgi_int_status::R`](R) reader structure"]
impl crate::Readable for SgiIntStatusSpec {}
#[doc = "`reset()` method sets sgi_int_status to value 0"]
impl crate::Resettable for SgiIntStatusSpec {}
