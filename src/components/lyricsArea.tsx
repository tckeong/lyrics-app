import parseLRC from "./utils/parseLrc";
import countLine from "./utils/countLine";
import { useState, useRef, useMemo, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface LyricsAreaProps {
    position: string;
    timeOffSet: React.MutableRefObject<number>;
}

function LyricsArea({ position, timeOffSet }: LyricsAreaProps) {
    const className = `overflow-y-auto h-full w-full ${position} font-mono text-2xl text-center py-10 font-medium mx-8`;
    const [notFound, setNotFound] = useState<boolean>(false);
    const [notPlaying, setNotPlaying] = useState<boolean>(false);
    const [id, setId] = useState<string>("");
    const [lyrics, setLyrics] = useState<string[]>([]);
    const [times, setTimes] = useState<number[]>([]);
    const durationRef = useRef<number>(0);
    const [elapsedTime, setElapsedTime] = useState<number>(0);
    const intervalRef = useRef<NodeJS.Timeout | null>(null);
    const lyricsContainer = useRef<HTMLDivElement>(null);
    const prevLineRef = useRef<number>(-1);
    const startTimeRef = useRef<number>(0);
    const isPlayingRef = useRef<boolean>(false);
    const index = useMemo(() => countLine(elapsedTime, times), [elapsedTime]);

    const centerLyric = (index: number) => {
        index = Math.max(0, index);
        index = Math.min(lyrics.length - 1, index);

        if (lyricsContainer.current) {
            const container = lyricsContainer.current;
            const prevLine = container.children[
                prevLineRef.current
            ] as HTMLElement;
            const currentLine = container.children[index] as HTMLElement;

            if (prevLine && prevLineRef.current < index) {
                prevLine.classList.remove("text-gray-900");
                prevLine.classList.remove("font-bold");
                prevLine.classList.remove("text-white");
                prevLine.classList.add("text-gray-600");
            } else if (prevLine && prevLineRef.current > index) {
                prevLine.classList.remove("text-gray-900");
                prevLine.classList.remove("font-bold");
                prevLine.classList.add("text-white");
            }

            if (currentLine) {
                currentLine.classList.remove("text-white");
                currentLine.classList.add("font-bold");
                currentLine.classList.add("text-gray-900");
                const containerHeight = container.clientHeight;
                const lineHeight = currentLine.clientHeight;
                const scrollTop =
                    currentLine.offsetTop - containerHeight / 2 + lineHeight;
                container.scrollTo({ top: scrollTop, behavior: "smooth" });
            }

            prevLineRef.current = index;
        }
    };

    useEffect(() => {
        const interval = setInterval(() => {
            invoke("get_id")
                .then((curId) => curId as string)
                .then((curId) => {
                    if (curId !== id) {
                        setId(curId);
                    }
                })
                .catch((_) => setNotPlaying(true));

            invoke("get_play_status").then((isPlaying) => {
                isPlayingRef.current = isPlaying as boolean;
            });
        }, 1000);

        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        const interval = setInterval(() => {
            invoke("get_time").then((time) => {
                startTimeRef.current =
                    Date.now() - (time as number) + timeOffSet.current;
            });
        }, 5000);

        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        invoke("get_time").then((time) => {
            startTimeRef.current =
                Date.now() - (time as number) + timeOffSet.current;
        });
    }, [timeOffSet.current, id]);

    useEffect(() => {
        invoke("get_duration")
            .then((duration) => (durationRef.current = duration as number))
            .catch((_) => console.log("Error getting duration"));

        invoke("get_lyrics")
            .then((lrc) => {
                const lrcContents = lrc as string;
                const parsedLyrics = parseLRC(lrcContents);
                const lyrics = parsedLyrics.map((lyricLine) => lyricLine.lyric);
                const times = parsedLyrics.map((lyricLine) => lyricLine.time);

                setLyrics(lyrics);
                setTimes(times);

                intervalRef.current = setInterval(() => {
                    if (isPlayingRef.current) {
                        setElapsedTime(
                            (Date.now() - startTimeRef.current) %
                                durationRef.current
                        );
                    }
                }, 10);
            })
            .catch((_) => setNotFound(true));

        return () => clearInterval(intervalRef.current as NodeJS.Timeout);
    }, [id]);

    useEffect(() => {
        centerLyric(index);
    }, [index]);

    return (
        <div ref={lyricsContainer} className={className}>
            {notPlaying && <p className="text-gray-600">No Music Playing!</p>}
            {(notFound || lyrics.length === 0) && (
                <p className="text-gray-600">
                    No Lyrics Found! Please start the neteaseapi server!
                </p>
            )}
            {lyrics.map((lyric, i) =>
                i < index ? (
                    <p key={i} className="my-3 cursor-default text-gray-600">
                        {lyric}
                    </p>
                ) : (
                    <p key={i} className="my-3 cursor-default text-white">
                        {lyric}
                    </p>
                )
            )}
            {Array.from({ length: 10 }).map((_, index) => (
                <p
                    key={lyrics.length + index}
                    className="opacity-0 cursor-default"
                >
                    Dummy Line
                </p>
            ))}
        </div>
    );
}

export default LyricsArea;
