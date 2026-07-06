// MARK: - Rollipop

use napi::Either;
use napi_derive::napi;
use oxc_transform_napi::TransformOptions as OxcTransformOptions;
use rolldown_common::{
  JsxOptions, ReactCompilerDynamicGating, ReactCompilerGating, ReactCompilerOptions,
  ReactRefreshOptions,
};

use crate::types::binding_string_or_regex::{
  BindingStringOrRegex, bindingify_string_or_regex_array,
};

// Oxc's transformer has React Compiler support, but `oxc_transform_napi` does not expose a NAPI option object for it yet.
// Keep this Oxc-shaped mirror isolated so it can be replaced by the upstream binding type when that lands.

#[napi(object, object_to_js = false)]
#[derive(Default)]
pub struct BindingTransformOptions {
  pub options: OxcTransformOptions,
  // MARK: - Rollipop
  #[napi(ts_type = "'preserve' | 'react' | 'react-jsx' | 'disable' | BindingJsxOptions")]
  pub jsx: Option<Either<String, BindingJsxOptions>>,
}

// MARK: - Rollipop
/// Configure how TSX and JSX are transformed.
///
/// Mirrors Oxc's JSX options with Rollipop-only React Compiler and refresh filters.
#[napi(object, object_to_js = false)]
#[derive(Default)]
pub struct BindingJsxOptions {
  /// Decides which runtime to use.
  #[napi(ts_type = "'classic' | 'automatic'")]
  pub runtime: Option<String>,
  /// Emit development-specific information, such as `__source` and `__self`.
  pub development: Option<bool>,
  /// Toggles whether or not to throw an error if an XML namespaced tag name is used.
  pub throw_if_namespace: Option<bool>,
  /// Mark JSX elements and top-level React method calls as pure for tree shaking.
  pub pure: Option<bool>,
  /// Replaces the import source when importing functions.
  pub import_source: Option<String>,
  /// Replace the function used when compiling JSX expressions.
  pub pragma: Option<String>,
  /// Replace the component used when compiling JSX fragments.
  pub pragma_frag: Option<String>,
  /// Enable React Fast Refresh.
  pub refresh: Option<Either<bool, BindingReactRefreshOptions>>,
  /// Enable React Compiler.
  pub compiler: Option<OxcReactCompilerOptions>,
}

// MARK: - Rollipop
/// React Fast Refresh options.
#[napi(object, object_to_js = false)]
#[derive(Default)]
pub struct BindingReactRefreshOptions {
  /// File patterns to transform. Empty means all files that enter the transform pipeline.
  pub include: Option<Vec<BindingStringOrRegex>>,
  /// File patterns to skip.
  pub exclude: Option<Vec<BindingStringOrRegex>>,
  /// Specify the identifier of the refresh registration variable.
  pub refresh_reg: Option<String>,
  /// Specify the identifier of the refresh signature variable.
  pub refresh_sig: Option<String>,
  pub emit_full_signatures: Option<bool>,
}

/// Options for the experimental React Compiler transform.
///
/// @category Utilities
#[napi(object, object_to_js = false)]
#[derive(Default)]
pub struct OxcReactCompilerOptions {
  /// File patterns to compile. Empty means all files that enter the transform pipeline.
  pub include: Option<Vec<BindingStringOrRegex>>,
  /// File patterns to skip.
  pub exclude: Option<Vec<BindingStringOrRegex>>,
  /// Which functions to compile.
  ///
  /// @default 'infer'
  #[napi(ts_type = "'infer' | 'syntax' | 'annotation' | 'all'")]
  pub compilation_mode: Option<String>,
  /// What to do when a function cannot be compiled.
  ///
  /// @default 'none'
  #[napi(ts_type = "'none' | 'critical_errors' | 'all_errors'")]
  pub panic_threshold: Option<String>,
  /// React runtime version target.
  ///
  /// @default '19'
  #[napi(ts_type = "'17' | '18' | '19'")]
  pub target: Option<String>,
  /// Analyze and report diagnostics only; emit no transformed code.
  ///
  /// @default false
  pub no_emit: Option<bool>,
  /// Compiler output mode.
  ///
  /// @default undefined
  #[napi(ts_type = "'client' | 'ssr' | 'lint'")]
  pub output_mode: Option<String>,
  /// Compile even functions marked with opt-out directives.
  ///
  /// @default false
  pub ignore_use_no_forget: Option<bool>,
  /// Treat Flow suppression comments as opt-outs.
  ///
  /// @default true
  pub flow_suppressions: Option<bool>,
  /// Enable `react-native-reanimated` support.
  ///
  /// @default false
  pub enable_reanimated: Option<bool>,
  /// Development mode.
  ///
  /// @default false
  pub is_dev: Option<bool>,
  /// Source file name, used for the fast-refresh hash and in diagnostics.
  pub filename: Option<String>,
  /// ESLint rules whose suppressions opt a function out of compilation.
  pub eslint_suppression_rules: Option<Vec<String>>,
  /// Extra directives that opt a function out of compilation.
  pub custom_opt_out_directives: Option<Vec<String>>,
  /// Also emit a gated version of each compiled function.
  pub gating: Option<OxcReactCompilerGating>,
  /// Dynamically-gated compilation.
  pub dynamic_gating: Option<OxcReactCompilerDynamicGating>,
}

#[napi(object)]
pub struct OxcReactCompilerGating {
  /// Module the gating import comes from.
  pub source: String,
  /// Imported specifier used as the gate.
  pub import_specifier_name: String,
}

#[napi(object)]
pub struct OxcReactCompilerDynamicGating {
  /// Module the gating import comes from.
  pub source: String,
}

impl From<OxcReactCompilerOptions> for ReactCompilerOptions {
  fn from(value: OxcReactCompilerOptions) -> Self {
    Self {
      include: value.include.map(bindingify_string_or_regex_array).unwrap_or_default(),
      exclude: value.exclude.map(bindingify_string_or_regex_array).unwrap_or_default(),
      compilation_mode: value.compilation_mode,
      panic_threshold: value.panic_threshold,
      target: value.target,
      no_emit: value.no_emit,
      output_mode: value.output_mode,
      ignore_use_no_forget: value.ignore_use_no_forget,
      flow_suppressions: value.flow_suppressions,
      enable_reanimated: value.enable_reanimated,
      is_dev: value.is_dev,
      filename: value.filename,
      eslint_suppression_rules: value.eslint_suppression_rules,
      custom_opt_out_directives: value.custom_opt_out_directives,
      gating: value.gating.map(|gating| ReactCompilerGating {
        source: gating.source,
        import_specifier_name: gating.import_specifier_name,
      }),
      dynamic_gating: value
        .dynamic_gating
        .map(|dynamic_gating| ReactCompilerDynamicGating { source: dynamic_gating.source }),
    }
  }
}

impl From<BindingReactRefreshOptions> for ReactRefreshOptions {
  fn from(value: BindingReactRefreshOptions) -> Self {
    Self {
      include: value.include.map(bindingify_string_or_regex_array).unwrap_or_default(),
      exclude: value.exclude.map(bindingify_string_or_regex_array).unwrap_or_default(),
      refresh_reg: value.refresh_reg,
      refresh_sig: value.refresh_sig,
      emit_full_signatures: value.emit_full_signatures,
    }
  }
}

impl From<BindingJsxOptions> for JsxOptions {
  fn from(value: BindingJsxOptions) -> Self {
    Self {
      runtime: value.runtime,
      development: value.development,
      throw_if_namespace: value.throw_if_namespace,
      pure: value.pure,
      import_source: value.import_source,
      pragma: value.pragma,
      pragma_frag: value.pragma_frag,
      refresh: value.refresh.map(|refresh| match refresh {
        Either::A(enabled) => itertools::Either::Left(enabled),
        Either::B(options) => itertools::Either::Right(options.into()),
      }),
      compiler: value.compiler.map(Into::into),
    }
  }
}
