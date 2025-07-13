import parseLRC from "./utils/parseLrc";
import countLine from "./utils/countLine";
import { useState, useRef, useMemo, useEffect } from "react";
import React from "react";
import { invoke } from "@tauri-apps/api/core";

interface LyricsAreaProps {
    position: string;
    timeOffSet: React.MutableRefObject<number>;
}

// Clear the previous line's styles
// useMemo will not trigger re-render, so we need to clear the previous line's styles manually
function clearPrevLineStyle(prevLine: HTMLElement | null, prevLineIndex: number, index: number) {
    if (!prevLine) return;

    prevLine.classList.remove("font-bold");

    if (prevLineIndex < index) {
        prevLine.classList.replace("text-gray-900", "text-gray-600")
    } else if (prevLineIndex > index) {
        prevLine.classList.replace("text-gray-900", "text-white");
    }
}

function LyricsArea({ position, timeOffSet }: LyricsAreaProps) {
    const className = `overflow-y-auto h-full w-full ${position} font-mono text-2xl text-center py-10 font-medium mx-8`;

    const [notFound, setNotFound] = useState<boolean>(false);
    // check if the user is not playing any music
    const [notPlaying, setNotPlaying] = useState<boolean>(false);

    // the id of currently playing song of spotify
    const [id, setId] = useState<string>("");
    const [lyrics, setLyrics] = useState<string[]>([]);
    const [times, setTimes] = useState<number[]>([]);
    const [elapsedTime, setElapsedTime] = useState<number>(0);

    // the total duration of the song
    const durationRef = useRef<number>(0);
    const intervalRef = useRef<number | null>(null);

    // Reference to check if the music is playing
    // This is used to avoid unnecessary re-renders and calculations
    // Can be visible dynamically in a interval reference
    const isPlayingRef = useRef<boolean>(false);

    // Reference to the lyrics container and trace the previous line index in the container
    const lyricsContainer = useRef<HTMLDivElement>(null);
    const prevLineRef = useRef<number>(-1);

    // Reference to track the progress time of the song
    // lastUpdateTimeRef is to track the last time the progress time was updated
    const progressTimeRef = useRef<number>(0);
    const lastUpdateTimeRef = useRef<number>(0);

    const index = useMemo(() => countLine(elapsedTime, times), [elapsedTime]);

    const centerLyric = (index: number) => {
        index = Math.max(0, index);
        index = Math.min(lyrics.length - 1, index);

        if (lyricsContainer.current) {
            const container = lyricsContainer.current;
            const currentLine = container.children[index] as HTMLElement;
            const prevLine = container.children[prevLineRef.current] as HTMLElement;

            clearPrevLineStyle(prevLine, prevLineRef.current, index);

            if (currentLine) {
                currentLine.classList.replace("text-white", "text-gray-900");
                currentLine.classList.add("font-bold");
                const containerHeight = container.clientHeight;
                const lineHeight = currentLine.clientHeight;
                const scrollTop =
                    currentLine.offsetTop - containerHeight / 2 + lineHeight;
                container.scrollTo({ top: scrollTop, behavior: "smooth" });
            }

            prevLineRef.current = index;
        }
    };

    // Check the current playing song and the playing status every 0.5 seconds
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
        }, 500);

        return () => clearInterval(interval);
    }, []);

    // Update the progress time of the song every 3 seconds.
    // This is used to align the lyrics with the actual song progress.
    // This is necessary because sometime the song status maybe changed.
    useEffect(() => {
        const interval = setInterval(() => {
            invoke("get_progress_time").then((time) => {
                progressTimeRef.current = (time as number) + timeOffSet.current;
                lastUpdateTimeRef.current = Date.now();
            });
        }, 3000);

        return () => clearInterval(interval);
    }, []);

    // Fetch the progress time immediately when the time offset changes.
    useEffect(() => {
        invoke("get_progress_time").then((time) => {
            progressTimeRef.current = (time as number) + timeOffSet.current;
            lastUpdateTimeRef.current = Date.now();
        });
    }, [timeOffSet.current]);

    // Update the duration and the progress time when the song changes.
    useEffect(() => {
        invoke("get_duration")
            .then((duration) => (durationRef.current = duration as number))
            .catch((_) => console.log("Error getting duration"));

        invoke("get_progress_time").then((time) => {
            progressTimeRef.current = (time as number) + timeOffSet.current;
            lastUpdateTimeRef.current = Date.now();
        });

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
                            progressTimeRef.current +
                            (Date.now() - lastUpdateTimeRef.current)
                        );
                    }
                }, 50);
            })
            .catch((_) => setNotFound(true));

        return () => clearInterval(intervalRef.current as number);
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
