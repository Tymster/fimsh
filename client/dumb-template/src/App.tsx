import React from "react";
import ReactDOM from "react-dom";
import React, { useState, useEffect } from "react";
import { createRoot } from "react-dom/client";
import {
  BrowserRouter,
  Routes,
  Route,
  Navigate,
  useSearchParams,
} from "react-router-dom";
import { Home, NotFound } from "./pages/index.ts"
const App = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="*" element={<NotFound />} />
      </Routes>

    </BrowserRouter>
  );
};

const domNode = document.getElementById('root');
if (domNode != null) {
  const root = createRoot(domNode);
  root.render(<App />)
}