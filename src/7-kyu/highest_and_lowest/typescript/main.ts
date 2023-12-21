export const Kata = {
	highAndLow(numbers: string): string {
		const numbersArray = numbers.split(' ').map(Number);
		return `${Math.max(...numbersArray)} ${Math.min(...numbersArray)}`;
	},
};
