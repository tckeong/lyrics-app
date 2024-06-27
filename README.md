# Lyrics App

## Overview

This Lyrics App (Spotify) is a desktop application that allows users to view lyrics for the currently playing Spotify track of their own spotify account. The app is built using Tauri, React, TailwindCSS and Rust, and it leverages the Spotify API to fetch track details and use [NeteaseCloudMusicApi](https://gitlab.com/Binaryify/neteasecloudmusicapi) to fetch the lyrics.

## Features

- **Spotify Authentication**: Users can log in with their Spotify account.
- **Lyrics Display**: Fetches and displays lyrics for the currently playing track.
- **Persistent Storage**: Stores (Caching) lyrics locally for faster access.

## Prerequisites

- [Node.js](https://nodejs.org/en/) (v20.x or later)
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Tauri CLI](https://tauri.app) (v1.0 or later)
- [Spotify Developer Account](https://developer.spotify.com/)

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/tckeong/lyrics-app.git
cd lyrics-app
```

### 2. Install Dependencies

\
Frontend (React)

```bash
npm install
```

\
Backend (Rust) \
_Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/)_

### 3. Set Up Spotify API Credentials

- Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
- Create a new application and note down the **Client ID** and **Client Secret**.
- Set the Redirect URI to `http://localhost:8081/callback`

### 4. Set Up the NetEaseCloudMusicAPI

- Use the method provided by the [NeteaseCloudMusicApi]("https://gitlab.com/Binaryify/neteasecloudmusicapi") repository to setup the NetEaseCloudMusicAPI server.
- In this app, we use docker as the lyrics server (using port 3000).

### 5. Build and run the Application

- Development Version

```bash
npm run tauri dev
```

- Release Version

```bash
npm run tauri build
```

> Get the installer from src-tauri/target/release/bundle

## Usage

1. Login: Open the app and log in with your Spotify credentials (Need the Client ID and Client Secret from Spotify Developer Dashboard).
2. View Lyrics: Play a song on Spotify. The app will fetch and display the lyrics automatically.
3. Save Lyrics: The user can choose to save the lyric or not (for caching convenience). The user can check the lyrics list that saved by them.

## Project Structure

```
| spotify-lyrics-app/
├── src-tauri/            # tauri src
│ ├── src/
│ │ ├── main.rs           # Main Rust entry point
│ │ ├── api/              # Handles all api
| | | ├── spotify_api/    # Handles all spotify api, like get spotify credentials, get current playing track and so on
| | | ├── server.rs       # Handles all api relevant to callback and get the spotify api token
| | | └── mod.rs          # Abstract the api, make them easier to use by the controllers
| | ├── controllers/      # Handles all tauri frontend app function invoke
| | ├── models/           # Handles all the web response data models
| | └── utils/            # Handles all the services that relevant to create local folder and local file
│ ├── tauri.conf.json     # Tauri configuration
│ └── Cargo.toml          # Rust dependencies
├── src/
│ ├── components/         # React components, stores all the components that used in the pages
| | └── utils/
| ├── pages/              # stores all the pages
│ └── App.tsx             # Main React component, handles the pages route
├── package.json          # Node.js dependencies
├── tailwind.config.js
├── vite.config.ts
└── README.md             # This file
```

## Troubleshooting

### Common Issues

- **Authentication Errors**: Ensure your Spotify API credentials are correct and the Redirect URI is properly set.
- **Missing Lyrics**: The app relies on external APIs for lyrics. Ensure the APIs are accessible.
- **Lyrics Mismatching**: Sometimes the lyrics may mismatching, this may occurs due to the NetEaseCloudMusicAPI searching problem, already make some advancement about this such as: translate the name of chinese singer, and remove some irrelevant things in song name, and handle some special case that the singer name cannot be translate properly.

## Contribution

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## Advancement

Feel free to create issues if you meet any problem when using this app, plan to do advancement in following things:

- add the lyrics edit page, such that the user can edit the lyrics manually.
- add the link in the lyrics list page, when user press on the link, it will make spotify jump to the song that user choose.

### LICENSE

This project is licensed under the MIT License. See the LICENSE file for details.

### Acknowledgements

- [Tauri](https://tauri.app)
- [React](https://react.dev/)
- [Vite](https://vitejs.dev/)
- [TailwindCSS](https://tailwindcss.com/)
- [Spotify API](https://developer.spotify.com/documentation/web-api)
- [NetEaseCloudMusicAPI](https://gitlab.com/Binaryify/neteasecloudmusicapi)
