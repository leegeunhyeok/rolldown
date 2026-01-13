import { defineConfig } from '@rollipop/rolldown';

export default defineConfig({
  input: './index.js',
  plugins: [
    {
      name: 'test',
      closeBundle() {
        console.log('[test:closeBundle]');
      },
    },
  ],
});
