import js from "@eslint/js";
import prettier from "@vue/eslint-config-prettier";
import vue from "eslint-plugin-vue";
import ts from "typescript-eslint";

export default ts.config(
  { ignores: ["dist/**", "node_modules/**"] },
  js.configs.recommended,
  ...ts.configs.recommended,
  ...vue.configs["flat/recommended"],
  {
    files: ["**/*.vue"],
    languageOptions: {
      parserOptions: { parser: ts.parser },
    },
  },
  {
    rules: {
      // Component names here are scoped by BEM, and the views are routed by
      // path rather than referenced by name in a template.
      "vue/multi-word-component-names": "off",
    },
  },
  prettier,
);
