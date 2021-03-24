module.exports = {
  root: true,
  env: {
    browser: true,
    es6: true
  },
  extends: [
    'eslint:recommended',
    'plugin:vue/recommended',
    '@vue/standard'
  ],
  parserOptions: {
    parser: 'babel-eslint'
  },
  rules: {
    'no-console': 'warn',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'error' : 'warn',
    'no-extra-parens': 'warn',
    'no-unreachable-loop': 'error',
    'no-useless-backreference': 'error',
    'block-scoped-var': 'error',
    curly: [ 'error', 'multi-or-nest' ],
    'default-case': 'warn',
    'default-case-last': 'error',
    'default-param-last': 'error',
    'dot-location': [ 'error', 'property' ],
    'dot-notation': [ 'error', { allowKeywords: true } ],
    eqeqeq: [ 'error', 'smart' ],
    'grouped-accessor-pairs': [ 'error', 'getBeforeSet' ],
    'no-alert': 'error',
    'no-caller': 'error',
    'no-constructor-return': 'error',
    'no-div-regex': 'error',
    'no-else-return': 'error',
    'no-empty-function': 'warn',
    'no-eval': 'error',
    'no-extend-native': 'error',
    'no-extra-bind': 'error',
    'no-extra-label': 'error',
    'no-implicit-globals': 'error',
    'no-implied-eval': 'error',
    'no-invalid-this': 'error',
    'no-labels': 'error',
    'no-lone-blocks': 'error',
    'no-return-await': 'error',
    'no-script-url': 'error',
    'no-self-compare': 'error',
    'no-sequences': 'error',
    'no-throw-literal': 'error',
    'no-unused-expressions': 'error',
    'no-useless-call': 'error',
    'no-useless-concat': 'warn',
    'no-useless-return': 'error',
    'require-await': 'error',
    'wrap-iife': 'error',
    strict: [ 'error', 'global' ],
    'no-label-var': 'error',
    'no-shadow': 'error',
    'no-undefined': 'error',
    'no-use-before-define': 'error',
    'array-bracket-newline': [ 'error', { multiline: true } ],
    'array-bracket-spacing': [ 'error', 'always', { objectsInArrays: true } ],
    'block-spacing': [ 'error', 'always' ],
    'brace-style': [ 'error', '1tbs' ],
    'eol-last': 'error',
    'func-call-spacing': [ 'error', 'never' ],
    'function-paren-newline': [ 'error', 'never' ],
    indent: [ 'error', 2 ],
    'jsx-quotes': [ 'error', 'prefer-double' ],
    'key-spacing': 'error',
    'keyword-spacing': [ 'error', { overrides: { if: { after: false }, for: { after: false }, while: { after: false } } } ],
    'linebreak-style': [ 'error', 'unix' ],
    'no-lonely-if': 'error',
    'no-trailing-spaces': 'error',
    'no-underscore-dangle': 'error',
    'no-unneeded-ternary': 'error',
    'no-whitespace-before-property': 'error',
    'nonblock-statement-body-position': [ 'error', 'below' ],
    'prefer-exponentiation-operator': 'error',
    'quote-props': [ 'error', 'as-needed' ],
    quotes: [ 'error', 'single', { avoidEscape: true, allowTemplateLiterals: true } ],
    semi: [ 'error', 'always' ],
    'semi-spacing': [ 'error', { before: false, after: true } ],
    'semi-style': [ 'error', 'last' ],
    'space-before-blocks': [ 'error', 'always' ],
    'space-before-function-paren': [ 'error', { anonymous: 'never', named: 'never', asyncArrow: 'always' } ],
    'space-infix-ops': [ 'error', { int32Hint: true } ],
    'spaced-comment': [ 'error', 'always', { exceptions: [ '-', '*' ] } ],
    'switch-colon-spacing': [ 'error', { after: true, before: false } ],
    'template-tag-spacing': [ 'error', 'never' ],
    'arrow-body-style': [ 'error', 'as-needed' ],
    'arrow-parens': [ 'error', 'as-needed' ],
    'arrow-spacing': [ 'error', { before: true, after: true } ],
    'generator-star-spacing': [ 'error', 'before' ],
    'no-confusing-arrow': [ 'error', { allowParens: true } ],
    'no-duplicate-imports': 'error',
    'no-useless-computed-key': 'error',
    'no-useless-constructor': 'warn',
    'no-useless-rename': 'error',
    'no-var': 'error',
    'object-shorthand': [ 'error', 'always' ],
    'prefer-arrow-callback': [ 'error', { allowUnboundThis: true } ],
    'prefer-const': 'error',
    'prefer-numeric-literals': 'error',
    'prefer-rest-params': 'error',
    'prefer-spread': 'error',
    'prefer-template': 'error',
    'rest-spread-spacing': [ 'error', 'never' ],
    'sort-imports': [ 'error', { allowSeparatedGroups: true, ignoreCase: true } ],
    'symbol-description': 'error',
    'template-curly-spacing': [ 'error', 'never' ],
    'yield-star-spacing': [ 'error', 'after' ]
  }
};