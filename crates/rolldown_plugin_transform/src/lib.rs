mod types;
mod utils;

use std::borrow::Cow;
use std::path::{Path, PathBuf};

use arcstr::ArcStr;
use itertools::Itertools;
use oxc::codegen::{Codegen, CodegenOptions, CodegenReturn};
use oxc::parser::Parser;
use oxc::semantic::SemanticBuilder;
use oxc::transformer::Transformer;
use rolldown_common::ModuleType;
use rolldown_error::{BuildDiagnostic, Severity};
use rolldown_plugin::{HookUsage, Plugin, SharedTransformPluginContext};
use rolldown_utils::{pattern_filter::StringOrRegex, stabilize_id::stabilize_id, url::clean_url};
use swc_common::Mark;
use swc_common::comments::NoopComments;
use swc_ecma_codegen::Emitter;
use swc_ecma_codegen::text_writer::JsWriter;
use swc_ecma_transforms_compat::es2015;
use swc_ecma_transforms_react::{Options, jsx, react};
use swc_ecma_transforms_typescript::strip;
pub use types::{
  CompilerAssumptions, DecoratorOptions, IsolatedDeclarationsOptions, JsxOptions,
  ReactRefreshOptions, TransformOptions, TypeScriptOptions,
};

// swc
use swc_common::{FileName, SourceMap, sync::Lrc};
use swc_ecma_ast::{EsVersion, Pass, Program};
use swc_ecma_parser::{Lexer, Parser as SwcParser, StringInput, Syntax, TsSyntax};
use swc_ecma_transforms_base::{self, resolver};
use swc_ecma_visit::{FoldPass, FoldWith, visit_mut_pass};

#[derive(Debug, Default)]
pub struct TransformPlugin {
  pub include: Vec<StringOrRegex>,
  pub exclude: Vec<StringOrRegex>,
  pub jsx_refresh_include: Vec<StringOrRegex>,
  pub jsx_refresh_exclude: Vec<StringOrRegex>,
  pub jsx_inject: Option<String>,
  pub is_server_consumer: bool,
  pub sourcemap: bool,
  pub transform_options: TransformOptions,
}

/// only handle ecma like syntax, `jsx`,`tsx`,`ts`
impl Plugin for TransformPlugin {
  fn name(&self) -> Cow<'static, str> {
    Cow::Borrowed("builtin:transform")
  }

  async fn transform(
    &self,
    ctx: SharedTransformPluginContext,
    args: &rolldown_plugin::HookTransformArgs<'_>,
  ) -> rolldown_plugin::HookTransformReturn {
    // swc
    let cm: Lrc<SourceMap> = Default::default();
    let file_name = Lrc::new(FileName::Real(PathBuf::from(args.id)));
    let fm = cm.new_source_file(file_name, args.code.clone());
    let input = StringInput::from(&*fm);

    let syntax = Syntax::Typescript(TsSyntax { tsx: true, ..Default::default() });
    let lexer = Lexer::new(syntax, EsVersion::Es5, input, None);

    let mut swc_parser = SwcParser::new_from(lexer);
    let module =
      swc_parser.parse_module().map_err(|e| anyhow::anyhow!("Failed to parse module"))?;

    let unresolved_mark = Mark::fresh(Mark::root());
    let top_level_mark = Mark::fresh(Mark::root());

    let mut pass = (
      visit_mut_pass(resolver(top_level_mark, Mark::fresh(Mark::root()), false)),
      strip(unresolved_mark, top_level_mark),
      jsx(cm.clone(), None::<NoopComments>, Default::default(), top_level_mark, unresolved_mark),
      react(cm.clone(), None::<NoopComments>, Default::default(), top_level_mark, unresolved_mark),
      es2015(unresolved_mark, None::<NoopComments>, Default::default()),
    );

    let program = Program::Module(module).apply(&mut pass);

    let mut buf = vec![];
    let mut emitter = Emitter {
      cfg: Default::default(),
      cm: cm.clone(),
      comments: None,
      wr: JsWriter::new(cm.clone(), "\n", &mut buf, None),
    };

    match &program {
      Program::Module(m) => emitter.emit_module(m)?,
      Program::Script(s) => emitter.emit_script(s)?,
    };

    let code = String::from_utf8(buf)?;

    // let cwd = ctx.inner.cwd().to_string_lossy();
    // let extension = Path::new(args.id).extension().map(|s| s.to_string_lossy());
    // let extension = extension.as_ref().map(|s| clean_url(s));
    // let module_type = extension.map(ModuleType::from_str_with_fallback);
    // if !self.filter(args.id, &cwd, &module_type) {
    //   return Ok(None);
    // }

    // let (source_type, transform_options) =
    //   self.get_modified_transform_options(&ctx, args.id, &cwd, extension)?;

    // let allocator = oxc::allocator::Allocator::default();
    // let ret = Parser::new(&allocator, args.code, source_type).parse();
    // if ret.panicked || !ret.errors.is_empty() {
    //   let errors = BuildDiagnostic::from_oxc_diagnostics(
    //     ret.errors,
    //     &ArcStr::from(args.code.as_str()),
    //     &stabilize_id(args.id, ctx.inner.cwd()),
    //     &Severity::Error,
    //   )
    //   .iter()
    //   .map(|error| error.to_diagnostic().with_kind(self.name().into_owned()).to_color_string())
    //   .join("\n\n");
    //   Err(anyhow::anyhow!("\n{errors}"))?;
    // }

    // let mut program = ret.program;
    // let scoping: oxc::semantic::Scoping =
    //   SemanticBuilder::new().build(&program).semantic.into_scoping();
    // let transformer = Transformer::new(&allocator, Path::new(args.id), &transform_options);

    // let transformer_return = transformer.build_with_scoping(scoping, &mut program);
    // if !transformer_return.errors.is_empty() {
    //   let errors = BuildDiagnostic::from_oxc_diagnostics(
    //     transformer_return.errors,
    //     &ArcStr::from(args.code.as_str()),
    //     &stabilize_id(args.id, ctx.inner.cwd()),
    //     &Severity::Error,
    //   )
    //   .iter()
    //   .map(|error| error.to_diagnostic().with_kind(self.name().into_owned()).to_color_string())
    //   .join("\n\n");
    //   Err(anyhow::anyhow!("\n{errors}"))?;
    // }

    // let ret = Codegen::new()
    //   .with_options(CodegenOptions {
    //     comments: false,
    //     source_map_path: Some(args.id.into()),
    //     ..CodegenOptions::default()
    //   })
    //   .build(&program);
    // let CodegenReturn { mut code, map, .. } = ret;

    // if let Some(inject) = &self.jsx_inject {
    //   let mut new_code = String::with_capacity(inject.len() + 1 + code.len());
    //   new_code.push_str(inject);
    //   new_code.push(';');
    //   new_code.push_str(&code);
    //   code = new_code;
    // }

    Ok(Some(rolldown_plugin::HookTransformOutput {
      map: None,
      code: Some(code),
      module_type: Some(ModuleType::Js),
      ..Default::default()
    }))
  }

  fn register_hook_usage(&self) -> HookUsage {
    HookUsage::Transform
  }
}
