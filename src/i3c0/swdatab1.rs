#[doc = "Register `SWDATAB1` writer"]
pub type W = crate::W<Swdatab1Spec>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Swdatab1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Swdatab1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Target Write Data Byte\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdatab1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swdatab1Spec;
impl crate::RegisterSpec for Swdatab1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swdatab1::W`](W) writer structure"]
impl crate::Writable for Swdatab1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWDATAB1 to value 0"]
impl crate::Resettable for Swdatab1Spec {}
