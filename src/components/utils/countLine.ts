function countLine(timeNow: number, times: number[]): number {
  timeNow -= 250;

  for (let i = 0; i < times.length; i++) {
    if (i + 1 < times.length && timeNow >= times[i] && timeNow <= times[i + 1]) {
      return i;
    } else if (i + 1 === times.length && timeNow >= times[i]) {
      return i;
    }
  }

  // unreachable
  return -1;
}

export default countLine;