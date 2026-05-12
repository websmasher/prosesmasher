import tseslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";

export default [
  {
    ignores: ["dist/**", "node_modules/**"]
  },
  {
    files: ["src/**/*.ts"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: "./tsconfig.json",
        tsconfigRootDir: import.meta.dirname
      }
    },
    plugins: {
      "@typescript-eslint": tseslint
    },
    rules: {
      "no-restricted-imports": [
        "error",
        {
          paths: [
            {
              name: "escape-string-regexp",
              message: "Regex helpers are not allowed in prose rules."
            }
          ],
          patterns: [
            {
              group: ["*regex*", "*regexp*"],
              message: "Regex packages are not allowed in prose rules."
            }
          ]
        }
      ],
      "no-restricted-syntax": [
        "error",
        {
          selector: "Literal[regex]",
          message: "Do not use regex for prose rule matching. Use tokens, AST nodes, character scans, or token tries."
        },
        {
          selector: "NewExpression[callee.name='RegExp']",
          message: "Do not construct regex for prose rule matching. Use shared token matchers."
        },
        {
          selector: "CallExpression[callee.name='RegExp']",
          message: "Do not construct regex for prose rule matching. Use shared token matchers."
        }
      ]
    }
  }
];
