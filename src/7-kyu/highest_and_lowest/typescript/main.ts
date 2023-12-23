export const Kata = { // eslint-disable-line @typescript-eslint/naming-convention
	highAndLow(numbers: string): string {
		const numbersArray = numbers.split(' ').map(Number);
		return `${Math.max(...numbersArray)} ${Math.min(...numbersArray)}`;
	},
};
