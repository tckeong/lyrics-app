import "./App.css";
import Page from "./components/Page";
import { Routes, Route, BrowserRouter } from "react-router-dom";

function App() {
  return (
    <>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Page name="hello"/>} />
        <Route path="/page" element={<Page name="page"/>} />
      </Routes>
    </BrowserRouter>  
    </>
  )
}

export default App;
