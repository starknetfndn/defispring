import "./globals.css";
import { StarknetProvider } from "./components/StarknetProvider";
import Home from "./pages/Home";

function App() {
  return (
    <StarknetProvider>
      <Home />
    </StarknetProvider>
  );
}

export default App;
