// Write a function called same, which accepts two arrays
// The function should return true if every value in the
// array has it's corresponding value squared in the second
// array. The frequency of values must be the same.

export const same = (arr1: number[], arr2: number[]): boolean => {
  const frequency1 = toFrequency(arr1);
  const frequency2Squared = toFrequency(arr2.map((num) => Math.sqrt(num)));

  //compare lengths
  if (Object.keys(frequency1).length !== Object.keys(frequency2Squared).length)
    return false;

  //compare each frequency
  const result = Object.entries(frequency2Squared).every(
    ([num, frequency]) => frequency1[num] === frequency
  );

  return result;

  //Time complexity: O(n)
};

const toFrequency = (arr: number[]) =>
  arr.reduce((frequency, num) => {
    if (frequency[num] === undefined) frequency[num] = 0; //initialization
    frequency[num]++;
    return frequency;
  }, {} as Record<number, number>);

console.log(same([1, 2, 3], [4, 1, 9]));
console.log(same([1, 2, 3], [1, 9]));
console.log(same([1, 2, 1], [4, 4, 1]));
