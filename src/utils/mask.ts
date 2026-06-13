/**
 * 脱敏显示敏感字符串。
 * "sk-abc123xyz" → "sk-a...xyz"
 * 短字符串（≤8 字符）全部用星号替代。
 */
export function maskToken(value: string): string {
	if (!value) return ''
	if (value.length <= 8) return '*'.repeat(value.length)
	const prefix = value.slice(0, 4)
	const suffix = value.slice(-3)
	return `${prefix}...${suffix}`
}
