#[doc = "Register `UPOSH2` reader"]
pub type R = crate::R<Uposh2Spec>;
#[doc = "Field `UPOSH2` reader - UPOSH2"]
pub type Uposh2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UPOSH2"]
    #[inline(always)]
    pub fn uposh2(&self) -> Uposh2R {
        Uposh2R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPOSH2").field("uposh2", &self.uposh2()).finish()
    }
}
#[doc = "Upper Position Holder Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`uposh2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uposh2Spec;
impl crate::RegisterSpec for Uposh2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uposh2::R`](R) reader structure"]
impl crate::Readable for Uposh2Spec {}
#[doc = "`reset()` method sets UPOSH2 to value 0"]
impl crate::Resettable for Uposh2Spec {}
