import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import {mdsvex} from 'mdsvex';

import rehypeKatexSvelte from "rehype-katex-svelte";
import remarkMath from 'remark-math'

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte', '.md', '.svx'],
    preprocess: [
		vitePreprocess(),
        mdsvex({
            extensions: ['.md', '.svx'],
			layout: "./src/blog.svelte",
			rehypePlugins: [rehypeKatexSvelte],
			remarkPlugins: [remarkMath]
        })
    ],

	kit: {
		adapter: adapter()
	}
};

export default config;
