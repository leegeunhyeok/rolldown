use rolldown_sourcemap::{SourceJoiner, SourceMap};

use super::HMR_RUNTIME_NAME;

pub fn wrap_hmr_patch(
  mut source_joiner: SourceJoiner<'_>,
  id: &str,
) -> (String, Option<SourceMap>) {
  source_joiner.prepend_source(format!("(function ({HMR_RUNTIME_NAME}) {{"));

  let id = serde_json::to_string(id).unwrap();
  source_joiner
    .append_source(format!("}})(globalThis.__rollipop_runtime__.graphs.get({id}).runtime);"));

  source_joiner.join()
}

#[cfg(test)]
mod tests {
  use rolldown_sourcemap::{SourceMapBuilder, SourceMapSource};

  use super::wrap_hmr_patch;

  #[test]
  fn wraps_patch_and_offsets_sourcemap() {
    let source = "throw new Error('boom');";
    let mut builder = SourceMapBuilder::default();
    let source_id = builder.add_source_and_content("source.js", source);
    builder.add_token(0, 0, 0, 0, Some(source_id), None);

    let mut source_joiner = rolldown_sourcemap::SourceJoiner::default();
    source_joiner.append_source(SourceMapSource::new(
      source.to_string(),
      builder.into_sourcemap().into_owned(),
    ));

    let (code, map) = wrap_hmr_patch(source_joiner, "host\"\n");

    assert_eq!(
      code,
      "(function (__rolldown_runtime__) {\nthrow new Error('boom');\n})(globalThis.__rollipop_runtime__.graphs.get(\"host\\\"\\n\").runtime);"
    );

    let map = map.unwrap();
    let token = map.get_tokens().next().unwrap();
    assert_eq!(token.get_dst_line(), 1);
    assert_eq!(token.get_dst_col(), 0);
    assert_eq!(token.get_src_line(), 0);
    assert_eq!(token.get_src_col(), 0);
  }
}
