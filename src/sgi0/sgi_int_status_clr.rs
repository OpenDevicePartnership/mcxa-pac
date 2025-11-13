#[doc = "Register `sgi_int_status_clr` reader"]
pub type R = crate::R<SgiIntStatusClrSpec>;
#[doc = "Register `sgi_int_status_clr` writer"]
pub type W = crate::W<SgiIntStatusClrSpec>;
#[doc = "Field `int_clr` reader - Write to clear interrupt status flag (SGI_INT_STATUS.INT_PDONE=0)."]
pub type IntClrR = crate::BitReader;
#[doc = "Field `int_clr` writer - Write to clear interrupt status flag (SGI_INT_STATUS.INT_PDONE=0)."]
pub type IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `int_stsc_rsvd` reader - reserved"]
pub type IntStscRsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Write to clear interrupt status flag (SGI_INT_STATUS.INT_PDONE=0)."]
    #[inline(always)]
    pub fn int_clr(&self) -> IntClrR {
        IntClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - reserved"]
    #[inline(always)]
    pub fn int_stsc_rsvd(&self) -> IntStscRsvdR {
        IntStscRsvdR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("sgi_int_status_clr")
            .field("int_clr", &self.int_clr())
            .field("int_stsc_rsvd", &self.int_stsc_rsvd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write to clear interrupt status flag (SGI_INT_STATUS.INT_PDONE=0)."]
    #[inline(always)]
    pub fn int_clr(&mut self) -> IntClrW<'_, SgiIntStatusClrSpec> {
        IntClrW::new(self, 0)
    }
}
#[doc = "Interrupt status clear\n\nYou can [`read`](crate::Reg::read) this register and get [`sgi_int_status_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgi_int_status_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SgiIntStatusClrSpec;
impl crate::RegisterSpec for SgiIntStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgi_int_status_clr::R`](R) reader structure"]
impl crate::Readable for SgiIntStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`sgi_int_status_clr::W`](W) writer structure"]
impl crate::Writable for SgiIntStatusClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets sgi_int_status_clr to value 0"]
impl crate::Resettable for SgiIntStatusClrSpec {}
