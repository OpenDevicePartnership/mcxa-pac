#[doc = "Register `MWDATAH1` writer"]
pub type W = crate::W<Mwdatah1Spec>;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Mwdatah1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, Mwdatah1Spec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Controller Write Halfword Data (to Bus)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatah1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mwdatah1Spec;
impl crate::RegisterSpec for Mwdatah1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwdatah1::W`](W) writer structure"]
impl crate::Writable for Mwdatah1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MWDATAH1 to value 0"]
impl crate::Resettable for Mwdatah1Spec {}
