import config from "@tree-company/eslint-config";
export default [
  ...config.configs.react,
  {
    ignores: ["dist/", "frontend/dist", "main/target"],
  },
];
