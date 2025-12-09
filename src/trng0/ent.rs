#[doc = "Register `ENT%s` reader"]
pub type R = crate::R<EntSpec>;
#[doc = "Register `ENT%s` writer"]
pub type W = crate::W<EntSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Entropy Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ent::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ent::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntSpec;
impl crate::RegisterSpec for EntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ent::R`](R) reader structure"]
impl crate::Readable for EntSpec {}
#[doc = "`write(|w| ..)` method takes [`ent::W`](W) writer structure"]
impl crate::Writable for EntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENT%s to value 0"]
impl crate::Resettable for EntSpec {}
