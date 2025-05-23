use rustc_abi::Endian;
use crate::spec::{PanicStrategy, Target, TargetMetadata, TargetOptions, base, CodeModel};

pub(crate) fn target() -> Target {
    let mut base: TargetOptions = base::nanvix::opts();
    base.cpu = "pentium4".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "-mmx,-avx,-sse,-sse2".into();
    base.endian = Endian::Little;
    base.c_int_width = "32".into();
    base.max_atomic_width = Some(64);
    base.code_model = Some(CodeModel::Small);

    Target {
        llvm_target: "i686-unknown-none".into(),
        pointer_width: 32,
        data_layout:
            "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-i128:128-f64:32:64-f80:32-n8:16:32-S128"
                .into(),
        arch: "x86".into(),
        options: base,
        metadata: TargetMetadata { description: None, tier: None, host_tools: None, std: None },
    }
}
