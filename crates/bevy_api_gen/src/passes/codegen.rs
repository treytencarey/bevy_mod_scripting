use crate::{configure_tera, Args, BevyCtxt, TemplateKind};

use log::info;
use rustc_hir::def_id::LOCAL_CRATE;
use std::{
    fs::{self, File},
    io::Write,
};
use tera::Context;

/// generates a module with the appropriate wrappers for all the found reflection ADT's in the crate
pub(crate) fn codegen(ctxt: &mut BevyCtxt<'_>, args: &Args) -> bool {
    let (output, templates) = match &args.cmd {
        crate::Command::Generate {
            output, templates, ..
        } => (output, templates),
        _ => return true,
    };

    let tera = configure_tera(ctxt.tcx.crate_name(LOCAL_CRATE).as_str(), templates);

    // perform code gen using templates
    fs::create_dir_all(output).unwrap();
    info!("Writing code files to : {}", output);

    let template_data = ctxt.template_context.as_ref().unwrap();
    let mut context = Context::from_serialize(template_data).unwrap();
    crate::extend_context_with_args(args.template_args.as_deref(), &mut context);
    // generate crate artifact
    let mut file = File::create(output.join(format!("{}.rs", template_data.crate_name))).unwrap();

    tera.render_to(
        &TemplateKind::CrateArtifact.to_string(),
        &context,
        &mut file,
    )
    .expect("Failed to render crate artifact");

    file.flush().unwrap();

    true
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use strum::VariantNames;

    use crate::TEMPLATE_DIR;

    use super::*;

    #[test]
    fn test_templates_exist() {
        let template_files: HashSet<&str> = TEMPLATE_DIR
            .files()
            .map(|file| file.path().file_name().unwrap().to_str().unwrap())
            .collect();

        TemplateKind::VARIANTS.iter().for_each(|f| {
            assert!(
                template_files.contains(f),
                "Template file not in variants: {}",
                f
            );
        });
    }
}
