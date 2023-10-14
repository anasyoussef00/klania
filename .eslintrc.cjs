module.exports = {
  root: true,
  extends: [
    // add more generic rulesets here, such as:
    'eslint:recommended',
    'plugin:vue/vue3-strongly-recommended',
    'prettier',
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    ecmaVersion: 'latest',
    sourceType: 'module',
  },
  rules: {
    // override/add rules settings here, such as:
    // 'vue/no-unused-vars': 'error'
  },
};
