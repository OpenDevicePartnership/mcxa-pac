#[doc = "Register `DATA16` reader"]
pub type R = crate::R<Data16Spec>;
#[doc = "Register `DATA16` writer"]
pub type W = crate::W<Data16Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC data register, 16-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`data16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data16Spec;
impl crate::RegisterSpec for Data16Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data16::R`](R) reader structure"]
impl crate::Readable for Data16Spec {}
#[doc = "`write(|w| ..)` method takes [`data16::W`](W) writer structure"]
impl crate::Writable for Data16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA16 to value 0xffff"]
impl crate::Resettable for Data16Spec {
    const RESET_VALUE: u16 = 0xffff;
}
