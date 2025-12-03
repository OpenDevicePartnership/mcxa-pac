#[doc = "Register `GPOLY16` reader"]
pub type R = crate::R<Gpoly16Spec>;
#[doc = "Register `GPOLY16` writer"]
pub type W = crate::W<Gpoly16Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Polynomial, 16-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`gpoly16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoly16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpoly16Spec;
impl crate::RegisterSpec for Gpoly16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`gpoly16::R`](R) reader structure"]
impl crate::Readable for Gpoly16Spec {}
#[doc = "`write(|w| ..)` method takes [`gpoly16::W`](W) writer structure"]
impl crate::Writable for Gpoly16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPOLY16 to value 0x1021"]
impl crate::Resettable for Gpoly16Spec {
    const RESET_VALUE: u16 = 0x1021;
}
