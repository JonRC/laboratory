/**
 * countUniqueValues
 * Implement a function called countUniqueValues,
 * which accepts a sorted array, and counts the
 * unique values in the array. There can be negative
 * numbers in the array, but it will always be sorted.
 */

const countUniqueValues = (arr: number[]) => {
  let lastNumber: number | undefined = undefined;
  let uniqueValues = 0;

  arr.forEach((num) => {
    if (num !== lastNumber) {
      uniqueValues++;
      lastNumber = num;
    }
  });

  return uniqueValues;
};

console.log(countUniqueValues([1, 1, 1, 1, 1, 1, 1, 1, 2]));
console.log(countUniqueValues([1, 2, 3, 4, 4, 4, 7, 7, 12, 12, 13]));
console.log(countUniqueValues([]));
console.log(countUniqueValues([-2, -1, -1, 0, 1]));
