import "./App.css";
import { Routes, Route, BrowserRouter } from "react-router-dom";
import Index from "./pages";
import Login from "./pages/login";
import LyricsList from "./pages/lyric-list";
import Lyric from "./pages/lyric";

function App() {
  return (
    <>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Index />} />
        <Route path="/login" element={<Login />} />
        <Route path="/lyrics-ls" element={<LyricsList />} />
        <Route path="/lyric" element={<Lyric />} />
      </Routes>
    </BrowserRouter>  
    </>
  )
}

export default App;
