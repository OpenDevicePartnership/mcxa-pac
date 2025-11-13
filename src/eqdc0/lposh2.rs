#[doc = "Register `LPOSH2` reader"]
pub type R = crate::R<Lposh2Spec>;
#[doc = "Field `LPOSH2` reader - LPOSH2"]
pub type Lposh2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LPOSH2"]
    #[inline(always)]
    pub fn lposh2(&self) -> Lposh2R {
        Lposh2R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPOSH2").field("lposh2", &self.lposh2()).finish()
    }
}
#[doc = "Lower Position Holder Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`lposh2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lposh2Spec;
impl crate::RegisterSpec for Lposh2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lposh2::R`](R) reader structure"]
impl crate::Readable for Lposh2Spec {}
#[doc = "`reset()` method sets LPOSH2 to value 0"]
impl crate::Resettable for Lposh2Spec {}
