use std::{borrow::Cow, fmt::Write};

use rolldown_common::RUNTIME_MODULE_KEY;
use rolldown_plugin::{
  HookTransformOutput, HookTransformOutputMap, HookUsage, Plugin, SharedTransformPluginContext,
};
use rolldown_plugin_utils::to_string_literal;
use rolldown_utils::{
  pattern_filter::{StringOrRegex, filter},
  url::clean_url,
};

const ROLLIPOP_RUNTIME: &str = "__rollipop_runtime__";

#[derive(Debug)]
pub struct RollipopReactRefreshWrapperPluginOptions {
  pub cwd: String,
  pub include: Vec<StringOrRegex>,
  pub exclude: Vec<StringOrRegex>,
  pub jsx_import_source: Option<String>,
}

#[derive(Debug)]
pub struct RollipopReactRefreshWrapperPlugin {
  cwd: String,
  include: Vec<StringOrRegex>,
  exclude: Vec<StringOrRegex>,
}

impl RollipopReactRefreshWrapperPlugin {
  pub fn new(options: RollipopReactRefreshWrapperPluginOptions) -> Self {
    Self { cwd: options.cwd, include: options.include, exclude: options.exclude }
  }

  fn add_refresh_wrapper(&self, code: &str, id: &str) -> String {
    let escaped_id = to_string_literal(id);
    let mut new_code = code.to_string();
    write!(
      new_code,
      "\
\nif (import.meta.hot) {{
  if ({ROLLIPOP_RUNTIME} == null) throw new Error('Rollipop dev runtime is not initialized');
  import.meta.hot.accept((nextExports) => {{
    if (!nextExports) return;
    if ({ROLLIPOP_RUNTIME}.reactRefresh.isReactRefreshBoundary(nextExports)) {{
      {ROLLIPOP_RUNTIME}.reactRefresh.enqueueUpdate();
    }} else {{
      import.meta.hot.invalidate();
    }}
  }});
}}
"
    )
    .unwrap();

    write!(
      new_code,
      "\
function $RefreshReg$(type, id) {{ return {ROLLIPOP_RUNTIME}.reactRefresh.register(type, {escaped_id} + ' ' + id); }}
function $RefreshSig$() {{ return {ROLLIPOP_RUNTIME}.reactRefresh.createSignatureFunctionForTransform(); }}
",
    )
    .unwrap();

    new_code
  }

  fn should_wrap(&self, id: &str) -> bool {
    let cleaned_id = clean_url(id);
    if id == RUNTIME_MODULE_KEY || cleaned_id == RUNTIME_MODULE_KEY {
      return false;
    }

    let exclude = (!self.exclude.is_empty()).then_some(self.exclude.as_slice());
    let include = (!self.include.is_empty()).then_some(self.include.as_slice());

    if filter(exclude, include, id, &self.cwd).inner() {
      return true;
    }

    if cleaned_id != id {
      return filter(exclude, include, cleaned_id, &self.cwd).inner();
    }

    false
  }
}

impl Plugin for RollipopReactRefreshWrapperPlugin {
  fn name(&self) -> Cow<'static, str> {
    Cow::Borrowed("builtin:rollipop-react-refresh-wrapper")
  }

  fn register_hook_usage(&self) -> HookUsage {
    HookUsage::Transform
  }

  async fn transform(
    &self,
    _ctx: SharedTransformPluginContext,
    args: &rolldown_plugin::HookTransformArgs<'_>,
  ) -> rolldown_plugin::HookTransformReturn {
    if !self.should_wrap(args.id) {
      return Ok(None);
    }

    let code = self.add_refresh_wrapper(args.code, args.id);
    Ok(Some(HookTransformOutput {
      code: Some(code),
      map: HookTransformOutputMap::Null,
      ..Default::default()
    }))
  }
}
