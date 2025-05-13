use crate::spec::{Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {

    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &[
            "-nostdlib",
			"-Wl,--entry=_do_start",
			"-Wl,-melf_i386",
        ],
    );

    TargetOptions {
        os: "nanvix".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        pre_link_args,
        ..Default::default()
    }
}
