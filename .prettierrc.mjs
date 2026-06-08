/**
 * Prettier 代码格式化配置
 * 详见：https://prettier.io/docs/en/options.html
 */
export default {
    printWidth: 120,
    tabWidth: 4,
    useTabs: true,
    semi: false,
    singleQuote: true,
    jsxSingleQuote: true,
    trailingComma: 'none',
    bracketSpacing: true,
    arrowParens: 'avoid',
    endOfLine: 'lf',
    htmlWhitespaceSensitivity: 'css',
    vueIndentScriptAndStyle: false,
    embeddedLanguageFormatting: 'auto',
    proseWrap: 'preserve',
    quoteProps: 'as-needed',
    overrides: [
        {
            files: ['*.json', '*.jsonc'],
            options: {
                tabWidth: 2,
                useTabs: true
            }
        }
    ]
}
