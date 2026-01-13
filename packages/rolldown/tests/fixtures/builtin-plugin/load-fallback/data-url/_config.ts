import { defineTest } from 'rolldown-tests';
import { viteLoadFallbackPlugin } from '@rollipop/rolldown/experimental';

export default defineTest({
  config: {
    plugins: [viteLoadFallbackPlugin()],
  },
  async afterTest() {
    await import('./assert.mjs');
  },
});
