#[doc = "Register `PARAM` reader"]
pub type R = crate::R<ParamSpec>;
#[doc = "Field `MTXFIFO` reader - Controller Transmit FIFO Size"]
pub type MtxfifoR = crate::FieldReader;
#[doc = "Field `MRXFIFO` reader - Controller Receive FIFO Size"]
pub type MrxfifoR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Controller Transmit FIFO Size"]
    #[inline(always)]
    pub fn mtxfifo(&self) -> MtxfifoR {
        MtxfifoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Controller Receive FIFO Size"]
    #[inline(always)]
    pub fn mrxfifo(&self) -> MrxfifoR {
        MrxfifoR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("mtxfifo", &self.mtxfifo())
            .field("mrxfifo", &self.mrxfifo())
            .finish()
    }
}
#[doc = "Parameter\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParamSpec;
impl crate::RegisterSpec for ParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for ParamSpec {}
#[doc = "`reset()` method sets PARAM to value 0x0202"]
impl crate::Resettable for ParamSpec {
    const RESET_VALUE: u32 = 0x0202;
}
