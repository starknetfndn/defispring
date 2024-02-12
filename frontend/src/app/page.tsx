"use client";
import WalletBar, { zeroPadHex } from "@/components/WalletBar";
import contractAbi from "./abi.json";
import {
  reddioProvider,
  useAccount,
  useContract,
  useContractRead,
  useContractWrite,
} from "@starknet-react/core";
import { useEffect, useMemo, useState } from "react";
import { Button } from "@/components/ui/Button";

export default function Home() {
  const BASE_BACKEND_URL = "http://127.0.0.1:8080/";
  const CONTRACT_ADDRESS =
    "0x120412481b79cfd73eb1160dee54fc944db7dffbd46ff1129f7de27c472ab6a";

  const { contract } = useContract({
    abi: contractAbi,
    address: CONTRACT_ADDRESS,
  });
  const { address } = useAccount();

  // Data to be utilized for claiming tokens
  interface ClaimCalldata {
    // How much to claim. Should always claim the maximum amount
    amount: string;
    // Merkle proof for the claim
    proof: string[];
  }

  const [walletAddress, setWalletAddress] = useState<string>("");
  const [alreadyClaimed, setAlreadyClaimed] = useState<BigInt>(BigInt(0));
  const [airdropAmount, setAirdropAmount] = useState<BigInt>(BigInt(0));
  const [receivedcalldata, setReceivedCalldata] = useState<ClaimCalldata>();
  const [isClaimReady, setIsClaimReady] = useState<boolean>(false);
  const [errors, setErrors] = useState<String>("");

  useEffect(() => {
    if (address) {
      setWalletAddress(zeroPadHex(address!));
    }
  }, [address]);

  const {
    data: alreadyClaimedData,
    isError,
    isLoading,
    error,
  } = useContractRead({
    functionName: "amount_already_claimed",
    args: [walletAddress!],
    abi: contractAbi,
    address: CONTRACT_ADDRESS,
    watch: false,
  });

  useEffect(() => {
    if (!isLoading && !isError) {
      setAlreadyClaimed(alreadyClaimedData as BigInt);
    }
  }, [isLoading, isError, alreadyClaimedData]);

  const calls = useMemo(() => {
    if (!walletAddress || !contract || !receivedcalldata) return [];

    if (!receivedcalldata.amount) {
      setErrors(receivedcalldata.toString());
      return;
    }

    return contract.populateTransaction["claim"]!(
      receivedcalldata.amount,
      receivedcalldata.proof
    );
  }, [contract, receivedcalldata, walletAddress]);

  const { writeAsync, data, isPending } = useContractWrite({
    calls,
  });

  const claim = async () => {
    if (!walletAddress) {
      console.error("No wallet connected");
      return;
    }
    const response = await fetch(
      BASE_BACKEND_URL + "get_calldata?address=" + walletAddress
    );
    const calldata: ClaimCalldata = await response.json();

    setReceivedCalldata(calldata);

    setIsClaimReady(true);
  };

  const claim2 = async () => {
    if (!isClaimReady) {
      console.error("Prepare the claim first");
      return;
    }
    await writeAsync();
  };

  const getAirdropAmount = async () => {
    if (!walletAddress) {
      console.error("No wallet connected");
      return;
    }
    const response = await fetch(
      BASE_BACKEND_URL + "get_airdrop_amount?address=" + walletAddress
    );
    const amount = await response.json();
    console.log("got amount", amount);
    let num = BigInt(amount);

    setAirdropAmount(num);
  };

  return (
    <main className="flex flex-col items-center justify-center min-h-screen gap-12">
      <WalletBar />
      <div>
        <p>
          <b>Execute</b>
        </p>
        <div>
          <Button onClick={claim}>Prepare airdrop claim</Button>
        </div>
        <div style={{ padding: "5px" }}>
          <Button onClick={claim2}>Claim airdrop</Button>
        </div>
      </div>
      <div>
        <p>
          <b>Read functionality</b>
        </p>
        <div>
          <Button onClick={getAirdropAmount}>
            Get allocated airdrop amount
          </Button>
        </div>
      </div>
      <div>
        <p>
          <b>Results</b>
        </p>
        {alreadyClaimed !== undefined && (
          <div>Already claimed: {alreadyClaimed.toString()}</div>
        )}
        <div>
          <p>Total allocated airdrop amount: {airdropAmount.toString()}</p>
        </div>
        {errors && (
          <div>
            <p style={{ color: "red" }}>Errors: {errors}</p>
          </div>
        )}
      </div>
    </main>
  );
}
