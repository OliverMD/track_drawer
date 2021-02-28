# Track Drawer

A fun, creative, tool for generating art from lines and dots within a grid, using randomisation.

## Building
1. Install dependencies:
    * [trunk](https://github.com/thedodd/trunk)
2. If any changes have been made to the `tailwind.config.js` then you'll need to generate the `public/tailwind.css` file again. This requires the following npm packages: `tailwindcss`, `@tailwindcss/custom-forms`. Then the follwoing can be run to generate the css:
```text
npx tailwindcss-cli@latest build -o ./public/tailwind.css -c tailwind.config.js
```
3. Run the desired trunk command, eg. `trunk serve` to serve it locally.
