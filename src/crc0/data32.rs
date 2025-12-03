#[doc = "Register `DATA32` reader"]
pub type R = crate::R<Data32Spec>;
#[doc = "Register `DATA32` writer"]
pub type W = crate::W<Data32Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC data register, 32-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`data32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data32Spec;
impl crate::RegisterSpec for Data32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data32::R`](R) reader structure"]
impl crate::Readable for Data32Spec {}
#[doc = "`write(|w| ..)` method takes [`data32::W`](W) writer structure"]
impl crate::Writable for Data32Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA32 to value 0xffff_ffff"]
impl crate::Resettable for Data32Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
