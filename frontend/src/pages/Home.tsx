// other project would import from NPM package
import DeFiSpringAllocation from "../components/DeFiSpringAllocation";

function ExampleProject() {
  const BASE_BACKEND_URL = "http://35.195.237.203:8080/";
  const CONTRACT_ADDRESS =
    "0x03e942530ef96da8e65e453f0fbbb198994515c69edd1dcf3be353b0956fbd1a";

  // this component uses inline styles to show that no styling is imposed on the users
  return (
    <main style={{ minHeight: "100vh" }}>
      <div
        style={{
          minHeight: "15vh",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <h1>Example Project</h1>
      </div>
      <div style={{ display: "flex", minHeight: "85vh" }}>
        <DeFiSpringAllocation
          protocolAddress={CONTRACT_ADDRESS}
          backendUrl={BASE_BACKEND_URL}
        />
      </div>
    </main>
  );
}

export default ExampleProject;
