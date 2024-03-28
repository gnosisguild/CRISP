import React, { Fragment, useEffect } from "react";
import init, { greet } from "../../rust/pkg/rfv";
import Navbar from "./components/Navbar";
import { Routes, Route } from "react-router-dom";
import Landing from "./pages/Landing/Landing";
import Footer from "./components/Footer";

const App: React.FC = () => {
  useEffect(() => {
    init().then(() => {
      greet("World");
    });
  }, []);

  return (
    <Fragment>
      <Navbar/>
      <Routes>
        <Route path="/" element={<Landing />} />
      </Routes>
      <Footer/>
    </Fragment>
  );
}

export default App;
