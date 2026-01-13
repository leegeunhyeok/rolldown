import { defineTest } from 'rolldown-tests';
import { viteImportGlobPlugin } from '@rollipop/rolldown/experimental';

export default defineTest({
  config: {
    plugins: [viteImportGlobPlugin({
      isV2: { sourcemap: false },
    })],
  },
  async afterTest() {
    await import('./assert.mjs');
  },
});
