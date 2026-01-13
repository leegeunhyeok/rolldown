import type { BuildOptions } from '@rollipop/rolldown';
import type { DevOptions } from '../types/dev-options';

export interface DevConfig {
  build?: BuildOptions;
  dev?: DevOptions;
}

export function defineDevConfig(config: DevConfig): DevConfig {
  return config;
}
