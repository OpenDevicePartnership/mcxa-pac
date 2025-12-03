#[doc = "Register `DATA8` reader"]
pub type R = crate::R<Data8Spec>;
#[doc = "Register `DATA8` writer"]
pub type W = crate::W<Data8Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CRC data register, 8-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`data8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data8Spec;
impl crate::RegisterSpec for Data8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`data8::R`](R) reader structure"]
impl crate::Readable for Data8Spec {}
#[doc = "`write(|w| ..)` method takes [`data8::W`](W) writer structure"]
impl crate::Writable for Data8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA8 to value 0xff"]
impl crate::Resettable for Data8Spec {
    const RESET_VALUE: u8 = 0xff;
}
