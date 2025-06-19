use crate::spec::{Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions, cvs};

pub(crate) fn opts() -> TargetOptions {
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &["-nostdlib", "-Wl,--entry=_do_start", "-Wl,-melf_i386"],
    );

    TargetOptions {
        os: "nanvix".into(),
        exe_suffix: ".elf".into(),
        families: cvs!["unix"],
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        panic_strategy: crate::spec::PanicStrategy::Abort,
        pre_link_args,
        ..Default::default()
    }
}
