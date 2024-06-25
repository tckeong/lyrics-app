export interface LyricLine {
    time: number;
    lyric: string;
}

function parseLRC(lrcContent: string) {
    const lines = lrcContent.split('\n');
    const lyrics: LyricLine[] = [];

    const timeExp = /\[(\d{2}):(\d{2})(\.\d{2,3})?\]/;

    lines.forEach(line => {
        const timeMatch = line.match(timeExp);
        if (timeMatch) {
            const minutes = parseInt(timeMatch[1], 10);
            const seconds = parseInt(timeMatch[2], 10);
            const milliseconds = parseInt(timeMatch[3].slice(1), 10);
            const time = minutes * 60000 + seconds * 1000 + milliseconds;

            const lyric = line.replace(timeExp, '').trim();
            if (lyric) {
                lyrics.push({ time, lyric });
            }
        }
    });

    return lyrics;
}

export default parseLRC;