# My Personal Website

There's not much to say.
It's written in Svelte and I try to update this regularly.

## Development

```bash
npm install
node-sass src/styles.scss > static/styles.css
npm run dev
```

can also use ``--watch`` for node sass

## Deployment

```bash
npm run build
```

Then copy the build folder to github pages (should write a script for this).
