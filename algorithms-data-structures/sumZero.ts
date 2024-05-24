/*
Write a function called sumZero which accepts a sorted
array of integers. The function should find the first pair
where the sum is 0. Return an array that includes both
values that sum to zero or undefined if a pair does not exist
*/

export const sumZero = (arr: number[]) => {
  let i = 0
  let j = arr.length - 1

  while(i - j !== -1) {
    const iNum = arr[i]
    const jNum = arr[j]

    if(iNum + jNum === 0) return [iNum, jNum]
    else if(iNum + jNum > 0) j--
    else if(iNum + jNum < 0) i++
  }

  return undefined
}

console.log(sumZero([-3, -2, -1, 0, 1, 2, 3]))
console.log(sumZero([-2, 0, 1, 3]))
console.log(sumZero([1, 2, 3]))