use rustc_abi::Endian;

use crate::spec::{
    CodeModel, PanicStrategy, RustcAbi, Target, TargetMetadata, TargetOptions, base,
};

pub(crate) fn target() -> Target {
    let mut base: TargetOptions = base::nanvix::opts();
    base.cpu = "pentium4".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.endian = Endian::Little;
    base.c_int_width = "32".into();
    base.max_atomic_width = Some(64);
    base.code_model = Some(CodeModel::Small);
    base.features = "-mmx,-avx,-avx2,-sse2,-sse,+soft-float".into();
    base.rustc_abi = Some(RustcAbi::X86Softfloat);
    base.has_thread_local = false;

    Target {
        llvm_target: "i686-unknown-none".into(),
        pointer_width: 32,
        data_layout:
            "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-i128:128-f64:32:64-f80:32-n8:16:32-S128"
                .into(),
        arch: "x86".into(),
        options: base,
        metadata: TargetMetadata {
            description: Some("32-bit Nanvix".into()),
            tier: None,
            host_tools: Some(false),
            std: Some(true),
        },
    }
}
