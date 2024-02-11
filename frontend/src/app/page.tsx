"use client";
import WalletBar from "@/components/WalletBar";
import contractAbi from "./abi.json";
import {
  useAccount,
  useContract,
  useContractRead,
  useContractWrite,
} from "@starknet-react/core";
import { useEffect, useMemo, useState } from "react";
import { Button } from "@/components/ui/Button";

export default function Home() {
  const BASE_BACKEND_URL = "http://127.0.0.1:8080/";
  const contractAddress =
    "0x07fc96c0049151d938d0d799228716f60bf98c578fbf9cdb1eb7e81f7440a850";

  const { contract } = useContract({
    abi: contractAbi,
    address: contractAddress,
  });
  const { address } = useAccount();

  interface ClaimCalldata {
    address: string;
    amount: string;
    proof: string[];
  }

  const [walletAddress, setWalletAddress] = useState<string>("");
  const [alreadyClaimed, setAlreadyClaimed] = useState<BigInt>(BigInt(0));
  const [airdropAmount, setAirdropAmount] = useState<string>("");
  const [receivedcalldata, setReceivedCalldata] = useState<ClaimCalldata>();
  const [isClaimReady, setIsClaimReady] = useState<boolean>(false);

  useEffect(() => {
    if (address) {
      setWalletAddress(address!);
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
    address: contractAddress,
    watch: false,
  });

  useEffect(() => {
    if (!isLoading && !isError) {
      console.log("DATA", alreadyClaimedData as BigInt);
      setAlreadyClaimed(alreadyClaimedData as BigInt);
    }
  }, [isLoading, isError, alreadyClaimedData]);

  const calls = useMemo(() => {
    if (!walletAddress || !contract || !receivedcalldata) return [];
    console.log("settin memo");
    return contract.populateTransaction["claim"]!(
      receivedcalldata.address,
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
    console.log("Received claldata", calldata);

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

    setAirdropAmount(amount);
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
        <div>Already claimed: {alreadyClaimed.toString()}</div>
        <div>
          <p>Allocated airdrop amount: {airdropAmount}</p>
        </div>
      </div>
    </main>
  );
}
