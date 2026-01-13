import { defineTest } from 'rolldown-tests';
import { replacePlugin } from '@rollipop/rolldown/plugins';

export default defineTest({
  config: {
    plugins: [
      replacePlugin({
        'process.env.NODE_ENV': JSON.stringify('production'),
      }),
    ],
  },
});
