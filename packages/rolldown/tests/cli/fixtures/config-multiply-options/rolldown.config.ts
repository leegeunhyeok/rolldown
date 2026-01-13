import { defineConfig } from '@rollipop/rolldown';

export default defineConfig([
  {
    input: 'index.js',
    output: {
      format: 'esm',
      entryFileNames: 'esm.js',
    },
  },
  {
    input: 'index.js',
    output: {
      format: 'cjs',
      entryFileNames: 'cjs.js',
    },
  },
]);
