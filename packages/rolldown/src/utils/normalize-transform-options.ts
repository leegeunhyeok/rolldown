import type {
  BindingReactRefreshOptions,
  BindingStringOrRegex,
  BindingTransformOptions,
  TransformOptions as OxcTransformOptions,
} from '../binding.cjs';
import type { InputOptions } from '../options/input-options';
import { normalizedStringOrRegex } from './normalize-string-or-regex';

type RolldownOxcTransformOptions = Omit<
  OxcTransformOptions,
  'sourceType' | 'lang' | 'cwd' | 'sourcemap' | 'define' | 'inject' | 'jsx'
>;

type RolldownTransformOptions = {
  options: RolldownOxcTransformOptions;
  // MARK: - Rollipop
  jsx?: BindingTransformOptions['jsx'];
};

// MARK: - Rollipop
type RolldownBindingJsxOptions = Omit<
  Exclude<InputOptions['transform'], undefined> extends { jsx?: infer Jsx }
    ? Exclude<Jsx, false | string | undefined>
    : never,
  'refresh'
> & {
  refresh?: boolean | BindingReactRefreshOptions;
};

export interface NormalizedTransformOptions {
  define: Array<[string, string]> | undefined;
  inject: Record<string, string | [string, string]> | undefined;
  dropLabels: string[] | undefined;
  oxcTransformOptions: RolldownTransformOptions | undefined;
}

/**
 * Normalizes transform options by extracting `define`, `inject`, and `dropLabels` separately from OXC transform options.
 *
 * Prioritizes values from `transform.define`, `transform.inject`, and `transform.dropLabels` over deprecated top-level options.
 */
export function normalizeTransformOptions(inputOptions: InputOptions): NormalizedTransformOptions {
  const transform = inputOptions.transform;

  const define = transform?.define ? Object.entries(transform.define) : undefined;
  const inject = transform?.inject;
  const dropLabels = transform?.dropLabels;

  // Extract OXC transform options (excluding define, inject, and dropLabels)
  let oxcTransformOptions: RolldownTransformOptions | undefined;
  if (transform) {
    const { define: _define, inject: _inject, dropLabels: _dropLabels, jsx, ...rest } = transform;
    // MARK: - Rollipop
    const normalizedJsx = normalizeJsxOptions(jsx);
    // Only set oxcTransformOptions if there are actual options
    if (Object.keys(rest).length > 0 || normalizedJsx != null) {
      // MARK: - Rollipop
      oxcTransformOptions = {
        options: rest as RolldownOxcTransformOptions,
        ...(normalizedJsx != null ? { jsx: normalizedJsx } : {}),
      };
    }
  }

  return {
    define,
    inject,
    dropLabels,
    oxcTransformOptions,
  };
}

// MARK: - Rollipop
function normalizeJsxOptions(
  jsx: Exclude<InputOptions['transform'], undefined>['jsx'],
): RolldownTransformOptions['jsx'] | undefined {
  if (jsx === false) {
    return 'disable';
  }
  if (jsx == null || typeof jsx !== 'object') {
    return jsx;
  }

  const { refresh, ...bindingJsxOptions } = jsx;

  if (refresh == null || typeof refresh !== 'object') {
    const normalizedJsx = {
      ...bindingJsxOptions,
      ...(refresh !== undefined ? { refresh } : {}),
    } satisfies RolldownBindingJsxOptions;
    return normalizedJsx;
  }

  const { include, exclude, ...refreshOptions } = refresh as {
    include?: BindingStringOrRegex | BindingStringOrRegex[];
    exclude?: BindingStringOrRegex | BindingStringOrRegex[];
  };

  const normalizedJsx = {
    ...bindingJsxOptions,
    refresh: {
      ...refreshOptions,
      include: normalizedStringOrRegex<BindingStringOrRegex[]>(include),
      exclude: normalizedStringOrRegex<BindingStringOrRegex[]>(exclude),
    },
  } satisfies RolldownBindingJsxOptions;
  return normalizedJsx;
}
