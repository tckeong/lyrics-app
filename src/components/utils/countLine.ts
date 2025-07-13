/**
 * Using binary search to find the index of timeNow within the given times array.
 * @param timeNow The time to count.
 * @param times The array of times.
 * @returns The index of the timeNow in the times array.
 */
function countLine(timeNow: number, times: number[]): number {
    let left = 0;
    let right = times.length - 1;
    let index = 0;

    while (left <= right) {
        const mid = Math.floor((left + right) / 2);

        if (times[mid] >= timeNow) {
            index = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    return Math.max(0, index - 1);
}

export default countLine;
