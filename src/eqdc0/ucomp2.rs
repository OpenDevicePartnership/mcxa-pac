#[doc = "Register `UCOMP2` writer"]
pub type W = crate::W<Ucomp2Spec>;
#[doc = "Field `UCOMP2` writer - UCOMP2"]
pub type Ucomp2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Ucomp2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - UCOMP2"]
    #[inline(always)]
    pub fn ucomp2(&mut self) -> Ucomp2W<'_, Ucomp2Spec> {
        Ucomp2W::new(self, 0)
    }
}
#[doc = "Upper Position Compare 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucomp2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucomp2Spec;
impl crate::RegisterSpec for Ucomp2Spec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`ucomp2::W`](W) writer structure"]
impl crate::Writable for Ucomp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCOMP2 to value 0x8000"]
impl crate::Resettable for Ucomp2Spec {
    const RESET_VALUE: u16 = 0x8000;
}
