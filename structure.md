.
├── index.html
├── LICENSE.txt
├── package-lock.json
├── package.json
├── postcss.config.js
├── README.md
├── src/
│   ├── App.css
│   ├── App.tsx
│   ├── components/
│   │   ├── addMinusButton.tsx
│   │   ├── authButton.tsx
│   │   ├── dropDown.tsx
│   │   ├── homeButton.tsx
│   │   ├── lyricsArea.tsx
│   │   ├── saveButton.tsx
│   │   ├── styles/
│   │   │   └── dropDown.js
│   │   ├── userButton.tsx
│   │   └── utils/
│   │       ├── countLine.ts
│   │       └── parseLrc.ts
│   ├── main.tsx
│   ├── pages/
│   │   ├── index.tsx
│   │   ├── login.tsx
│   │   ├── lyric-list.tsx
│   │   ├── lyrics.tsx
│   │   └── styles/
│   │       ├── index.js
│   │       ├── login.js
│   │       ├── lyric.js
│   │       └── lyrics-list.js
│   └── vite-env.d.ts
├── src-tauri/
│   ├── build.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── data/
│   │   ├── client/
│   │   │   └── auth.json
│   │   ├── lyrics/
│   │   └── lyrics.json
│   ├── env.json
│   ├── src/
│   │   ├── api/
│   │   │   ├── lyrics_api.rs
│   │   │   ├── mod.rs
│   │   │   ├── server.rs
│   │   │   └── spotify_api/
│   │   │       ├── mod.rs
│   │   │       └── spotify_credentials.rs
│   │   ├── controllers/
│   │   │   ├── authentication.rs
│   │   │   ├── mod.rs
│   │   │   └── song_details.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── netease_lyrics_model.rs
│   │   │   ├── netease_songs_model.rs
│   │   │   ├── spotify_credentials_model.rs
│   │   │   ├── spotify_track_model.rs
│   │   │   └── spotify_user_model.rs
│   │   └── utils/
│   │       └── mod.rs
│   └── tauri.conf.json
├── tailwind.config.js
├── tsconfig.json
├── tsconfig.node.json
└── vite.config.ts
