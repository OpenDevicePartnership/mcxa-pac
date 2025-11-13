#[doc = "Register `LPOSH1` reader"]
pub type R = crate::R<Lposh1Spec>;
#[doc = "Field `LPOSH1` reader - LPOSH1"]
pub type Lposh1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LPOSH1"]
    #[inline(always)]
    pub fn lposh1(&self) -> Lposh1R {
        Lposh1R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPOSH1").field("lposh1", &self.lposh1()).finish()
    }
}
#[doc = "Lower Position Holder Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lposh1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lposh1Spec;
impl crate::RegisterSpec for Lposh1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lposh1::R`](R) reader structure"]
impl crate::Readable for Lposh1Spec {}
#[doc = "`reset()` method sets LPOSH1 to value 0"]
impl crate::Resettable for Lposh1Spec {}
