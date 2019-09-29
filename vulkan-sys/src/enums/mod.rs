pub mod accessflags;
pub mod androidsurfacecreateflagskhr;
pub mod attachmentdescriptionflags;
pub mod attachmentloadop;
pub mod attachmentstoreop;
pub mod blendfactor;
pub mod blendop;
pub mod bool32;
pub mod bordercolor;
pub mod buffercreateflags;
pub mod bufferusageflags;
pub mod bufferviewcreateflags;
pub mod colorcomponentflags;
pub mod colorspacekhr;
pub mod commandbufferlevel;
pub mod commandbufferresetflags;
pub mod commandbufferusageflags;
pub mod commandpoolcreateflags;
pub mod commandpoolresetflags;
pub mod commandpooltrimflags;
pub mod compareop;
pub mod componentswizzle;
pub mod compositealphaflagskhr;
pub mod cullmodeflags;
pub mod debugreporterrorext;
pub mod debugreportflagsext;
pub mod debugreportobjecttypeext;
pub mod debugutilsmessageseverityflagsext;
pub mod debugutilsmessagetypeflagsext;
pub mod debugutilsmessengercallbackdataflagsext;
pub mod debugutilsmessengercreateflags;
pub mod dependencyflags;
pub mod descriptorpoolcreateflags;
pub mod descriptorpoolresetflags;
pub mod descriptorsetlayoutcreateflags;
pub mod descriptortype;
pub mod devicecreateflags;
pub mod devicequeuecreateflags;
pub mod displaymodecreateflagskhr;
pub mod displayplanealphaflagskhr;
pub mod displaysurfacecreateflagskhr;
pub mod dynamicstate;
pub mod eventcreateflags;
pub mod externalmemoryhandletypeflags;
pub mod fencecreateflags;
pub mod filter;
pub mod format;
pub mod formatfeatureflags;
pub mod framebuffercreateflags;
pub mod frontface;
pub mod imageaspectflags;
pub mod imagecreateflags;
pub mod imagelayout;
pub mod imagetiling;
pub mod imagetype;
pub mod imageusageflags;
pub mod imageviewcreateflags;
pub mod imageviewtype;
pub mod indextype;
pub mod instancecreateflags;
pub mod internalallocationtype;
pub mod iossurfacecreateflagsmvk;
pub mod logicop;
pub mod macossurfacecreateflagsmvk;
pub mod memoryheapflags;
pub mod memorymapflags;
pub mod memorypropertyflags;
pub mod mirsurfacecreateflagskhr;
pub mod objecttype;
pub mod physicaldevicetype;
pub mod pipelinebindpoint;
pub mod pipelinecachecreateflags;
pub mod pipelinecacheheaderversion;
pub mod pipelinecolorblendstatecreateflags;
pub mod pipelinecreateflags;
pub mod pipelinedepthstencilstatecreateflags;
pub mod pipelinedynamicstatecreateflags;
pub mod pipelineinputassemblystatecreateflags;
pub mod pipelinelayoutcreateflags;
pub mod pipelinemultisamplestatecreateflags;
pub mod pipelinerasterizationstatecreateflags;
pub mod pipelineshaderstagecreateflags;
pub mod pipelinestageflags;
pub mod pipelinetesselationstatecreateflags;
pub mod pipelinevertexinputstatecreateflags;
pub mod pipelineviewportstatecreateflags;
pub mod polygonmode;
pub mod presentmodekhr;
pub mod primitivetopology;
pub mod querycontrolflags;
pub mod querypipelinestatisticsflags;
pub mod querypoolcreateflags;
pub mod queryresultflags;
pub mod querytype;
pub mod queueflags;
pub mod renderpasscreateflags;
pub mod result;
pub mod samplecountflags;
pub mod sampleraddressmode;
pub mod samplercreateflags;
pub mod samplermipmapmode;
pub mod semaphorecreateflags;
pub mod shadermodulecreateflags;
pub mod shaderstageflags;
pub mod sharingmode;
pub mod sparseimageformatflags;
pub mod sparsememorybindflags;
pub mod stencilfaceflags;
pub mod stencilop;
pub mod structuretype;
pub mod subpasscontents;
pub mod subpassdescriptionflags;
pub mod surfacetransformflagskhr;
pub mod swapchaincreateflagskhr;
pub mod systemallocationscope;
pub mod vertexinputrate;
pub mod waylandsurfacecreateflagskhr;
pub mod win32surfacecreateflagskhr;
pub mod xcbsurfacecreateflagskhr;
pub mod xlibsurfacecreateflagskhr;

pub mod amd;
pub mod ext;
pub mod nv;

pub mod prelude;

#[macro_export]
macro_rules! SetupVkFlags {
    ($flags: ident, $bits: ident) => {
        use std::cmp::Ordering;
        use std::cmp::PartialEq;
        use std::convert::From;
        use std::fmt;
        use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

        impl fmt::Debug for $bits {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}(0x{:08x?})", stringify!($bits), self.0)
            }
        }

        impl BitAnd for $bits {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self {
                $bits(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $bits {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = $bits(self.0 & rhs.0)
            }
        }

        impl BitOr for $bits {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self {
                $bits(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $bits {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = $bits(self.0 | rhs.0)
            }
        }

        impl BitXor for $bits {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self {
                $bits(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $bits {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = $bits(self.0 ^ rhs.0)
            }
        }

        impl PartialEq<u8> for $bits {
            fn eq(&self, rhs: &u8) -> bool {
                self.0 == *rhs as u32
            }
        }

        impl PartialEq<u16> for $bits {
            fn eq(&self, rhs: &u16) -> bool {
                self.0 == *rhs as u32
            }
        }

        impl PartialEq<u32> for $bits {
            fn eq(&self, rhs: &u32) -> bool {
                self.0 == *rhs
            }
        }

        impl PartialEq<i32> for $bits {
            fn eq(&self, rhs: &i32) -> bool {
                self.0 as i32 == *rhs
            }
        }

        impl PartialEq<u64> for $bits {
            fn eq(&self, rhs: &u64) -> bool {
                self.0 as u64 == *rhs
            }
        }

        impl PartialEq<$flags> for $bits {
            fn eq(&self, rhs: &$flags) -> bool {
                self.0 as u32 == *rhs as u32
            }
        }

        impl BitOr<$flags> for $bits {
            type Output = $bits;

            fn bitor(self, rhs: $flags) -> $bits {
                $bits(self.0 | rhs as u32)
            }
        }

        impl BitOr<$bits> for $flags {
            type Output = $bits;

            fn bitor(self, rhs: $bits) -> $bits {
                $bits(self as u32 | rhs.0)
            }
        }

        impl BitOr<$flags> for $flags {
            type Output = $bits;

            fn bitor(self, rhs: $flags) -> $bits {
                $bits(self as u32 | rhs as u32)
            }
        }

        impl BitOr<u32> for $bits {
            type Output = u32;

            fn bitor(self, rhs: u32) -> u32 {
                self.0 | rhs
            }
        }

        impl BitOrAssign<$flags> for $bits {
            fn bitor_assign(&mut self, rhs: $flags) {
                *self = $bits(self.0 | rhs as u32)
            }
        }

        impl BitAnd<$flags> for $bits {
            type Output = $bits;

            fn bitand(self, rhs: $flags) -> $bits {
                $bits(self.0 & rhs as u32)
            }
        }

        impl BitAnd<$bits> for $flags {
            type Output = $bits;

            fn bitand(self, rhs: $bits) -> $bits {
                $bits(self as u32 & rhs.0)
            }
        }

        impl BitAnd<$flags> for $flags {
            type Output = $bits;

            fn bitand(self, rhs: $flags) -> $bits {
                $bits(self as u32 & rhs as u32)
            }
        }

        impl BitAnd<u32> for $bits {
            type Output = u32;

            fn bitand(self, rhs: u32) -> u32 {
                self.0 & rhs
            }
        }

        impl BitAndAssign<$flags> for $bits {
            fn bitand_assign(&mut self, rhs: $flags) {
                *self = $bits(self.0 & rhs as u32)
            }
        }

        impl BitXor<$flags> for $bits {
            type Output = $bits;

            fn bitxor(self, rhs: $flags) -> $bits {
                $bits(self.0 ^ rhs as u32)
            }
        }

        impl BitXor<$bits> for $flags {
            type Output = $bits;

            fn bitxor(self, rhs: $bits) -> $bits {
                $bits(self as u32 ^ rhs.0)
            }
        }

        impl BitXor<$flags> for $flags {
            type Output = $bits;

            fn bitxor(self, rhs: $flags) -> $bits {
                $bits(self as u32 ^ rhs as u32)
            }
        }

        impl BitXor<u32> for $bits {
            type Output = u32;

            fn bitxor(self, rhs: u32) -> u32 {
                self.0 ^ rhs
            }
        }

        impl BitXorAssign<$flags> for $bits {
            fn bitxor_assign(&mut self, rhs: $flags) {
                *self = $bits(self.0 ^ rhs as u32)
            }
        }

        impl Into<u32> for $bits {
            fn into(self) -> u32 {
                self.0 as u32
            }
        }

        impl From<u32> for $bits {
            fn from(n: u32) -> $bits {
                $bits(n)
            }
        }

        impl Into<u32> for $flags {
            fn into(self) -> u32 {
                self as u32
            }
        }

        impl From<$flags> for $bits {
            fn from(flags: $flags) -> $bits {
                $bits(flags as u32)
            }
        }

        impl PartialOrd for $flags {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $flags {
            fn cmp(&self, other: &Self) -> Ordering {
                (*self as u32).cmp(&(*other as u32))
            }
        }

        impl PartialOrd for $bits {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $bits {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
    };
}
