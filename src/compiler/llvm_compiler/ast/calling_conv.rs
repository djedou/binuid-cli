use crate::compiler::{llvm_compiler::Rule, BuildFrom};


#[derive(Debug)]
pub enum CallingConv {
    None,
    AmdgpuCs,
    AmdgpuEs,
    AmdgpuGs,
    AmdgpuHs,
    AmdgpuKernel,
    AmdgpuLs,
    AmdgpuPs,
    AmdgpuVs,
    AnyregCc,
    ArmAapcsVfpcc,
    ArmAapcscc,
    ArmApcscc,
    AvrIntrcc,
    AvrSignalcc,
    Coldcc,
    CxxFastTlscc,
    FastCc,
    GhCcc,
    HhvmCcc,
    HhvmCc,
    IntelOclBicc,
    Msp430Intrcc,
    PreserveAllcc,
    PreserveMostcc,
    PtxDevice,
    PtxKernel,
    SpirFunc,
    SpirKernel,
    Swiftcc,
    WebkitJscc,
    Win64cc,
    X8664Sysvcc,
    X86Fastcallcc,
    X86Intrcc,
    X86Regcallcc,
    X86Stdcallcc,
    X86Thiscallcc,
    X86Vectorcallcc,
    Ccc,
	Cc {
        int: u32
    },
}


impl BuildFrom for CallingConv {
    fn build_from(pair: &pest::iterators::Pair<Rule>) -> CallingConv {
        match pair.clone().into_inner().next() {
            Some(inner_pair) => {
                match inner_pair.as_rule() {
                    Rule::AmdgpuCs => CallingConv::AmdgpuCs,
                    Rule::AmdgpuEs => CallingConv::AmdgpuEs,
                    Rule::AmdgpuGs => CallingConv::AmdgpuGs,
                    Rule::AmdgpuHs => CallingConv::AmdgpuHs,
                    Rule::AmdgpuKernel => CallingConv::AmdgpuKernel,
                    Rule::AmdgpuLs => CallingConv::AmdgpuLs,
                    Rule::AmdgpuPs => CallingConv::AmdgpuPs,
                    Rule::AmdgpuVs => CallingConv::AmdgpuVs,
                    Rule::AnyregCc => CallingConv::AnyregCc,
                    Rule::ArmAapcsVfpcc => CallingConv::ArmAapcsVfpcc,
                    Rule::ArmAapcscc => CallingConv::ArmAapcscc,
                    Rule::ArmApcscc => CallingConv::ArmApcscc,
                    Rule::AvrIntrcc => CallingConv::AvrIntrcc,
                    Rule::AvrSignalcc => CallingConv::AvrSignalcc,
                    Rule::Coldcc => CallingConv::Coldcc,
                    Rule::CxxFastTlscc => CallingConv::CxxFastTlscc,
                    Rule::FastCc => CallingConv::FastCc,
                    Rule::GhCcc => CallingConv::GhCcc,
                    Rule::HhvmCcc => CallingConv::HhvmCcc,
                    Rule::HhvmCc => CallingConv::HhvmCc,
                    Rule::IntelOclBicc => CallingConv::IntelOclBicc,
                    Rule::Msp430Intrcc => CallingConv::Msp430Intrcc,
                    Rule::PreserveAllcc => CallingConv::PreserveAllcc,
                    Rule::PreserveMostcc => CallingConv::PreserveMostcc,
                    Rule::PtxDevice => CallingConv::PtxDevice,
                    Rule::PtxKernel => CallingConv::PtxKernel,
                    Rule::SpirFunc => CallingConv::SpirFunc,
                    Rule::SpirKernel => CallingConv::SpirKernel,
                    Rule::Swiftcc => CallingConv::Swiftcc,
                    Rule::WebkitJscc => CallingConv::WebkitJscc,
                    Rule::Win64cc => CallingConv::Win64cc,
                    Rule::X8664Sysvcc => CallingConv::X8664Sysvcc,
                    Rule::X86Fastcallcc => CallingConv::X86Fastcallcc,
                    Rule::X86Intrcc => CallingConv::X86Intrcc,
                    Rule::X86Regcallcc => CallingConv::X86Regcallcc,
                    Rule::X86Stdcallcc => CallingConv::X86Stdcallcc,
                    Rule::X86Thiscallcc => CallingConv::X86Thiscallcc,
                    Rule::X86Vectorcallcc => CallingConv::X86Vectorcallcc,
                    Rule::Ccc => CallingConv::Ccc,
                    Rule::CcIntLit => {
                        let mut int = 0;
                        for p in inner_pair.clone().into_inner() {
                            match p.as_rule() {
                                Rule::IntLit => {
                                    int = p.as_str().parse::<u32>().map_or(0, |d| d.clone());
                                },
                                _ => {}
                            }
                        }
                        CallingConv::Cc {
                            int: int
                        }
                    }, 
                    _ => CallingConv::None
                }
            },
            None => CallingConv::None
        }
    }
}