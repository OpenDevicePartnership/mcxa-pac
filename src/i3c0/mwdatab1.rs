#[doc = "Register `MWDATAB1` writer"]
pub type W = crate::W<Mwdatab1Spec>;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Mwdatab1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, Mwdatab1Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Controller Write Byte Data 1 (to Bus)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatab1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mwdatab1Spec;
impl crate::RegisterSpec for Mwdatab1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwdatab1::W`](W) writer structure"]
impl crate::Writable for Mwdatab1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MWDATAB1 to value 0"]
impl crate::Resettable for Mwdatab1Spec {}
