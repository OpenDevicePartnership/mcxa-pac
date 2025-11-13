#[doc = "Register `UPOSH1` reader"]
pub type R = crate::R<Uposh1Spec>;
#[doc = "Field `UPOSH1` reader - UPOSH1"]
pub type Uposh1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UPOSH1"]
    #[inline(always)]
    pub fn uposh1(&self) -> Uposh1R {
        Uposh1R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPOSH1").field("uposh1", &self.uposh1()).finish()
    }
}
#[doc = "Upper Position Holder Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uposh1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uposh1Spec;
impl crate::RegisterSpec for Uposh1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uposh1::R`](R) reader structure"]
impl crate::Readable for Uposh1Spec {}
#[doc = "`reset()` method sets UPOSH1 to value 0"]
impl crate::Resettable for Uposh1Spec {}
