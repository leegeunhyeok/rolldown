import path from 'node:path';
import { defineParallelPlugin } from '@rollipop/rolldown/experimental';

export default defineParallelPlugin(path.resolve(import.meta.dirname, './impl.js'));
