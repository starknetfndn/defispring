"use client";
import { useAccount, useConnect, useDisconnect } from "@starknet-react/core";
import { useMemo } from "react";
import { Button } from "./ui/Button";

export function zeroPadHex(inputHex: string) {
  // Remove "0x" prefix if present
  let hexString = inputHex.startsWith("0x") ? inputHex.slice(2) : inputHex;

  // Ensure the hex string is 64 characters long by prepending zeros
  while (hexString.length < 64) {
    hexString = "0" + hexString;
  }

  // Add "0x" prefix back
  hexString = "0x" + hexString;

  return hexString;
}

function WalletConnected() {
  const { address } = useAccount();
  const { disconnect } = useDisconnect();

  const shortenedAddress = useMemo(() => {
    if (!address) return "";
    let useAddress = zeroPadHex(address);
    return `${useAddress.slice(0, 6)}...${useAddress.slice(-4)}`;
  }, [address]);

  return (
    <div>
      <span>Connected: {shortenedAddress}</span>
      <Button onClick={() => disconnect()}>Disconnect</Button>
    </div>
  );
}

function ConnectWallet() {
  const { connectors, connect } = useConnect();

  return (
    <div>
      <span>Choose a wallet: </span>
      {connectors.map((connector) => {
        return (
          <Button
            key={connector.id}
            onClick={() => connect({ connector })}
            className="gap-x-2 mr-2"
          >
            {connector.id}
          </Button>
        );
      })}
    </div>
  );
}

export default function WalletBar() {
  const { address } = useAccount();

  return address ? <WalletConnected /> : <ConnectWallet />;
}
