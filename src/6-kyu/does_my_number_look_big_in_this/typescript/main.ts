export function narcissistic(value: number, n = value,
	numberDigits = Math.floor(Math.log10(value)) + 1, sum = 0): boolean {
	return n === 0 ? sum === value : narcissistic(value, Math.floor(n / 10),
		numberDigits, sum + ((n % 10) ** numberDigits));
}
