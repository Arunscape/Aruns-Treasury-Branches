module.exports = {
  mode: "jit",
  //darkMode: "class",
  content: [
    "**/src/**/*.{html,rs}",
    "**/style.{css,scss}",
  ],
  theme: {
    extend: {},
  },
  //safelist: [
  //	{
  //		pattern: /(bg|text)-.+/
  //	},
  //],
  plugins: [
    require("daisyui"),
  ],
  daisyui: {
    themes: [
      "emerald",
      "forest",
    ],
    // suppresses ansi colours which makes cargo-leptos error:
    // https://github.com/leptos-rs/cargo-leptos/issues/136
    logs: false,
  },
};
